import { rmSync } from "node:fs";
import path from "node:path";

import { type Classification, classify as classifyPackages } from "./classify.ts";

const rootDir = process.cwd();
const dryRun = process.argv.includes("--dry-run");

const classification = await classifyPackages(rootDir);
const { workspacePackageJsons } = classification;

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

for (const fixturePackageJson of classification.fixturePackageJsons) {
    const fixtureDir = path.join(rootDir, path.dirname(fixturePackageJson));

    await runCommand(["bun", "update", "--latest"], fixtureDir);
    await runCommand(["bun", "install"], fixtureDir);
    cleanupInstallState(fixtureDir);
}
