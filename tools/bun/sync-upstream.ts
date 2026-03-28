/**
 * Upstream sync orchestrator for the swc-bun fork.
 *
 * Fetches upstream, merges, then runs a pipeline of typed steps
 * (Rust via cargo xtask, TypeScript via bun) to re-apply fork patches.
 *
 * Usage:
 *   bun tools/bun/sync-upstream.ts                    # Full sync
 *   bun tools/bun/sync-upstream.ts --skip-merge        # Re-apply patches only
 *   bun tools/bun/sync-upstream.ts --skip-verify       # Skip verification
 *   bun tools/bun/sync-upstream.ts --extended-verify    # Include tests
 *   bun tools/bun/sync-upstream.ts --step fix-package-names  # Single step
 */

const UPSTREAM_REMOTE = "upstream";
const UPSTREAM_BRANCH = "main";

// ── CLI parsing ────────────────────────────────────────────────────────────

type Options = {
    skipMerge: boolean;
    skipVerify: boolean;
    extendedVerify: boolean;
    step: string | null;
};

function parseArgs(): Options {
    const opts: Options = {
        skipMerge: false,
        skipVerify: false,
        extendedVerify: false,
        step: null,
    };

    const args = process.argv.slice(2);
    for (let i = 0; i < args.length; i++) {
        switch (args[i]) {
            case "--skip-merge":
                opts.skipMerge = true;
                break;
            case "--skip-verify":
                opts.skipVerify = true;
                break;
            case "--extended-verify":
                opts.extendedVerify = true;
                break;
            case "--step":
                opts.step = args[++i];
                if (!opts.step) {
                    console.error("--step requires a step name");
                    process.exit(1);
                }
                break;
            case "-h":
            case "--help":
                console.log(`Usage: bun tools/bun/sync-upstream.ts [OPTIONS]

Options:
  --skip-merge         Skip git fetch/merge (re-apply patches only)
  --skip-verify        Skip verification step
  --extended-verify    Run tests during verification
  --step <name>        Run a single step
  -h, --help           Show this help`);
                process.exit(0);
            default:
                console.error(`Unknown option: ${args[i]}`);
                process.exit(1);
        }
    }

    return opts;
}

// ── Process spawning ───────────────────────────────────────────────────────

async function run(cmd: string, args: string[]): Promise<void> {
    const proc = Bun.spawn({
        cmd: [cmd, ...args],
        cwd: process.cwd(),
        stdout: "inherit",
        stderr: "inherit",
    });

    const exitCode = await proc.exited;
    if (exitCode !== 0) {
        throw new Error(`Command failed (exit ${exitCode}): ${cmd} ${args.join(" ")}`);
    }
}

async function runOutput(cmd: string, args: string[]): Promise<string> {
    const proc = Bun.spawn({
        cmd: [cmd, ...args],
        cwd: process.cwd(),
        stdout: "pipe",
        stderr: "inherit",
    });

    const output = await new Response(proc.stdout).text();
    const exitCode = await proc.exited;
    if (exitCode !== 0) {
        throw new Error(`Command failed (exit ${exitCode}): ${cmd} ${args.join(" ")}`);
    }
    return output.trim();
}

async function runMayFail(cmd: string, args: string[]): Promise<number> {
    const proc = Bun.spawn({
        cmd: [cmd, ...args],
        cwd: process.cwd(),
        stdout: "inherit",
        stderr: "inherit",
    });
    return await proc.exited;
}

async function git(...args: string[]): Promise<void> {
    await run("git", args);
}

async function gitOutput(...args: string[]): Promise<string> {
    return runOutput("git", args);
}

async function cargo(...args: string[]): Promise<void> {
    await run("cargo", args);
}

async function bun(...args: string[]): Promise<void> {
    await run("bun", args);
}

// ── Step: fetch and merge ──────────────────────────────────────────────────

const CONFLICT_STRATEGY: Record<string, "ours" | "theirs"> = {
    "Cargo.lock": "theirs",
    "bun.lock": "theirs",
    "yarn.lock": "theirs",
    "rust-toolchain.toml": "ours",
    "Cargo.toml": "theirs",
};

async function resolveConflicts(): Promise<void> {
    const conflicted = (await gitOutput("diff", "--name-only", "--diff-filter=U")).split("\n").filter(Boolean);
    const unresolved: string[] = [];

    for (const file of conflicted) {
        // Check explicit strategy
        const strategy = CONFLICT_STRATEGY[file];
        if (strategy) {
            console.log(`  ${file} → accepting ${strategy}`);
            await git("checkout", `--${strategy}`, file);
            await git("add", file);
            continue;
        }

        // Pattern-based: member Cargo.toml files → accept theirs
        if (file.match(/^(crates|tools|xtask)\/.*Cargo\.toml$/)) {
            console.log(`  ${file} → accepting theirs (edition fix in later step)`);
            await git("checkout", "--theirs", file);
            await git("add", file);
            continue;
        }

        unresolved.push(file);
    }

    if (unresolved.length > 0) {
        console.error("\nUnresolved merge conflicts:");
        for (const f of unresolved) {
            console.error(`  ${f}`);
        }
        console.error("\nResolve manually, then run:");
        console.error("  git add <files> && git commit --no-edit");
        console.error("  bun tools/bun/sync-upstream.ts --skip-merge");
        process.exit(1);
    }

    await git("commit", "--no-edit");
}

async function fetchAndMerge(): Promise<void> {
    // Verify clean worktree
    const status = await gitOutput("status", "--porcelain");
    if (status.length > 0) {
        throw new Error("Working tree has uncommitted changes. Commit or stash first.");
    }

    // Verify upstream remote exists
    try {
        await gitOutput("remote", "get-url", UPSTREAM_REMOTE);
    } catch {
        throw new Error(
            `Remote '${UPSTREAM_REMOTE}' not found. Add it with:\n` +
            `  git remote add ${UPSTREAM_REMOTE} https://github.com/swc-project/swc.git`
        );
    }

    await git("fetch", UPSTREAM_REMOTE, UPSTREAM_BRANCH, "--tags");

    const behind = parseInt(
        await gitOutput("rev-list", "--count", `HEAD..${UPSTREAM_REMOTE}/${UPSTREAM_BRANCH}`)
    );

    if (behind === 0) {
        console.log("Already up to date with upstream.");
        return;
    }
    console.log(`${behind} new commit(s) from upstream.`);

    const branch = `sync/upstream-${new Date().toISOString().slice(0, 10)}`;

    // Check if branch exists
    const branchExists = (await runMayFail("git", ["show-ref", "--verify", "--quiet", `refs/heads/${branch}`])) === 0;
    if (branchExists) {
        console.log(`Branch ${branch} already exists. Checking it out.`);
        await git("checkout", branch);
    } else {
        await git("checkout", "-b", branch);
    }

    const mergeExit = await runMayFail("git", ["merge", `${UPSTREAM_REMOTE}/${UPSTREAM_BRANCH}`, "--no-edit"]);
    if (mergeExit === 0) {
        console.log("Merge completed cleanly.");
        return;
    }

    console.log("Merge conflicts detected. Attempting auto-resolution...");
    await resolveConflicts();
    console.log("Merge completed with auto-resolved conflicts.");
}

// ── Pipeline ───────────────────────────────────────────────────────────────

type Step = {
    name: string;
    run: () => Promise<void>;
};

function buildSteps(opts: Options): Step[] {
    const steps: Step[] = [];

    if (!opts.skipMerge) {
        steps.push({ name: "fetch-and-merge", run: fetchAndMerge });
    }

    steps.push(
        { name: "fix-cargo-workspace", run: () => cargo("xtask", "sync", "fix-cargo-workspace") },
        { name: "fix-edition", run: () => cargo("xtask", "sync", "fix-edition") },
        { name: "fix-package-names", run: () => bun("tools/bun/fix-package-names.ts") },
        { name: "fix-rust-refs", run: () => cargo("xtask", "sync", "fix-rust-refs") },
        { name: "fix-toolchain", run: () => cargo("xtask", "sync", "fix-toolchain") },
        { name: "regenerate-lockfiles", run: () => cargo("xtask", "sync", "regenerate-lockfiles") },
        { name: "format-and-lint", run: () => cargo("xtask", "sync", "format-and-lint") },
    );

    if (!opts.skipVerify) {
        const verifyArgs = opts.extendedVerify
            ? ["xtask", "sync", "verify", "--extended"]
            : ["xtask", "sync", "verify"];
        steps.push({ name: "verify", run: () => cargo(...verifyArgs) });
    }

    return steps;
}

// ── Main ───────────────────────────────────────────────────────────────────

const opts = parseArgs();
const allSteps = buildSteps(opts);

// Single step mode
if (opts.step) {
    const step = allSteps.find((s) => s.name === opts.step);
    if (!step) {
        console.error(`Unknown step: ${opts.step}`);
        console.error("Available steps:");
        for (const s of allSteps) {
            console.error(`  ${s.name}`);
        }
        process.exit(1);
    }
    await step.run();
    process.exit(0);
}

// Full pipeline
console.log("");
console.log("╔══════════════════════════════════════════════════╗");
console.log("║        swc-bun upstream sync pipeline            ║");
console.log("╚══════════════════════════════════════════════════╝");
console.log("");

for (let i = 0; i < allSteps.length; i++) {
    const step = allSteps[i];
    const label = `[${i + 1}/${allSteps.length}] ${step.name}`;
    console.log(`\n==> ${label}`);
    console.log("─".repeat(55));

    try {
        await step.run();
        console.log(`✓  ${label} complete`);
    } catch (err) {
        console.error(`✗  ${label} FAILED`);
        console.error(err instanceof Error ? err.message : String(err));
        console.error(`\nFix the issue, then resume with:`);
        console.error(`  bun tools/bun/sync-upstream.ts --skip-merge --step ${step.name}`);
        process.exit(1);
    }
}

console.log("");
console.log("╔══════════════════════════════════════════════════╗");
console.log("║        Sync pipeline complete!                   ║");
console.log("╚══════════════════════════════════════════════════╝");
console.log("");
console.log("Next steps:");
console.log("  1. Review changes: git diff --stat");
console.log("  2. Commit:         git add -A && git commit -m 'chore(workspace): sync upstream and re-apply fork patches'");
console.log("  3. Push:           git push -u origin $(git branch --show-current)");
console.log(`  4. Create PR:      gh pr create --title 'chore: sync upstream ${new Date().toISOString().slice(0, 10)}'`);
