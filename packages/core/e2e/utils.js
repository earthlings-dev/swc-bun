import assert from "assert";
import { globSync } from "glob";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export const getPkgRoot = (() => () => {
    let ret;

    if (!ret) {
        ret = path.resolve(__dirname, "..");
    }
    return ret;
})();

/**
 * Temporarily move out existing napi bindings to avoid test fixture setup overwrite it.
 */
export const preserveBinaries = async (fromExt, toExt) => {
    const existingBinary = globSync(`${getPkgRoot()}/*.${fromExt}`);
    assert.equal(
        existingBinary.length <= 1,
        true,
        "There are more than one prebuilt binaries, current test fixture setup cannot handle this"
    );

    const binaryPath = existingBinary[0];
    if (!binaryPath) {
        return;
    }

    const tmpBinaryPath = path.join(
        path.dirname(binaryPath),
        `${path.basename(binaryPath, `.${fromExt}`)}.${toExt}`
    );

    await fs.promises.rename(binaryPath, tmpBinaryPath);
};
