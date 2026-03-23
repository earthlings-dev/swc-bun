import { rmSync } from "node:fs";
import path from "node:path";

type Classification = {
    fixturePackageJsons: string[];
    unmanagedPackageJsons: string[];
    workspacePackageJsons: string[];
};

const rootDir = process.cwd();
const dryRun = process.argv.includes("--dry-run");
const rootPackageJsonPath = "package.json";

const fixturePackageJsons = [
    "crates/swc/tests/fixture/jsc-paths/vercel-site/1/input/package.json",
    "crates/swc_ecma_loader/tests/hoisting/packages/app/package.json",
    "crates/swc_ecma_parser/tests/test262-parser/package.json",
    "crates/swc_ecma_transforms_proposal/tests/decorator-tests/package.json",
    "crates/swc_estree_compat/tests/package.json",
    "crates/swc_node_bundler/tests/package.json",
    "crates/swc_node_bundler/tests/integration/react/package.json",
    "crates/swc_node_bundler/tests/pass/resolve-name-fix/input/package.json",
].sort();

const allPackageJsons = Array.from(
    new Bun.Glob("**/package.json").scanSync({
        cwd: rootDir,
        onlyFiles: true,
    })
)
    .filter((file) => !file.includes("/node_modules/"))
    .sort();

const rootPackageJson = await Bun.file(path.join(rootDir, "package.json")).json();
const workspacePatterns: string[] = rootPackageJson.workspaces;

const workspacePackageJsons = Array.from(
    new Set(
        [
            rootPackageJsonPath,
            ...workspacePatterns.flatMap((pattern) => {
                const normalizedPattern = pattern.replace(/^\.\//, "");
                const packageJsonPattern = path.posix.join(
                    normalizedPattern,
                    "package.json"
                );

                return Array.from(
                    new Bun.Glob(packageJsonPattern).scanSync({
                        cwd: rootDir,
                        onlyFiles: true,
                    })
                );
            }),
        ]
    )
)
    .filter((file) => !file.includes("/node_modules/"))
    .sort();

function classify(): Classification {
    const fixtureSet = new Set(fixturePackageJsons);
    const workspaceSet = new Set(workspacePackageJsons);

    const unmanagedPackageJsons = allPackageJsons.filter((file) => {
        return !fixtureSet.has(file) && !workspaceSet.has(file);
    });

    return {
        fixturePackageJsons,
        unmanagedPackageJsons,
        workspacePackageJsons,
    };
}

async function runCommand(cmd: string[], cwd: string) {
    const proc = Bun.spawn({
        cmd,
        cwd,
        stdout: "inherit",
        stderr: "inherit",
    });

    const exitCode = await proc.exited;
    if (exitCode !== 0) {
        throw new Error(
            `Command failed with exit code ${exitCode}: ${cmd.join(" ")}`
        );
    }
}

function cleanupInstallState(dir: string) {
    rmSync(path.join(dir, "bun.lock"), { force: true });
    rmSync(path.join(dir, "node_modules"), { recursive: true, force: true });
}

function cleanupWorkspaceInstallState() {
    const workspaceDirs = new Set(
        workspacePackageJsons.map((file) => path.join(rootDir, path.dirname(file)))
    );

    for (const dir of workspaceDirs) {
        cleanupInstallState(dir);
    }
}

function printClassification(result: Classification) {
    console.log(`workspace-managed package.json files: ${result.workspacePackageJsons.length}`);
    console.log(`fixture-only package.json files: ${result.fixturePackageJsons.length}`);
    console.log(`unmanaged package.json files: ${result.unmanagedPackageJsons.length}`);

    if (result.fixturePackageJsons.length > 0) {
        console.log("\nfixture-only manifests:");
        for (const file of result.fixturePackageJsons) {
            console.log(`  - ${file}`);
        }
    }

    if (result.unmanagedPackageJsons.length > 0) {
        console.log("\nunmanaged manifests:");
        for (const file of result.unmanagedPackageJsons) {
            console.log(`  - ${file}`);
        }
    }
}

const classification = classify();
printClassification(classification);

if (classification.unmanagedPackageJsons.length > 0) {
    throw new Error("Every package.json must be workspace-managed or fixture-exempt.");
}

if (dryRun) {
    process.exit(0);
}

await runCommand(["bun", "update", "--latest", "--recursive"], rootDir);
await runCommand(["bun", "install", "--force"], rootDir);
cleanupWorkspaceInstallState();

for (const fixturePackageJson of fixturePackageJsons) {
    const fixtureDir = path.join(rootDir, path.dirname(fixturePackageJson));

    await runCommand(["bun", "update", "--latest"], fixtureDir);
    await runCommand(["bun", "install"], fixtureDir);
    cleanupInstallState(fixtureDir);
}
