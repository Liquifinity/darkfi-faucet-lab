# DarkFi Faucet Lab

This repository is the public coordination layer for a DarkFi testnet faucet.

It is intentionally Rust-first because DarkFi contracts, tooling, and the relevant builder workflow are Rust/WASM-oriented. The repository does not publish maintainer wallet state or historical private artifacts.

Current status:

- This is a public collaboration repository.
- It does not contain maintainer wallet material, VHDX backups, raw transaction artifacts, or private chain state.
- The historical FaucetPool work is treated as private evidence, not as a public claimable faucet.
- The recommended next path is a clean-room faucet cycle: accessible runtime, explicit wallet, deploy, top-up, resume, claim, confirmation, and duplicate-claim rejection.

Do not commit wallet databases, private keys, raw VM disks, secrets, local chain state, or raw transaction artifacts.

## Repository Scope

Intended public content:

- Rust safety tooling;
- FaucetPool contract model and Rust/WASM skeleton;
- clean-room faucet design notes;
- contributor and maintainer security process;
- reproducible public documentation;
- non-custodial testnet coordination.

Excluded from Git:

- `backups/`;
- `evidence/`;
- WSL VHDX files;
- wallet DBs;
- config files containing wallet passwords;
- raw transaction artifacts;
- local logs and state.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

See [SECURITY.md](SECURITY.md).

Public collaboration model:

- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Engineering quality bar](docs/ENGINEERING_QUALITY_BAR.md)
- [FaucetPool historical status](docs/FAUCETPOOL_HISTORICAL_STATUS.md)
- [Clean-room faucet cycle](docs/CLEAN_ROOM_FAUCET_CYCLE.md)
- [Public content inventory](docs/PUBLIC_CONTENT_INVENTORY.md)
- [Technical research summary](docs/TECHNICAL_RESEARCH_SUMMARY.md)
- [Evidence and decision index](docs/EVIDENCE_AND_DECISION_INDEX.md)
- [Node and wallet operation notes](docs/NODE_WALLET_OPERATION_NOTES.md)
- [Open engineering questions](docs/OPEN_ENGINEERING_QUESTIONS.md)
- [Pull request security model](docs/PR_SECURITY_MODEL.md)
- [Public access model](docs/PUBLIC_ACCESS_MODEL.md)
- [Public release status](docs/PUBLIC_RELEASE_STATUS.md)
- [Safe contributor quickstart](docs/SAFE_CONTRIBUTOR_QUICKSTART.md)
- [Security boundary](docs/SECURITY_BOUNDARY.md)
- [Threat model](docs/THREAT_MODEL.md)
- [Maintainer security operations](docs/MAINTAINER_SECURITY_OPERATIONS.md)
- [Repository security audit](docs/REPOSITORY_SECURITY_AUDIT.md)
- [GitHub admin checklist](docs/GITHUB_ADMIN_CHECKLIST.md)
- [Remote repository setup](docs/REMOTE_REPOSITORY_SETUP.md)
- [License decision](docs/LICENSE_DECISION.md)

Contract work:

- [FaucetPool specification](contracts/faucet-pool/SPEC.md)
- [DarkFi porting checklist](contracts/faucet-pool/DARKFI_PORTING_CHECKLIST.md)
- [FaucetPool Rust/WASM skeleton](contracts/faucet-pool/rust-skeleton)

## Safety Baseline

Run before publishing changes:

```bash
cargo run --locked --bin publication-safety-check
```

## License

Licensed under the Apache License 2.0. See [LICENSE](LICENSE).
