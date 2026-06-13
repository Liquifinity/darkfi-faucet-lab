# Roadmap

This roadmap keeps the project focused.

## Phase 1: Public Repository Foundation

Status: complete.

- Public repo created.
- Apache-2.0 license.
- Branch protection.
- Required safety check.
- Security boundary.
- Contribution process.
- FaucetPool source and model published.
- Historical findings sanitized.

## Phase 2: Clean-Room Faucet Proof

Status: next.

Goal: prove one complete FaucetPool cycle without private maintainer runtime
state.

Required:

1. fresh accessible DarkFi testnet environment;
2. explicit test wallet;
3. build FaucetPool;
4. deploy;
5. minimal top-up;
6. resume;
7. controlled claim;
8. confirmed chain inclusion;
9. wallet receipt;
10. duplicate-claim rejection.

Output:

- sanitized proof document;
- confirmed tx hashes;
- confirmed block heights;
- updated readiness status.

## Phase 3: Public Faucet Service Boundary

Status: blocked until Phase 2.

Goal: connect public request handling to the proven contract flow.

Rules:

- hosted app must not hold maintainer wallet material;
- worker must not bypass contract limits;
- request queue is not custody;
- contract remains final authority.

## Phase 4: Community Hardening

Status: open.

Needs:

- independent review of FaucetPool rules;
- independent reproduction of clean-room deploy;
- more DarkFi-native tests;
- clearer contract state visibility;
- duplicate/replay resistance review.

## Out Of Scope For Now

- DAO;
- production treasury;
- public custody funding;
- mining optimization;
- mainnet assumptions.

