tic-tac-toe-wasm
================

Tic Tac Toe in Rust compiled to WebAssembly

Before building:
----------------

Add the WebAssembly target:

```
rustup target add wasm32-unknown-unknown
```

Install cargo-web:

```
cargo install -f cargo-web
```

Running in developer mode:
-------------------------

```
cargo web start --target-webasm
```

Then open the browser with the url given.

Building for a static web page:
----------------------------

```
cargo web build --target-webasm
```

Then copy `target/wasm32-unknown-unknown/release/tic_tac_toe_wasm.wasm` and `target/wasm32-unknown-unknown/release/tic_tac_toe_wasm.js`
into the `build/` folder.
