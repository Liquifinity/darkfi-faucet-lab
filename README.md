# DarkFi Faucet Lab

This repository contains operational runbooks, audits, scripts, and decision records for building and validating a DarkFi testnet faucet.

Current status:

- This is a public collaboration repository.
- It does not contain maintainer wallet material, VHDX backups, raw transaction artifacts, or private chain state.
- The historical FaucetPool work is treated as private evidence, not as a public claimable faucet.
- The recommended next path is a clean-room faucet cycle: accessible runtime, explicit wallet, deploy, top-up, resume, claim, confirmation, and duplicate-claim rejection.

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
- [Public release status](docs/PUBLIC_RELEASE_STATUS.md)
- [Safe contributor quickstart](docs/SAFE_CONTRIBUTOR_QUICKSTART.md)
- [Security boundary](docs/SECURITY_BOUNDARY.md)
- [Threat model](docs/THREAT_MODEL.md)
- [Maintainer security operations](docs/MAINTAINER_SECURITY_OPERATIONS.md)
- [Repository security audit](docs/REPOSITORY_SECURITY_AUDIT.md)
- [GitHub admin checklist](docs/GITHUB_ADMIN_CHECKLIST.md)
- [Remote repository setup](docs/REMOTE_REPOSITORY_SETUP.md)
- [License decision](docs/LICENSE_DECISION.md)

## Safety Baseline

Run before publishing changes:

```powershell
.\scripts\publication-safety-check.ps1
```

## License

Licensed under the Apache License 2.0. See [LICENSE](LICENSE).
