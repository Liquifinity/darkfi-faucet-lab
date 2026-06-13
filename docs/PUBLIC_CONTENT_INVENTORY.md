# Public Content Inventory

This repository does not publish the maintainer machine. It publishes the
parts of the DarkFi faucet work that are useful and safe for community review.

## Included

- FaucetPool specification.
- FaucetPool executable model and conformance vectors.
- FaucetPool Rust/WASM skeleton.
- Top-up guard model.
- Historical on-chain status summary.
- Clean-room reproduction plan.
- Architecture.
- Roadmap.
- Engineering quality bar.
- Technical research summary.
- Node/wallet operation notes.
- Evidence and decision index.
- Open engineering questions.
- Security boundary and threat model.
- Contribution process and branch protection policy.
- Rust publication safety checker.

## Excluded

- wallet databases;
- private keys and seed material;
- WSL VHDX backups;
- local chain DBs;
- raw transaction artifacts;
- local runtime configs;
- machine logs;
- private evidence folders;
- GitHub tokens and maintainer credentials.

## Why Historical Evidence Is Sanitized

The project produced useful operational reports while testing DarkFi node,
wallet, mining, relay, and FaucetPool behavior. Those reports contain paths,
runtime state, wallet metadata, raw outputs, and machine-specific context.

Publishing them directly would make the repository harder to use and could
weaken the security boundary. The public form is therefore a curated set of
status documents, source files, and reproducible checklists.

## Current Public Truth

The historical FaucetPool was deployed, topped up, and resumed on testnet, but
it is not published here as a currently claimable faucet.

Community work should proceed from the clean-room cycle.
