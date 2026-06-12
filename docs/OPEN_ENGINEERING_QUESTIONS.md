# Open Engineering Questions

These are the remaining technical questions for contributors.

## FaucetPool Claim Proof

Can the FaucetPool Rust/WASM path produce a claim transaction that:

- passes `tx-from-calls`;
- passes `inspect`;
- confirms on-chain;
- transfers exactly `0.00003 DRK`;
- returns coherent custody change;
- rejects duplicate claims inside `12` hours?

## DarkFi-Native Authorization

The private claim authorization path must be completed with DarkFi-native proof
verification. It must not be replaced by direct private wallet transfers.

Relevant files:

- `contracts/faucet-pool/PRIVATE_CLAIM_DESIGN.md`
- `contracts/faucet-pool/AUTH_MONEY_TRANSFER_PORT.md`
- `contracts/faucet-pool/rust-skeleton/src/money.rs`
- `contracts/faucet-pool/rust-skeleton/proof/faucet-claim.zk`

## Contract State Visibility

What is the cleanest public method to read or prove current FaucetPool custody,
pause state, daily pool state, and cooldown state from testnet?

The public proof should not depend on maintainer wallet DBs or private logs.

## Clean-Room Deployment

What is the smallest reproducible deployment sequence that a contributor can
run from a fresh DarkFi testnet environment?

The sequence must record:

- DarkFi commit/version;
- contract id;
- deploy/top-up/resume/claim tx hashes;
- confirmed heights;
- wallet receipt;
- duplicate rejection.

## Off-Chain Boundary

What data should a public API collect without weakening privacy?

The public API should validate and queue requests, but it should not hold
maintainer custody, wallet secrets, raw chain state, or private operator
configs.

## Production Readiness

The faucet is not production-ready until:

- on-chain claim success is proven;
- duplicate rejection is proven;
- top-up behavior is proven;
- pause/resume is proven;
- emergency withdrawal is proven;
- public docs explain how to reproduce proof without private maintainer state.

