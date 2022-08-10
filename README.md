# web3conf-ink-demo

## Getting Started

Setup Rust, ink! environment:

https://docs.substrate.io/tutorials/smart-contracts/prepare-your-first-contract/

```
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

`sudo apt install binaryen` OR `brew install binaryen`

Install cargo-contract

```
cargo install dylint-link
cargo install cargo-contract --force
cargo contract --help
```

Compile the contracts

```
make build
```

Download swanky node: https://github.com/AstarNetwork/swanky-node/releases

Run swanky node

```
./swanky-node --dev
```

```
git clone https://github.com/AstarNetwork/swanky-node.git

git checkout dapp-staking-chain-extension

cargo build --release

./target/release/swanky-node --dev
```

## Credits

- [Astar Dapp Staking example](https://github.com/AstarNetwork/dApp-Staking-Workshop)
