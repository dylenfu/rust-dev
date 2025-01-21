all: build-all-release

check-all: check-fmt check-clippy check-build

build-all-release:
	cargo build --release

fmt:
	cargo fmt --all

check-fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --fix --no-deps

check-clippy:
	cargo clippy --no-deps -- -D warnings

check-build:
	cargo check

dev-env:
	rustup toolchain install $(shell cat rust-toolchain)
	rustup component add --toolchain $(shell cat rust-toolchain) rust-src clippy rustfmt