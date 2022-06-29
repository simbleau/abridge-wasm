# 🌐 Abridge + WASM
A monolithic repository for WASM components to be exported for use in Abridge.

# Demos
- See [demos](https://simbleau.github.io/abridge-wasm/demos/) served by GitHub Pages.

# ⚠️ Status
Currently this is all research and experimentation. Please submit pull requests/issues.

**Disclaimer**: At this time, optimization is not a consideration. Pay absolutely no mind to the WASM build size, as this is a different topic I have ideas on...

# 🏁 Quickstart
## Dependencies
- [Rust](https://www.rust-lang.org/tools/install)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

### Install Rust:
```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
1
reboot
rustup update
```

### Install wasm-pack
```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sudo sh
```

## Build
```shell
git clone https://github.com/simbleau/abridge-wasm
cd abridge-wasm
cargo build --release
wasm-pack build --release --target web
```

## Serve
```shell
python3 -m http.server
```
View: [http://0.0.0.0:8000/](http://0.0.0.0:8000/)

# Exporting to Abridge
## Selecting the exports
- [lib.rs](src/lib.rs) will export **any** functions annotated with `#[wasm_bindgen]` that are (privately or publicly) included. When creating a demo, it makes sense to only export the module for the experiment. However, for Abridge, we should include everything a page needs.
- [Build](#build) the WASM artifacts.
- You will only need `abridge_wasm_bg.wasm` and `abridge_wasm.js`. The other files were generated for npm, and serve us no purpose. I am unsure how to only generate these 2 files.

## Optimizing WASM
- I have optimized the WASM settings in the Cargo.toml to aggressively optimize for size. These is [a good reference on this](https://rustwasm.github.io/book/reference/code-size.html).
- *Help needed*: steps to minify the javascript file exported

## Linking WASM
- You may need to make a `wasm-glue.js` wrapper which imports the `abridge_wasm.js` and attaches functionality to certain elements
  - e.g. [example](demos/theme-switch/wasm-glue.js)
- Include the WASM glue script in the head of your HTML.
  - e.g. `<script defer type="module" src="wasm-glue.js"></script>`.