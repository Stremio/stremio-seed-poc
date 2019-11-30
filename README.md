# Stremio seed example

## Prerequisites

- [Node.js](https://nodejs.org/en/)
- [Rust](https://www.rust-lang.org/tools/install)
- `rustup update`
- `rustup target add wasm32-unknown-unknown`
- `cargo install --force cargo-make`

## Init project

- `npm install`

## Development

- In standalone terminals:
  - `cargo make watch`
  - `cargo make watch_less`
  - `cargo make serve`
  
- Open [localhost:8000](http://localhost:8000) in a browser.
- _Note_: Auto-reload isn't implemented (yet).

## Test release version

- `cargo make bundle`
- `cargo make serve_dist`

## Deploy (WIP)

- See `/.github/workflows_example/main.yml`

