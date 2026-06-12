# FaucetPool Auth Money Transfer Port

This file records the concrete DarkFi pattern that must be ported before
FaucetPool claim payouts can be production enabled.

## Upstream Pattern

Primary references:

- `src/contract/dao/src/client/auth_xfer.rs`
- `src/contract/dao/src/entrypoint/auth_xfer.rs`
- `src/contract/dao/src/model.rs`
- `src/contract/money/src/client/transfer_v1/builder.rs`
- `src/contract/money/src/client/transfer_v1/proof.rs`
- `src/contract/money/src/entrypoint/transfer_v1.rs`

DAO uses `Dao::AuthMoneyTransfer` to authorize a native
`Money::TransferV1` without putting recipient output preimages in the parent
payload. The verifier has two distinct parts:

- get_metadata emits ZK public inputs for
  `AuthMoneyTransferEncCoin` and `AuthMoneyTransfer`;
- `process_instruction` checks call topology, Money contract/function,
  matching `user_data_enc`, expected output coins, and custody/change shape.

DAO proof payloads are not raw wallet secrets. `DaoAuthMoneyTransferParams`
contains encrypted note attributes:

- `enc_attrs`;
- `dao_change_attrs`.

The transaction carries the proofs. Metadata tells the validator which public
inputs must be verified for the call.

## FaucetPool Port Target

FaucetPool claim uses Faucet-native authorization metadata with the same
boundary:

- Money handles anonymous spend and recipient output creation;
- FaucetPool verifies policy: token, amount, cooldown, daily pool, pause state,
  and custody accounting;
- FaucetPool emits ZK public inputs for a claim authorization proof;
- FaucetPool process validates call topology and exact custody/change rules;
- recipient address, output public key, and output coin blind remain outside
  the FaucetPool payload.

The current transaction topology is parent `FaucetPool::Claim` with a child
`Money::TransferV1`. If the final port keeps this topology, it must use the
declared child `transfer_call_index` and `children_indexes` instead of DAO's
immediate sibling assumption.

## Production Evidence

Production proof must keep evidence that all items are true:

- FaucetPool has zkas namespaces for claim authorization metadata;
- `darkfi_get_metadata` emits non-empty claim ZK public inputs;
- `verify_private_claim_authorization` no longer fails closed
  unconditionally;
- the claim generator builds a real DarkFi transaction with attached proofs;
- deploy proof includes top-up, claim, pause/resume, rejection cases, scan, and
  balance evidence.
