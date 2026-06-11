# DarkFi Faucet Lab

This repository contains operational runbooks, audits, scripts, and decision records for building and validating a DarkFi testnet faucet.

Current status:

- The historical FaucetPool deployment is preserved as evidence.
- The old WSL runtime disk has been backed up for forensics.
- The recommended next path is a clean faucet cycle: accessible runtime, explicit wallet, deploy, top-up, resume, claim, confirmation, and duplicate-claim rejection.

Do not commit wallet databases, private keys, raw VM disks, secrets, local chain state, or raw transaction artifacts.

## Repository Scope

Intended public content:

- runbooks;
- non-secret scripts;
- audit summaries;
- architecture notes;
- contribution process;
- security process.

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

- [Public access model](docs/PUBLIC_ACCESS_MODEL.md)
- [Security boundary](docs/SECURITY_BOUNDARY.md)
- [Threat model](docs/THREAT_MODEL.md)
- [GitHub admin checklist](docs/GITHUB_ADMIN_CHECKLIST.md)
- [Remote repository setup](docs/REMOTE_REPOSITORY_SETUP.md)
- [License decision](docs/LICENSE_DECISION.md)

## License

Licensed under the Apache License 2.0. See [LICENSE](LICENSE).
