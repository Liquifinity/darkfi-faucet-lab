# Node And Wallet Operation Notes

These notes summarize operational findings that matter for contributors working
with DarkFi testnet.

They are intentionally generic. They do not include local wallet paths,
passwords, WSL storage paths, raw logs, raw transactions, or maintainer machine
state.

## Process Roles

- `darkfid` owns node sync, RPC, P2P, runtime validation, and Stratum.
- `drk` owns wallet operations, scan, coin visibility, inspect, broadcast, and
  transaction history.
- XMRig connects to the local Stratum endpoint; it does not prove canonical
  block inclusion by itself.

## Confirmation Rules

Treat these as non-final:

- transaction generated;
- `inspect` passed;
- `broadcast` returned a hash;
- `tx.pending` contains the transaction;
- wallet history says `Broadcasted`;
- local logs show template/proposal execution.

Treat this as the useful proof boundary:

- the transaction can be fetched from the chain;
- it has a confirmed block;
- wallet scan reconciles the expected coin effects.

## Pending Transactions

While a transaction is still pending:

- do not generate a replacement transaction from the same intended inputs;
- do not run broad `unspend`;
- do not treat `Broadcasted` as success;
- monitor chain height, pending state, fetch result, and wallet scan.

If a transaction leaves pending and cannot be fetched from the chain, recover
only after identifying the specific affected local coins. Do not perform global
wallet cleanup.

## Sync Health

Before claim or deploy work:

- compare local node height to a second node or public reference if available;
- confirm `tx.pending` is queryable;
- confirm wallet scan reaches the node's confirmed height;
- confirm the node is not stalled behind the current chain;
- confirm no stale local pending transaction remains.

## Mining Notes

Mining activity is not the same as confirmed inclusion.

Useful signals:

- Stratum connected to local `darkfid`;
- accepted local DarkFi work, not unrelated donate-pool activity;
- canonical block inclusion after proposal;
- node remains synced after mining activity.

## Wallet Notes

DRK is the distributed asset. OwnCoin is the local wallet representation of a
coin of that asset.

Before a claim attempt, the wallet must have:

- a known spendable fee coin;
- known custody coin state if the flow uses local custody;
- no stale spent state from abandoned transactions;
- scan height aligned with node confirmed height.

## Contributor Guidance

For public reproduction, use a fresh test wallet and record sanitized state
transitions. Do not reuse maintainer wallet state or local runtime artifacts.

