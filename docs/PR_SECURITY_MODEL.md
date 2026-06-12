# Pull Request Security Model

This repository is public. Public visibility means anyone can propose changes
through pull requests. It must not mean anyone can change protected code
directly.

## Required Boundary

Public users:

- may fork;
- may open issues;
- may open pull requests;
- may run local checks;
- must not have direct write access to `main`.

Maintainers:

- review changes;
- keep branch protection enabled;
- merge only after required checks pass;
- do not bypass checks for convenience.

## Required Branch Protection

The `main` branch should require:

- status check: `safety-check`;
- strict branch up-to-date check;
- force pushes disabled;
- branch deletion disabled;
- admin enforcement enabled.

The review requirement may be enabled later when there is more than one
maintainer. Until then, the required CI gate is the minimum non-negotiable
protection.

## Required CI Coverage

The required `safety-check` workflow must validate:

- no private operational files are tracked;
- no obvious secrets are tracked;
- Rust formatting;
- publication safety checker;
- FaucetPool Rust skeleton builds;
- FaucetPool model tests;
- FaucetPool target verification;
- top-up guard model tests.

## Security-Sensitive PRs

Treat a PR as security-sensitive if it changes:

- contract custody logic;
- claim generation;
- wallet, node, chain DB, broadcast, or deploy behavior;
- rate limits or daily pool limits;
- secret handling;
- GitHub Actions permissions;
- branch protection docs or safety tooling.

Security-sensitive PRs need explicit maintainer review even if the automated
check passes.

## What CI Does Not Prove

CI does not prove that:

- a contract is deployed;
- a faucet is funded;
- a claim is confirmed on-chain;
- wallet receipt happened;
- production is safe.

Those require testnet evidence documented through the clean-room faucet cycle.
