wasm-pack build --out-name wasm --out-dir esm --release --scope=swc --target web
bun ./scripts/esm.mjs
