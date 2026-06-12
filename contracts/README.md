# Faucet Contracts

This directory contains the public FaucetPool design, executable model, and
Rust/WASM skeleton.

The historical FaucetPool deployment is documented as prior testnet work. Do
not treat this source tree as proof that the historical deployment is currently
funded, unpaused, or claimable.

## Active Package

- `faucet-pool/SPEC.md` - contract rules and invariants.
- `faucet-pool/interface.json` - target call surface.
- `faucet-pool/pool-model.js` - executable reference model.
- `faucet-pool/pool-model.test.js` - invariant tests.
- `faucet-pool/conformance-vectors.json` - expected behavior vectors.
- `faucet-pool/DARKFI_PORTING_CHECKLIST.md` - required gates before testnet deployment.
- `faucet-pool/rust-skeleton/` - Rust-shaped FaucetPool source skeleton.
- `top-up-guard/` - conditional replenishment model for future operator tooling.

## Local Checks

From the repository root:

```bash
cargo run --locked --bin publication-safety-check
cd contracts/faucet-pool/rust-skeleton
cargo check
```

If Node.js is available, the JavaScript reference model can also be tested from
a contributor-maintained package setup.

## Deployment Boundary

Deployment must be performed only from a controlled DarkFi testnet environment
after the proof gates in `faucet-pool/DARKFI_PORTING_CHECKLIST.md` pass.

The public repository intentionally does not include:

- maintainer wallet files;
- raw transaction artifacts;
- WSL disks;
- local DarkFi configs;
- chain DBs;
- private logs;
- production secrets.

## Required Invariants

Any future DarkFi contract or on-chain pool must preserve these invariants:

- A wallet can withdraw at most `0.00003 DRK` per `12` hours.
- Total daily outflow cannot exceed `0.33 DRK`.
- The daily allowance resets to `0.33 DRK` each day.
- Unused daily allowance does not carry over.
- The contract must not expose private wallet material.
- The off-chain worker must never bypass the same amount limit.
- After proof passes, custody must live in the contract pool, not in a hosted
  process or private faucet wallet.

## Integration Gate

Generic DarkFi contract mechanics are not enough. FaucetPool requires proof of:

- contract build;
- deploy;
- top-up;
- resume;
- fee handling;
- address and TokenId handling;
- DRK withdrawal/claim to a controlled wallet;
- wallet receipt after scan;
- duplicate-claim rejection;
- pause/resume behavior;
- emergency withdrawal.

Until those commands are confirmed on testnet, any direct wallet-send path is a
temporary fallback only. The target production path is public request validation
plus contract-enforced custody, cooldown, and daily outflow.
