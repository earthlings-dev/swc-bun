/**
 * A postinstall script runs after `@swc/core` is installed.
 *
 * It checks if corresponding optional dependencies for native binary is installed and can be loaded properly.
 * If it fails, it'll internally try to install `@swc/wasm` as fallback.
 */
import { existsSync } from "fs";
import * as assert from "assert";
import * as path from "path";
import * as fs from "fs";

/**
 * Trying to validate @swc/core's native binary installation, then installs if it is not supported.
 */
const validateBinary = async () => {
    try {
        const { name } = require(path.resolve(
            process.env.INIT_CWD!,
            "package.json"
        ));
        if (name === "@swc/core" || name === "@swc/workspace") {
            return;
        }
    } catch (_) {
        return;
    }

    // TODO: We do not take care of the case if user try to install with `--no-optional`.
    // For now, it is considered as deliberate decision.
    let binding;
    try {
        binding = require("./binding.js");

        // Check if binding binary actually works.
        // For the latest version, checks target triple. If it's old version doesn't have target triple, use parseSync instead.
        const triple = binding.getTargetTriple
            ? binding.getTargetTriple()
            : binding.parseSync(
                  "console.log()",
                  Buffer.from(JSON.stringify({ syntax: "ecmascript" }))
              );
        assert.ok(triple, "Failed to read target triple from native binary.");
    } catch (error: any) {
        // if error is unsupported architecture, ignore to display.
        if (!error.message?.includes("Unsupported architecture")) {
            console.warn(error);
        }

        console.warn(
            `@swc/core was not able to resolve native bindings installation. It'll try to use @swc/wasm as fallback instead.`
        );
    }

    if (!!binding) {
        return;
    }

    // User choose to override the binary installation. Skip remanining validation.
    if (!!process.env["SWC_BINARY_PATH"]) {
        console.warn(
            `@swc/core could not resolve native bindings installation, but found manual override config SWC_BINARY_PATH specified. Skipping remaning validation.`
        );
        return;
    }

    // Check if top-level package.json installs @swc/wasm separately already
    let wasmBinding;
    try {
        wasmBinding = require.resolve(`@swc/wasm`);
    } catch (_) {}

    if (!!wasmBinding && existsSync(wasmBinding)) {
        return;
    }

    const env = { ...process.env, npm_config_global: undefined };
    const { version } = require(path.join(
        path.dirname(require.resolve("@swc/core")),
        "package.json"
    ));

    // We want to place @swc/wasm next to the @swc/core as if normal installation was done,
    // but can't directly set cwd to INIT_CWD as npm seems to acquire lock to the working dir.
    // Instead, create a temporary inner and move it out.
    const coreDir = path.dirname(require.resolve("@swc/core"));
    const installDir = path.join(coreDir, "npm-install");

    try {
        fs.mkdirSync(installDir);
        fs.writeFileSync(path.join(installDir, "package.json"), "{}");

        // Keep the fallback installation Bun-native so package recovery does not
        // depend on a separate npm installation being available on the machine.
        const result = Bun.spawnSync({
            cmd: [
                process.execPath,
                "add",
                "--no-save",
                "--silent",
                "--no-progress",
                "--ignore-scripts",
                `@swc/wasm@${version}`,
            ],
            cwd: installDir,
            env,
            stdout: "pipe",
            stderr: "pipe",
        });

        if (!result.success) {
            throw new Error(
                Buffer.from(result.stderr).toString() ||
                    `bun add exited with code ${result.exitCode}`
            );
        }

        const installedBinPath = path.join(
            installDir,
            "node_modules",
            `@swc/wasm`
        );
        const targetDir = path.resolve(
            process.env.INIT_CWD!,
            "node_modules",
            `@swc/wasm`
        );

        fs.mkdirSync(path.dirname(targetDir), { recursive: true });
        // INIT_CWD is injected via npm. If it doesn't exists, can't proceed.
        fs.renameSync(installedBinPath, targetDir);
    } catch (error) {
        console.error(error);

        console.error(
            `Failed to install fallback @swc/wasm@${version}. @swc/core will not properly.
Please install @swc/wasm manually, or retry whole installation.
If there are unexpected errors, please report at https://github.com/swc-project/swc/issues`
        );
    } finally {
        try {
            fs.rmSync(installDir, { recursive: true, force: true });
        } catch (_) {
            // Gracefully ignore any failures. This'll make few leftover files but it shouldn't block installation.
        }
    }
};

validateBinary().catch((error) => {
    // for now just throw the error as-is.
    throw error;
});
