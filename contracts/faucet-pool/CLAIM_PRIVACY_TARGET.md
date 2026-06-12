# Claim Privacy Target

This file defines the minimum production bar for FaucetPool claims.

The previous experimental claim path was not production-safe because the faucet
call payload contained a Money output preimage:

- recipient public key
- token id
- amount
- coin blind

That design could prove the output was correct, but it exposed data that must
stay private. The current target payload removes those fields and stays blocked
until `verify_private_claim_authorization` contains a real DarkFi-native proof.

## Production Requirements

A production claim path must satisfy all requirements below:

- The faucet call payload must not contain `recipient_output: MoneyOutputAuth`.
- The faucet call payload must not contain `recipient_address`.
- The faucet call payload must not contain an output `public_key`.
- The faucet call payload must not contain an output `coin_blind`.
- The contract must still enforce exact `claim_amount_units`.
- The contract must still enforce the configured DRK `token_id`.
- The contract must still enforce the recipient payment through a
  privacy-preserving proof or equivalent DarkFi-native authorization.
- Any change output must still return to `custody_public_key` with the
  FaucetPool claim spend hook.
- The worker must submit contract claims, not direct wallet transfers.
- The production proof must include a successful claim and the rejection cases
  listed in `DARKFI_PORTING_CHECKLIST.md`.

## Accepted Implementation Direction

Use a DarkFi-native proof/authorization pattern that lets FaucetPool verify the
claim payment without learning the output secret preimage. The DAO transfer
authorization path is the closest upstream reference and should be used before
inventing a custom proof shape.

## Release Gate

`npm run verify:claim-privacy-target` must pass before creating
`runtime/state/faucet-contract-proof.json` or enabling contract payouts.
