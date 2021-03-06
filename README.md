# 🌐 Abridge + WASM
A monolithic repository for WASM components to be exported for use in [Abridge](https://github.com/Jieiku/abridge).

# ✅ Demos
- See [demos](https://simbleau.github.io/abridge-wasm/demos/) served by GitHub Pages.

# ⚠️ Status
Currently this is all research and experimentation. Please submit pull requests/issues.

At this time, optimization is not a priority. [There are several ways to optimize the WASM bundle size](https://rustwasm.github.io/book/reference/code-size.html) once Abridge's WASM components are made and algorithms are optimized.

# 🏁 Quickstart
## Dependencies
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Install Rust:
```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustup update
```

### Install wasm-pack
```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Build
```shell
git clone https://github.com/simbleau/abridge-wasm
cd abridge-wasm
wasm-pack build --release --target web
```

## Serve
```shell
python3 -m http.server
```

- View: [http://0.0.0.0:8000/](http://0.0.0.0:8000/)
- View Demo: [http://0.0.0.0:8000/demos/theme-switch/](http://0.0.0.0:8000/demos/theme-switch/)

# 🟧 Exporting to Abridge
## Building WASM
- [lib.rs](src/lib.rs) will export **any** modules that are (privately or publicly) included. When creating a demo, it makes sense to only export the module for the demo itself. However, for Abridge, we should include only everything a certain page needs.
- [Build](#build) the WASM artifacts.
- You will only need `abridge_wasm_bg.wasm` and `abridge_wasm.js`. The other files were generated for npm, and serve us no purpose. I am unsure if there is a way to only generate these 2 files.

## Optimizing Bundle Size
### Release settings
- I have only optimized the WASM settings `build --release` settings in the [`Cargo.toml`](Cargo.toml).
### Avoid macros and type conversions
- *At all costs*. [They add bloat](https://rustwasm.github.io/book/reference/code-size.html#avoid-string-formatting).
### Avoid panicky code
- *At all costs*. [Panicking adds bloat, similar to macros](https://rustwasm.github.io/book/reference/code-size.html#avoid-panicking).
- [`twiggy`](https://github.com/rustwasm/twiggy) can be used to inspect LLVM-IR to determine which functions are panicking. The functions do not need to call `panic()`. Instead, they can arise naturally:
  - Indexing a slice panics on out of bounds indices: `my_slice[i]`
  - Division will panic if the divisor is zero: `dividend / divisor`
  - Unwrapping an `Option` or `Result`: `opt.unwrap()` or `res.unwrap()`
### Twiggy guide
- Install twiggy: `cargo install twiggy`
- Re-build WASM with `--dev` to retain function symbols, e.g. `wasm-pack build --dev --target web`
- Some things I check:
  - **Currently a lot more research needs to go into this...**
  - `twiggy garbase pkg/abridge_wasm_bg.wasm`, for dead functions. I am unsure how to strip these functions right now.
  - `twiggy top pkg/abridge_wasm_bg.wasm | head`, for the largest functions
  - `twiggy paths pkg/abridge_wasm_bg.wasm | grep panic -B 1 | grep -v '⬑' | grep -v '\-\-'`, A hacky script to check if panics are used in the WASM
### Minify JavaScript files
- *Help needed*: steps to minify the javascript file exported
### No-std?
- *Help needed*: *I am currently unsure if no-std and [wee_alloc](https://github.com/rustwasm/wee_alloc) will help. 

## Linking WASM
## Method 1: WASM-Glue
- You may need to make a `wasm-glue.js` wrapper which imports the `abridge_wasm.js` and attaches functionality to certain elements
  - e.g. [example](demos/theme-switch/wasm-glue.js)
- Include the WASM glue script in the head of your HTML.
  - e.g. `<script defer type="module" src="wasm-glue.js"></script>`.
## Method 2: Pure WASM with light wrapper
- I need to test which is better still. It is possible to grab the HTML elements outright by ID in Rust and add functionality inline, such as click handlers or changing a class. This would eliminate the need for a `glue` file alltogether. This will be my next experiment.