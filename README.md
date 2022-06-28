# 🌐 Abridge + WASM
A monolithic repository for WASM components to be exported for use in Abridge.

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