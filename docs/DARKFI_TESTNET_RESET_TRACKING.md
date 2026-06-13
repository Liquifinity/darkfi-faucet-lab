# DarkFi Testnet Reset Tracking

This project follows DarkFi testnet resets as protocol-boundary events.

## Latest Official Reset

- Source: `https://dark.fi/insights/testnet-v3a-r1-reset.html`
- Published: 2026-06-12
- Name in post: DarkFi Testnet `v0.3-r1` / `v0.3-r2` Alpha Hardening Reset.

## Relevant Fix Areas

The reset addresses protocol-level issues around:

- Sparse Merkle Tree nullifier absence proof constraints;
- fork proposal timestamp bounds;
- DAO money transfer authorization binding;
- DAO proposal input snapshot consistency;
- Halo2 engine upgrade after recent Zcash/Orchard-related findings.

## Operator Impact

The official guidance says node operators should:

- remove previous node database artifacts;
- update nodes;
- let `drk` reset to the new block sequence while preserving generated keys;
- redeploy deployed contracts.

## Project Impact

For this repository:

- historical FaucetPool deploy/top-up/resume are research history only;
- historical deployed contracts must not be treated as live or claimable;
- the clean-room faucet cycle is mandatory for the next proof;
- any future proof must record the DarkFi version/commit used after this reset.

## Do Not Do

- Do not resume from historical contract state.
- Do not claim the old FaucetPool is usable.
- Do not publish private wallet or chain state to prove reset recovery.
- Do not mix pre-reset and post-reset evidence.

