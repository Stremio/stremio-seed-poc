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

## Deploy (WIP)

- See `/.github/workflows_example/main.yml`

