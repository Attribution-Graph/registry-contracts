# registry-contracts

Soroban smart contracts for the Attribution Graph platform. Provides on-chain attestations linking Stellar accounts to GitHub repository contributions.

## Contracts

### `registry`

The primary contract managing:
- **Attestations** — contributor claims bound to a repo URL and Stellar address
- **Revocations** — ability to invalidate attestations
- **Repo Bindings** — mapping GitHub repository URLs to Stellar account owners

## Development

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all
```

## License

MIT OR Apache-2.0
