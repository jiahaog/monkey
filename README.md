# Monkey Interpreter (WIP)

Building a interpreter for the Monkey programming language.

Walking through [Writing An Interpreter In Go](https://interpreterbook.com/) in Rust.

## Development

### Prerequistes

- [Rust Toolchain](https://rustup.rs/)

#### For WebAssembly Development

- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
- [npm](https://www.npmjs.com/get-npm)

### Build

Builds Rust binaries and static web pages to `monkey-web/www/dist/`

```sh
make
```

### Watching for Changes

[Cargo Watch](https://github.com/passcod/cargo-watch) is required.

```sh
# In separate terminal processes:

make watch-rust

make watch-www
```

### Rust Only Development

```sh
cd monkey

# REPL
cargo run
```
