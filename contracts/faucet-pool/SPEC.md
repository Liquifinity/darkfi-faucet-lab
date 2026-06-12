# DarkFi Faucet Pool Contract Specification

This is the target behavior for the on-chain faucet pool. The current app and
worker must remain compatible with these rules while the DarkFi contract is
implemented and verified.

## Parameters

- Token: DRK testnet token id configured at deploy time.
- Custody public key: faucet-controlled Money public key configured at deploy
  time.
- Claim amount: `0.00003 DRK`.
- Wallet cooldown: `12 hours`.
- Daily pool allowance: `0.33 DRK`.
- Daily reset: calendar day boundary in the configured pool timezone.
- Carry-over: disabled. Unused allowance expires at the next daily reset.

Amounts must be represented as integer base units in contract code. The
decimal strings above are operator-facing values only.

## Pool Model

The contract is a DRK custody pool. Its custody balance and its daily allowance
are separate concepts:

- Custody balance is the DRK currently held by the contract.
- Daily allowance is the maximum DRK the contract may release during the active
  pool day.

Anyone may top up the custody balance with the configured DRK token. Top-ups do
not increase the daily allowance and do not bypass wallet cooldowns.

## Required State

The contract needs persistent state equivalent to:

- `admin`: authority allowed to pause, resume, and change operational metadata.
- `custody_public_key`: public key that must receive pool top-up and change
  outputs.
- `token_id`: accepted DRK token id.
- `claim_amount_units`: amount sent per successful claim.
- `cooldown_seconds`: minimum interval between claims for the same wallet.
- `daily_pool_units`: maximum amount claimable per pool day.
- `current_day`: active pool day id.
- `spent_today_units`: amount already claimed during `current_day`.
- `last_claim_at`: map from wallet identity to last accepted claim timestamp.
- `paused`: global circuit breaker.

## Claim Rules

A claim must be rejected unless all conditions hold:

- The contract is not paused.
- The recipient identity commitment is present.
- The requested token id matches `token_id`.
- The requested amount equals `claim_amount_units`.
- The recipient has not claimed within the last `cooldown_seconds`.
- `spent_today_units + claim_amount_units <= daily_pool_units`.
- The contract has enough available DRK custody balance.

On success:

- Transfer exactly `claim_amount_units` to the recipient.
- Verify the expected Money transfer through a private authorization proof.
- Require any change output to return to `custody_public_key` with the faucet
  claim spend hook.
- Set `last_claim_at[recipient_identity_commitment]` to the claim timestamp.
- Increase `spent_today_units` by `claim_amount_units`.
- Emit or expose enough transaction data for the off-chain API to mark the
  request as sent.

Privacy gate: the claim payload must not receive the recipient address or output
preimage (`public_key`, token id, amount, and `coin_blind`). Production claim
deployment must use a DarkFi-native authorization/proof pattern before Railway
payouts are enabled.

## Top-Up Rules

The `top_up` path should be public:

- Any wallet may contribute DRK testnet to the pool.
- The token id must match the configured DRK testnet token.
- The top-up increases custody balance only.
- The top-up Money output must be addressed to `custody_public_key` and locked
  with the faucet claim spend hook.
- The top-up must not change `daily_pool_units`.
- The top-up must not reset `spent_today_units`.
- The top-up must not alter any wallet cooldown.

The operator may run a daily conditional top-up helper, but it must only submit
funds when the contract custody balance is below the configured target. It must
not blindly send DRK every day.

## Daily Reset

At the first claim or admin action observed after a day boundary:

- Set `current_day` to the new day id.
- Set `spent_today_units` to `0`.
- Do not carry unused allowance forward.

Example: if only `0.15 DRK` was claimed yesterday, today's allowance is still
exactly `0.33 DRK`, not `0.51 DRK`.

## Admin Rules

The admin path must support:

- Pause and resume.
- Optional emergency withdrawal to admin.
- Optional parameter rotation only under strict bounds.

The Rust/WASM path implements `emergency_withdraw` as an admin-signed Money
flow that decreases custody after verification. Public custody must not be
topped up until the real DarkFi call generator exists and the path is proven on
testnet.

Parameter changes must not allow bypassing already-enforced limits for the
current day unless explicitly designed and audited.

## Off-Chain Compatibility

The public app should keep its current queue and rate-limit checks even after
the pool goes on-chain. Those checks become a first layer of abuse resistance;
the contract remains the final authority for custody and withdrawal limits.

The worker should move from "send from private wallet" to "submit claim/withdraw
call to contract" only after the following are proven on testnet:

- Build the faucet contract.
- Deploy the faucet contract.
- Top up contract custody with DRK.
- Prove public top-up does not increase the `0.33 DRK` daily allowance.
- Execute a successful claim to a controlled wallet.
- Reject duplicate wallet claim inside `12 hours`.
- Reject claims after `0.33 DRK` is consumed for the day.
- Reset allowance to exactly `0.33 DRK` on the next day.
- Pause blocks claims.
- Resume allows valid claims.
- Failure states do not leak wallet secrets or allow double-send retries.
