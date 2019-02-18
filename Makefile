export PATH := ~/.cargo/bin:$(PATH)

MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := build
.DELETE_ON_ERROR:
.SUFFIXES:

.PHONY: build
build: build-rust build-www

.PHONY: build-rust
build-rust: test-rust
	cargo build --release

.PHONY: build-wasm
build-wasm: build-rust
	./wasm-release

.PHONY: build-www
build-www: build-rust build-wasm
	# install needs to happen after the wasm build has finished
	cd monkey-web/www/ && npm install && npm run build

.PHONY: test-rust
test-rust:
	cargo test

.PHONY: bootstrap
bootstrap:
	curl https://sh.rustup.rs -sSf | sh -s - -y
	cargo install wasm-pack

.PHONY: watch-rust
watch-rust:
	cargo watch -x test -s 'wasm-pack build monkey-web' --ignore 'monkey-web/{pkg,www}/**/*'

.PHONY: watch-www
watch-www: build-rust
	cd monkey-web/www/ && npm start
