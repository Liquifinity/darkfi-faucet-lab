# DarkFi Porting Checklist

This checklist defines the minimum bar before the faucet pool can be considered
ready for DarkFi testnet deployment.

The current `pool-model.js` is the executable reference. The DarkFi Rust/WASM
contract must preserve the same invariants, then prove custody and transfer
behavior on-chain. The current Rust-shaped porting base is
`rust-skeleton/`.

Before porting or deploying, run:

```bash
npm run verify:contract-target
```

## Target Contract Boundary

Build one contract first:

- `FaucetPool`: custody, claims, daily pool, cooldown, pause/resume.

Do not combine points or DAO governance into the first contract. Those belong
to a later phase after the faucet pool has proven custody, withdrawal, and
replay resistance on testnet.

## Required Persistent State

- `admin_public_key`
- `token_id`
- `claim_amount_units`
- `daily_pool_units`
- `cooldown_seconds`
- `current_day`
- `spent_today_units`
- `paused`
- `last_claim_at[wallet_identity]`
- contract custody accounting for DRK

## Required Calls

- `init(config)`
- `top_up(amount_units, token_id)`
- `claim(recipient_address)`
- `pause()`
- `resume()`
- `emergency_withdraw(amount_units, recipient_address)`

Current safety status: `emergency_withdraw` is implemented in the Rust/WASM
path as an admin-signed Money flow, but the real DarkFi call generator and
testnet proof are still missing. A deploy with this skeleton is experimental
only and must not receive public custody.

## Required Invariants

- A valid claim transfers exactly `0.00003 DRK`.
- A wallet cannot claim again before `12` hours.
- All wallets share the same `0.33 DRK` daily pool.
- Daily reset restores allowance to exactly `0.33 DRK`; it does not carry over.
- Paused contracts reject claims.
- Claims fail if custody balance is insufficient.
- Public top-up increases custody balance only; it does not increase the daily
  pool, reset the daily pool, or bypass cooldown.
- Admin-only calls cannot be executed by public claimants.
- Failed local worker retries must not duplicate a broadcasted on-chain claim.

## DarkFi-Specific Proof Gates

Current operational status: deploy-ready preparation only. The DarkFi testnet
was reset and the preserved wallet keys do not currently have confirmed new
DRK, so do not run real deploy, top-up, or claim flows until balance is
confirmed.

Predeploy checks are intentionally limited to local readiness:

- `npm run verify:contract-target`
- WSL FaucetPool port/build
- no explicit `DEPLOY_BLOCKER` markers in the skeleton
- valid 32-byte hex `ADMIN_PUBLIC_KEY_HEX`
- valid 32-byte hex `CUSTODY_PUBLIC_KEY_HEX`
- valid 32-byte hex `DRK_TOKEN_ID_HEX`
- final WASM artifact exists
- final deploy ix artifact exists
- final deploy ix matches the current deploy env values

Before deploying publicly, prove each gate on testnet with command logs:

1. Build the contract WASM.
2. Deploy the contract through the DarkFi deploy flow.
3. Record the deployed contract id.
4. Top up the contract from the operator wallet with a tiny DRK amount.
5. Claim to a controlled wallet.
6. Scan the controlled wallet and verify received DRK.
7. Top up from a non-admin wallet and verify the custody balance increases.
8. Verify top-up does not increase the daily allowance.
9. Attempt duplicate claim before `12` hours and verify rejection.
10. Deplete a small configured daily pool and verify rejection.
11. Advance/reset the pool day in a controlled test and verify allowance reset.
12. Pause and verify claim rejection.
13. Resume and verify a valid claim succeeds.
14. Emergency withdraw remaining custody to admin-controlled address.

Gate 14 is mandatory for production. If `emergency_withdraw` has not been
called successfully through a real DarkFi call generator and proven on testnet,
do not top up public custody and do not enable Railway claims against the
contract.

Admin calls are also production-gated. `pause` and `resume` must verify a real
DarkFi admin signature/proof, not only compare a supplied public key to stored
configuration.

The production readiness check must also pass:

```bash
npm run verify:production-readiness
```

This check intentionally blocks while real DarkFi call generators for top-up,
claim, and emergency withdrawal are missing.

After an experimental deploy and controlled tests, create the local ignored
proof file:

```text
runtime/state/faucet-contract-proof.json
```

The file must use schema `darkfi-faucet-contract-proof-v1` and pass:

```bash
npm run verify:contract-deploy-proof
```

Use `contracts/faucet-pool/deploy-proof.example.json` as the shape reference.
Do not commit the real proof file because it belongs under ignored runtime
state and may contain operational transaction details.

This is intentionally separate from `verify:contract-target`: target validation
means the model, vectors, scripts, and WASM shape are coherent; deploy proof
means custody and rejection behavior were actually demonstrated.

When new DRK is confirmed, the operational sequence is:

1. Confirm faucet/operator wallet balance.
2. Generate deploy auth.
3. Run `scripts/prepare-faucet-contract-deploy.ps1` from Windows, or
   `scripts/prepare-faucet-contract-deploy-wsl.sh` directly in WSL, with real
   deploy envs.
4. Run experimental deploy.
5. Top up with a minimal amount.
6. Run one controlled claim.
7. Test rejection cases.
8. Generate `runtime/state/faucet-contract-proof.json`.
9. Run `npm run verify:contract-deploy-proof`.

## Implementation Notes

- Use integer base units only; no decimal math in contract code.
- Store wallet identity in the same form used by the claim authorization path.
- Follow the DarkFi `metadata -> exec -> apply` contract lifecycle.
- Use the DAO money-transfer authorization pattern as the closest reference for
  claim-controlled DRK movement.
- If DarkFi does not expose wall-clock timestamps safely to contracts, use
  block height plus configured target block time and document the conversion.
- Treat the off-chain app and worker as convenience layers. The contract must
  remain the final authority for custody, cooldown, and daily pool.
- Keep the current private-wallet worker active until every proof gate passes.
- Run any automatic top-up as conditional replenishment only. It should check
  contract balance first and send funds only if the pool is below target.

See `docs/faucet-pool-darkfi-blueprint.md` for the current Rust/WASM
implementation blueprint.
