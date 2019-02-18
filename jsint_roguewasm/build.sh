cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/jsint_roguewasm.wasm --out-dir .