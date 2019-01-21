MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.SHELLFLAGS := -eu -o pipefail -c
.DEFAULT_GOAL := start
.DELETE_ON_ERROR:
.SUFFIXES:

.PHONY: build
build: www-build

.PHONY: install
install: rust-build www-build
	cd monkey-web/www/ && npm install

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

.PHONY: www-release
www-release: rust-release
	cd monkey-web/www/ && npm run build

.PHONY: rust-test
rust-test:
	cargo test

.PHONY: www-watch
www-watch: rust-build
	cd monkey-web/www/ && npm start
