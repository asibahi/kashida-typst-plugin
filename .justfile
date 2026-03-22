check:
	cargo check --quiet

build:
	cargo build --release --target wasm32-unknown-unknown
