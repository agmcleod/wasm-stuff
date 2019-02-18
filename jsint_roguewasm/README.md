Dependencies:

    cargo install wasm-bindgen-cli
    rustup target add wasm32-unknown-unknown

Then compile

    cargo build --target wasm32-unknown-unknown
    wasm-bindgen target/wasm32-unknown-unknown/debug/jsint_roguewasm.wasm --out-dir .



