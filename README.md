# 🌐 Abridge + WASM
A monolithic repository for WASM components to be exported for use in Abridge.

# ⚠️ Status
Currently this is all research and experimentation. Please submit pull requests
**Disclaimer**: At this time, optimization is not a consideration. Pay absolutely no mind to the WASM build size, as this is a different topic I have ideas on...

# 🏁 Quickstart
## Dependencies
- [Rust](https://www.rust-lang.org/tools/install)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
## Build
- `wasm-pack build (--release) --target web`
## Serve
- `python3 -m http-server`
- View: [http://0.0.0.0:8000/](http://0.0.0.0:8000/)