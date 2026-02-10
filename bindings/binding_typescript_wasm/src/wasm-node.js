import initAsync from "./wasm.js";

const wasm = new URL("./wasm_bg.wasm", import.meta.url);

export default function (init = Bun.file(wasm).arrayBuffer()) {
    return initAsync(init);
}

export * from "./wasm.js";
