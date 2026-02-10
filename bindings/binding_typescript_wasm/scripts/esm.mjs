import { cp } from "node:fs/promises";

const pkgJson = await Bun.file("esm/package.json").json();
pkgJson.name = '@swc/wasm-typescript-esm';
pkgJson.exports = {
    types: "./wasm.d.ts",
    node: "./wasm-node.js",
    default: "./wasm.js",
};

await Promise.all([
    cp("src/wasm-node.js", "esm/wasm-node.js"),
    Bun.write("esm/package.json", JSON.stringify(pkgJson, null, 2)),
]);
