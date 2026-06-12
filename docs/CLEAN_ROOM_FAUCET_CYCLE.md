# Clean-Room Faucet Cycle

This is the recommended next engineering track for contributors.

The goal is not to reuse maintainer machine state. The goal is to reproduce a
minimal FaucetPool deployment and prove the full claim path with public,
sanitized evidence.

## Scope

In scope:

- build FaucetPool source;
- deploy on DarkFi testnet;
- top up with a minimal DRK amount;
- resume/unpause;
- execute one controlled claim;
- prove chain inclusion;
- prove wallet receipt;
- prove duplicate claim rejection.

Out of scope:

- DAO;
- production hosted payout;
- maintainer wallet recovery;
- historical WSL recovery;
- mining optimization;
- public custody funding before proof.

## Required Evidence

For each step, record:

- command used;
- DarkFi commit/version;
- contract id;
- transaction hash;
- confirmed block height;
- `fetch-tx` result;
- wallet scan result;
- balance/coin state before and after, with secrets removed.

Do not publish raw wallet DBs, private keys, seed phrases, raw transaction
files, local config files, or chain DBs.

## Stop Conditions

Stop immediately if:

- `inspect` fails;
- `fetch-tx` cannot find a transaction that left pending;
- wallet state diverges from chain state;
- the contract accepts duplicate claims inside the cooldown window;
- the contract releases more than the configured daily pool;
- custody change does not return to the expected contract custody key.

## Success Criteria

A FaucetPool cycle is successful only when all are true:

- deploy confirmed;
- top-up confirmed;
- resume confirmed;
- claim confirmed;
- recipient wallet receives exactly the configured claim amount;
- custody change is coherent;
- duplicate claim before `12` hours is rejected;
- no maintainer private material is required to reproduce the test.

## Contributor Entry Point

Start with:

```bash
cargo run --locked --bin publication-safety-check
cd contracts/faucet-pool/rust-skeleton
cargo check
```

Then review:

- `contracts/faucet-pool/SPEC.md`
- `contracts/faucet-pool/DARKFI_PORTING_CHECKLIST.md`
- `contracts/faucet-pool/PRIVATE_CLAIM_DESIGN.md`
- `docs/FAUCETPOOL_HISTORICAL_STATUS.md`
