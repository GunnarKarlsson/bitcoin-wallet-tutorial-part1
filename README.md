# Bitcoin Wallet in Rust — Part I

Companion code for part I of the [Let's Build a Bitcoin Wallet in Rust](https://medium.com/@gunnar.h.karlsson/lets-build-a-bitcoin-wallet-in-rust-part-i-7c3c80110d68) tutorial series.

This part generates a Bitcoin testnet keypair, derives a P2PKH address, and saves the wallet to `wallet.json` so it survives restarts.

Requires Rust 1.85 or later.

## Run

Create a new wallet:

```bash
cargo run -- new
```

Show the saved address:

```bash
cargo run
```

This is testnet learning code. The private key is stored unencrypted in `wallet.json` — do not use it on mainnet.

## License

MIT. See [LICENSE](LICENSE).
