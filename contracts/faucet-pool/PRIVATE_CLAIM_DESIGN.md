# Private Claim Design

This is the implementation target for replacing the experimental claim verifier.

## Current Problem

The previous FaucetPool claim path proved the outgoing Money output by passing
`MoneyOutputAuth` in the FaucetPool call payload. That exposed recipient output
preimage data and was not production-safe.

## Upstream References

Use these DarkFi references from a contributor-controlled DarkFi source
checkout before adding any custom proof shape:

- `src/contract/dao/src/client/auth_xfer.rs`
- `src/contract/dao/src/entrypoint/auth_xfer.rs`
- `src/contract/money/src/client/transfer_v1/builder.rs`
- `src/contract/money/src/client/transfer_v1/proof.rs`
- `src/contract/money/src/entrypoint/transfer_v1.rs`

The DAO `AuthMoneyTransfer` path is the primary reference because it verifies
Money transfer authorization without asking the caller to reveal the output
coin blind in the parent contract payload.

## Required Shape

The production claim should split responsibilities:

- Money handles anonymous input spend and output creation.
- FaucetPool verifies claim policy: amount, token, cooldown, daily pool,
  custody accounting, pause state.
- A FaucetPool authorization proof binds the Money transfer to the accepted
  claim without exposing recipient output preimage data.

The claim payload may include:

- recipient identity commitment used for cooldown;
- requested amount units;
- token id;
- child Money transfer call index;
- references/commitments required by the authorization proof.

The claim payload must not include:

- recipient address;
- recipient output public key;
- output coin blind;
- `recipient_output: MoneyOutputAuth`;
- any wallet secret, private key, seed, or decrypted address secret.

## Acceptance Criteria

`npm run verify:claim-privacy-target` can pass only after:

- `ClaimParams` no longer contains `recipient_output: MoneyOutputAuth`;
- `ClaimParams` no longer contains a recipient address;
- `MoneyOutputAuth` is not used to expose recipient public key or coin blind
  for claim verification;
- `verify_private_claim_authorization` contains the real DarkFi verification
  path and does not fail closed unconditionally;
- claim verification still checks exact amount and token;
- change output remains locked back to FaucetPool custody;
- a real `generate-faucet-claim-call-wsl.sh` creates a transaction that passes
  `inspect`, `broadcast`, `spend`, `scan`, and controlled wallet balance checks.

## Implemented Partial Verifier

The current contract source performs safe pre-proof checks only:

- the declared child Money transfer index must match the actual child call;
- the authorization payload must be non-empty;
- all Money inputs must share the same `user_data_enc`;
- token, amount, and custody public key inputs must have sane shape.

It still fails closed until the final DarkFi-native proof binds the exact token,
claim amount, recipient payment, and custody change without exposing output
preimage data.

## Non-Goals

- Do not add points or DAO logic to this contract.
- Do not enable Railway payout until production proof exists.
- Do not replace the contract claim with direct wallet transfer.
