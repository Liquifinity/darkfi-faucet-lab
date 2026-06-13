# Engineering Quality Bar

This is the minimum standard for changes to this repository.

## General Rule

Make the smallest change that improves the public faucet work without weakening
security or reproducibility.

## Required For Every PR

- No secrets.
- No wallet databases.
- No raw transaction artifacts.
- No WSL disks.
- No local configs.
- No private logs.
- Required CI passes.
- The PR explains whether it touches contract, wallet, node, deploy, broadcast,
  or production behavior.

## Required For Contract Changes

Contract changes must preserve:

- exact claim amount: `0.00003 DRK`;
- cooldown: `12 hours`;
- daily pool: `0.33 DRK`;
- no carry-over;
- pause blocks claims;
- top-up does not bypass limits;
- failed claims do not consume allowance;
- custody change stays coherent.

Contract changes should update:

- `contracts/faucet-pool/SPEC.md`;
- `contracts/faucet-pool/interface.json`;
- `contracts/faucet-pool/conformance-vectors.json`;
- model tests;
- Rust skeleton.

## Required For Operational Docs

Operational docs must separate:

- confirmed chain facts;
- local observations;
- assumptions;
- blocked or inconclusive states.

Never state that the faucet works unless a claim is confirmed and wallet receipt
is proven.

## Required Checks

```bash
cargo fmt --check
cargo run --locked --bin publication-safety-check
cargo check --manifest-path contracts/faucet-pool/rust-skeleton/Cargo.toml
node --test contracts/faucet-pool/pool-model.test.js
node contracts/faucet-pool/verify-contract-target.js
node --test contracts/top-up-guard/guard-model.test.js
```

## Merge Standard

CI passing is required. Maintainer judgment is still required for
security-sensitive changes.

