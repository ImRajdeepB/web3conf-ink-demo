build-chain-ext:
	cargo +nightly contract build --manifest-path ./chain_ext/Cargo.toml

build-token:
	cargo +nightly contract build --manifest-path ./token/Cargo.toml

build:
	make build-chain-ext
	make build-token
