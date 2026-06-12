# Technical Research Summary

This is the public, sanitized summary of the DarkFi Faucet research work done
before opening the repository.

It captures conclusions that are useful to contributors without publishing
wallet databases, raw transaction artifacts, WSL disks, local configs, node
state, logs, or maintainer-specific evidence.

## Scope Researched

- Windows + WSL2 DarkFi testnet setup.
- `darkfid` node operation and sync behavior.
- `drk` wallet behavior, scan semantics, coins, pending transactions, and
  chain visibility.
- XMRig connection to DarkFi Stratum.
- FaucetPool contract model and Rust/WASM skeleton.
- Deploy/top-up/resume/claim control flow.
- Transaction inclusion and pending-state diagnostics.
- Public repository security boundary.

## Confirmed Architecture

The useful operational split is:

- `darkfid`: full node, validator/runtime, local RPC, P2P, Stratum endpoint.
- `drk`: wallet/CLI, local wallet DB, chain scan, transaction generation and
  inspection.
- XMRig: miner process connected to the local DarkFi Stratum endpoint.
- FaucetPool: intended on-chain custody and policy contract.
- Public app/API: request validation and queueing only; not custody.

## Main Finding

The hard problem was not simply "make a faucet page".

The full path crosses separate proof layers:

1. contract source compiles;
2. contract deploy confirms;
3. top-up confirms;
4. resume/unpause confirms;
5. claim transaction is constructed correctly;
6. claim passes local inspection;
7. transaction enters pending;
8. transaction is included in a canonical block;
9. wallet scan observes receipt;
10. duplicate claim is rejected.

The historical work reached several of these layers but did not produce a
public end-to-end claim proof.

## Historical On-Chain Work

The historical FaucetPool work produced a deployed contract, top-up, and
resume transaction on DarkFi testnet. These are recorded in
`docs/FAUCETPOOL_HISTORICAL_STATUS.md`.

Those facts are useful, but they do not prove that the historical pool is
currently funded, unpaused, and claimable.

## Claim Attempts

Several claim attempts validated locally or reached pending/broadcast states.
They were not promoted to public proof because the required end condition was
not satisfied:

- canonical transaction confirmation;
- recipient wallet receipt after scan;
- custody change verification;
- duplicate claim rejection.

The public repository therefore treats historical claims as research evidence,
not as successful faucet operation.

## Node And Network Lesson

`tx.pending` is not confirmation.

`txs-history` with `Broadcasted` is not confirmation.

Local template/proposal evidence is not canonical block inclusion.

The reliable proof boundary is a confirmed transaction that can be fetched from
the chain and reconciled by wallet scan.

## Wallet Lesson

DRK is the asset being distributed. OwnCoin is a local wallet representation of
a spendable coin for that asset. FaucetPool claims should use DRK OwnCoins
under custody to construct a verifiable Money transfer.

Wallet state must be reconciled against chain state before any new claim
attempt. Reusing stale local wallet state creates false diagnostics.

## Current Public Recommendation

Do not try to resume from private maintainer runtime state.

Use a clean-room cycle:

1. accessible runtime;
2. explicit test wallet;
3. fresh deploy;
4. minimal top-up;
5. resume/unpause;
6. controlled claim;
7. confirmed chain inclusion;
8. wallet receipt proof;
9. duplicate-claim rejection.

See `docs/CLEAN_ROOM_FAUCET_CYCLE.md`.

