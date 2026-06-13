# Public Release Status

## Current State

This repository is safe for public collaboration on documentation, process, and future clean-room faucet work.

It is not proof that the historical FaucetPool is claimable.

The latest DarkFi hardening reset reinforces this: deployed contracts must be
redeployed for current testnet proof.

## Proven

- The public repository excludes local wallet/state/backups/evidence by default.
- Apache-2.0 license is selected.
- Branch protection is configured for `main`.
- Secret scanning and push protection are enabled on GitHub.
- Public contribution and security process exists.

## Not Proven

- Historical FaucetPool claim success.
- Current custody state of the old FaucetPool.
- Current wallet readiness from the old WSL runtime.
- Production faucet payout flow.

## Recommended Next Engineering Track

Use a clean-room cycle:

1. Fresh accessible runtime.
2. Updated DarkFi node and wallet tooling.
3. Reset node database artifacts according to official guidance.
4. Explicit test wallet.
5. Minimal deploy.
6. Minimal top-up.
7. Resume/unpause.
8. Controlled claim.
9. Confirmed chain inclusion.
10. Wallet receipt proof.
11. Duplicate-claim rejection.

Do not rely on private maintainer state for community reproduction.
