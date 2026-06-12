# FaucetPool Rust/WASM Skeleton

This is the first Rust-shaped porting target for the DarkFi FaucetPool contract.
It is intentionally kept outside the upstream DarkFi tree until deploy, top-up,
claim, and emergency withdrawal are proven on testnet.

Status:

- Buildable locally and through the WSL DarkFi port/build helper.
- Not publicly deploy-ready until real deploy credentials and end-to-end calls
  are tested.
- Intended to be copied/adapted into the DarkFi contract workspace.
- Limited to faucet pool behavior only. Points and DAO logic are excluded.

## Target Behavior

- Public `top_up`: accepts only the configured DRK testnet token and increases
  contract custody accounting.
- Public `claim`: sends exactly `0.00003 DRK` when cooldown, daily cap, pause,
  and custody checks pass.
- Admin `pause` and `resume`.
- Admin `emergency_withdraw`.
- Daily release cap: `0.33 DRK`.
- Wallet cooldown: `12 hours`.
- No carry-over between days.

## Porting Requirements

Before this can be deployed:

1. Move this skeleton into the DarkFi contract workspace.
2. Generate a deploy ix with real admin, custody, and DRK token ids.
3. Build a client-side top-up/claim call generator that supplies
   `MoneyOutputAuth` for the expected Money outputs.
4. Prove contract-owned custody identity in the DarkFi test harness or testnet.
5. Run the conformance vectors from `../conformance-vectors.json`.
6. Deploy through Deployooor only after harness tests pass.

The local crate is dependency-free so `cargo check` can validate the skeleton
shape before it is moved into the DarkFi workspace.

From this crate:

```bash
cargo check
```

DarkFi workspace integration must be performed in a contributor-controlled
DarkFi checkout. Do not copy maintainer wallet paths, runtime state, raw
transactions, or private configs into public examples.
