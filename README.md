# Stremio seed example

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- `rustup update`
- `rustup target add wasm32-unknown-unknown`
- `cargo install cargo-make`
- `cargo install wasm-bindgen-cli`
- `cargo install trunk`

## Development

- `trunk serve`
  
- Open [localhost:8080](http://localhost:8080) in a browser.
- _Note_: Auto-reload isn't implemented in [Trunk](https://crates.io/crates/trunk) (yet).

## Addons

- Install [Ivshti/stremio-kyuchek](https://github.com/Ivshti/stremio-kyuchek) for testing videos on Chrome.
   - Manifest url: `https://ivshti.github.io/stremio-kyuchek/manifest.json`

## Server

- Clone [Stremio/stremio-server](https://github.com/Stremio/stremio-server) (if you have permissions)

- Install deps and run `node init.js` 

## Deploy (WIP)

- See `/.github/workflows_example/main.yml`
- Release build
   - `trunk build --release`
   - NPM `wasm-opt` command (until `trunk` installs `wasm-opt` automatically):
      
```bash
cd dist
wasm-opt -Oz index-581317a2fdc60cc_bg.wasm -o index-581317a2fdc60cc_bg.wasm
```

