export PATH := ~/.cargo/bin:$(PATH)

MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := start
.DELETE_ON_ERROR:
.SUFFIXES:

.PHONY: build
build: www-build

.PHONY: rust-build
rust-build:
	cargo build

.PHONY: www-build
www-build: rust-build
	cd monkey-web/www/ && npm run build

.PHONY: release
release: rust-release www-release

.PHONY: rust-release
rust-release: rust-test
	cargo build --release

.PHONY: wasm-release
wasm-release: rust-release
	./wasm-release

.PHONY: www-release
www-release: rust-release wasm-release
	# install needs to happen after the wasm build has finished
	cd monkey-web/www/ && npm install && npm run build

.PHONY: rust-test
rust-test:
	cargo test

.PHONY: bootstrap
bootstrap:
	curl https://sh.rustup.rs -sSf | sh -s - -y
	cargo install wasm-pack

.PHONY: rust-watch
rust-watch:
	cargo watch -x test -s 'wasm-pack build monkey-web' --ignore 'monkey-web/pkg/**/*'

.PHONY: www-watch
www-watch: rust-build
	cd monkey-web/www/ && npm start
