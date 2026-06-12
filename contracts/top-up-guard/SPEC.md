# DarkFi Faucet TopUpGuard Contract Specification

`TopUpGuard` is an optional second contract for conditional replenishment of the
`FaucetPool`. It must not be deployed before `FaucetPool` has passed its
postdeploy proof gates.

## Purpose

The guard holds a limited DRK reserve and releases DRK only to the configured
`FaucetPool` contract when the pool is below a configured balance threshold.
It is not a scheduler. A local operator process still submits the transaction;
the contract only enforces whether that transaction is valid.

## Parameters

- `faucet_pool_contract_id`: the only contract that may receive replenishment.
- `token_id`: accepted DRK testnet token id.
- `min_pool_balance_units`: replenishment is rejected while FaucetPool custody is
  at or above this value.
- `target_pool_balance_units`: replenishment must not raise FaucetPool custody
  above this value.
- `daily_replenish_limit_units`: maximum DRK released by the guard per day.
- `admin_public_key`: authority allowed to pause/resume and emergency withdraw.

## Calls

- `deposit(amount_units, token_id)`: public reserve funding for the guard.
- `replenish(faucet_pool_state, amount_units)`: public trigger that transfers DRK
  from the guard to FaucetPool only when all rules pass.
- `pause()`: admin-only circuit breaker.
- `resume()`: admin-only.
- `emergency_withdraw(amount_units, recipient)`: admin-only, mandatory before
  production custody.

## Replenish Rules

A replenish call must be rejected unless:

- The guard is not paused.
- The destination contract id equals `faucet_pool_contract_id`.
- The token id equals `token_id`.
- FaucetPool reported custody is below `min_pool_balance_units`.
- `amount_units` is greater than zero.
- `faucet_pool_balance_units + amount_units <= target_pool_balance_units`.
- Daily released amount plus `amount_units` is within
  `daily_replenish_limit_units`.
- Guard reserve balance is sufficient.

On success:

- Decrease guard reserve by `amount_units`.
- Increase `released_today_units` by `amount_units`.
- Mint/transfer the output only to FaucetPool custody.

## Safety Boundary

This contract is only useful after FaucetPool can prove custody, claim,
top-up, rejection, pause/resume, and emergency withdrawal on testnet. Until
then, use local dry-run automation only.
