# Public Release Status

## Current State

This repository is safe for public collaboration on documentation, process, and future clean-room faucet work.

It is not proof that the historical FaucetPool is claimable.

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
2. Explicit test wallet.
3. Minimal deploy.
4. Minimal top-up.
5. Resume/unpause.
6. Controlled claim.
7. Confirmed chain inclusion.
8. Wallet receipt proof.
9. Duplicate-claim rejection.

Do not rely on private maintainer state for community reproduction.
