/**
 * Rename @swc/* → @swc-bun/* and @swc-contrib/* → @swc-bun-contrib/*
 * across package.json files, binding.js files, and JS/TS source files.
 *
 * Uses JSON-aware parsing for package.json (not regex on raw text).
 * Excludes test fixtures under crates/\*\/tests/.
 *
 * Usage: bun tools/bun/fix-package-names.ts
 */

import { readFileSync, writeFileSync, existsSync } from "node:fs";
import path from "node:path";
import { classify } from "./classify.ts";

type RenameResult = {
    packageJsons: { updated: string[]; skipped: string[] };
    bindings: { updated: string[]; skipped: string[] };
    sources: { updated: string[]; skipped: string[] };
    errors: string[];
};

const RENAMES: [RegExp, string][] = [
    // Order matters: @swc-contrib must be replaced before @swc
    [/@swc-contrib\//g, "@swc-bun-contrib/"],
    [/@swc\//g, "@swc-bun/"],
];

const BINDING_FILES = [
    "packages/core/binding.js",
    "packages/html/binding.js",
    "packages/minifier/binding.js",
    "packages/minifier/src/binding.js",
    "packages/react-compiler/binding.js",
    "packages/react-compiler/src/binding.js",
    "extra-bindings/packages/css/binding.js",
    "extra-bindings/packages/linter/binding.js",
];

const SOURCE_DIRS = [
    "packages",
    "pkgs",
    "bindings",
    "extra-bindings",
    "plugins",
    ".github",
    "scripts",
];

const SOURCE_EXTENSIONS = "*.{js,ts,mjs,cjs,mts,cts,tsx,jsx}";

function applyRenames(text: string): string {
    let result = text;
    for (const [pattern, replacement] of RENAMES) {
        result = result.replace(pattern, replacement);
    }
    return result;
}

function walkJsonValues(obj: unknown): unknown {
    if (typeof obj === "string") {
        return applyRenames(obj);
    }
    if (Array.isArray(obj)) {
        return obj.map(walkJsonValues);
    }
    if (obj !== null && typeof obj === "object") {
        const result: Record<string, unknown> = {};
        for (const [key, value] of Object.entries(obj)) {
            const newKey = applyRenames(key);
            result[newKey] = walkJsonValues(value);
        }
        return result;
    }
    return obj;
}

async function fixPackageJsons(rootDir: string): Promise<RenameResult["packageJsons"]> {
    const { workspacePackageJsons, fixturePackageJsons } = await classify(rootDir);
    const allTargets = [...workspacePackageJsons, ...fixturePackageJsons];

    const updated: string[] = [];
    const skipped: string[] = [];

    for (const relPath of allTargets) {
        const absPath = path.join(rootDir, relPath);
        if (!existsSync(absPath)) {
            continue;
        }

        const raw = readFileSync(absPath, "utf-8");

        // Quick check: does this file contain @swc/ at all?
        if (!raw.includes("@swc/") && !raw.includes("@swc-contrib/")) {
            skipped.push(relPath);
            continue;
        }

        const parsed = JSON.parse(raw);
        const transformed = walkJsonValues(parsed);
        const newRaw = JSON.stringify(transformed, null, 4) + "\n";

        if (newRaw !== raw) {
            writeFileSync(absPath, newRaw);
            updated.push(relPath);
        } else {
            skipped.push(relPath);
        }
    }

    return { updated, skipped };
}

function fixBindingFiles(rootDir: string): RenameResult["bindings"] {
    const updated: string[] = [];
    const skipped: string[] = [];

    for (const relPath of BINDING_FILES) {
        const absPath = path.join(rootDir, relPath);
        if (!existsSync(absPath)) {
            continue;
        }

        const content = readFileSync(absPath, "utf-8");
        const newContent = applyRenames(content);

        if (newContent !== content) {
            writeFileSync(absPath, newContent);
            updated.push(relPath);
        } else {
            skipped.push(relPath);
        }
    }

    return { updated, skipped };
}

function fixSourceFiles(rootDir: string): RenameResult["sources"] {
    const updated: string[] = [];
    const skipped: string[] = [];

    for (const dir of SOURCE_DIRS) {
        const absDir = path.join(rootDir, dir);
        if (!existsSync(absDir)) {
            continue;
        }

        const glob = new Bun.Glob(`**/${SOURCE_EXTENSIONS}`);
        for (const relFile of glob.scanSync({ cwd: absDir, onlyFiles: true })) {
            // Skip node_modules and crate test fixtures
            if (relFile.includes("node_modules/")) continue;

            const fullRelPath = path.join(dir, relFile);
            const absPath = path.join(rootDir, fullRelPath);
            const content = readFileSync(absPath, "utf-8");

            if (!content.includes("@swc/") && !content.includes("@swc-contrib/")) {
                continue;
            }

            const newContent = applyRenames(content);
            if (newContent !== content) {
                writeFileSync(absPath, newContent);
                updated.push(fullRelPath);
            } else {
                skipped.push(fullRelPath);
            }
        }
    }

    return { updated, skipped };
}

function verify(rootDir: string): string[] {
    const errors: string[] = [];

    // Check package.json files
    const pkgGlob = new Bun.Glob("**/package.json");
    for (const relPath of pkgGlob.scanSync({ cwd: rootDir, onlyFiles: true })) {
        if (relPath.includes("node_modules/")) continue;
        if (relPath.match(/crates\/[^/]+\/tests\//)) continue;

        const content = readFileSync(path.join(rootDir, relPath), "utf-8");
        // Check for @swc/ that isn't @swc-bun/
        const match = content.match(/"@swc\/(?!bun)/);
        if (match) {
            errors.push(`${relPath}: still contains @swc/ reference`);
        }
    }

    // Check binding.js files
    for (const relPath of BINDING_FILES) {
        const absPath = path.join(rootDir, relPath);
        if (!existsSync(absPath)) continue;

        const content = readFileSync(absPath, "utf-8");
        if (/@swc\/(?!bun)/.test(content)) {
            errors.push(`${relPath}: still contains @swc/ reference`);
        }
    }

    return errors;
}

// ── Main ──────────────────────────────────────────────────────────────────

const rootDir = process.cwd();

console.log("Part A: Renaming @swc/ → @swc-bun/ in package.json files...");
const pkgResult = await fixPackageJsons(rootDir);
console.log(`  Updated ${pkgResult.updated.length}, skipped ${pkgResult.skipped.length}`);

console.log("Part B: Renaming @swc/ → @swc-bun/ in binding.js files...");
const bindingResult = fixBindingFiles(rootDir);
console.log(`  Updated ${bindingResult.updated.length}, skipped ${bindingResult.skipped.length}`);

console.log("Part C: Renaming @swc/ → @swc-bun/ in JS/TS source files...");
const sourceResult = fixSourceFiles(rootDir);
console.log(`  Updated ${sourceResult.updated.length}, skipped ${sourceResult.skipped.length}`);

console.log("Verifying no @swc/ references remain...");
const errors = verify(rootDir);

if (errors.length > 0) {
    console.error(`\n${errors.length} file(s) still contain @swc/ references:`);
    for (const err of errors) {
        console.error(`  ${err}`);
    }
    process.exit(1);
}

console.log("All workspace files use @swc-bun/");
