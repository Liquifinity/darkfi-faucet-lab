# Safe Contributor Quickstart

## Before You Start

Clone the public repository. Do not request maintainer wallets, VHDX files, raw tx artifacts, or local configs.

Every contributor should use their own local testnet environment.

This repository is Rust-first. Install a stable Rust toolchain before running local checks.

## Safe Contribution Types

- Documentation improvements.
- Test plans.
- Static analysis.
- Reproducible setup scripts that do not mutate wallet or chain state by default.
- Contract review notes.
- Security hardening proposals.

## Unsafe Without Maintainer Approval

- Broadcast scripts.
- Deploy scripts.
- Wallet DB access.
- Chain DB repair.
- Unspend/recovery operations.
- Production/Railway changes.
- Any script that spends, claims, mines for payout, or modifies state.

## Pull Request Requirements

Your PR should state:

- what changed;
- why it is safe;
- what commands you ran;
- whether it touches wallet, node, chain DB, contract, deploy, broadcast, or production.

Minimum local check:

```bash
cargo run --locked --bin publication-safety-check
```

If the answer is unclear, mark the PR as security-sensitive.
