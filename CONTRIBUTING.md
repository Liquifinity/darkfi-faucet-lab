# Contributing

Thanks for helping improve this DarkFi faucet work.

## Ground Rules

- Do not commit secrets, wallet databases, seed phrases, private keys, WSL disks, raw chain state, or local configs.
- Do not submit changes that weaken validation, authentication, rate limits, encryption, or secret handling.
- Keep changes small and reviewable.
- Prefer reproducible commands and evidence over screenshots.
- Separate operational reports from code changes.

## Pull Request Checklist

Before opening a PR:

- Confirm no sensitive files are included.
- Run the relevant script or validation command.
- Document what was tested.
- Link any issue or report the PR addresses.
- Explain whether the change touches wallet, node, chain DB, contract, faucet claim logic, or deployment.
- Run the local checks before requesting review:

```bash
cargo fmt --check
cargo run --locked --bin publication-safety-check
cargo check --manifest-path contracts/faucet-pool/rust-skeleton/Cargo.toml
node --test contracts/faucet-pool/pool-model.test.js
node contracts/faucet-pool/verify-contract-target.js
node --test contracts/top-up-guard/guard-model.test.js
```

## PR Safety Gates

Public contributors may open pull requests. They should not receive direct
write access to `main`.

The `main` branch must stay protected with:

- required `safety-check` status check;
- force pushes disabled;
- branch deletion disabled;
- admins included in protection.

Maintainers may merge only after the required check passes. If a PR touches
wallet, node, chain DB, deployment, broadcast, contract custody, or security
limits, treat it as security-sensitive even if CI passes.

## Areas That Need Help

- DarkFi contract validation.
- FaucetPool claim proof workflow.
- P2P/relay inclusion diagnostics.
- Wallet/coin selection safety.
- Documentation cleanup.
- Testnet runbooks.

## What Not To Do Without Maintainer Approval

- Redeploy contracts.
- Broadcast transactions.
- Modify wallet DBs or chain DBs.
- Change faucet security limits.
- Change production/Railway settings.
- Add dependencies or external services.
- Move or delete evidence/backups.

## First-Time Contributor Path

Start with:

- [Safe contributor quickstart](docs/SAFE_CONTRIBUTOR_QUICKSTART.md)
- [Security boundary](docs/SECURITY_BOUNDARY.md)
- [Public release status](docs/PUBLIC_RELEASE_STATUS.md)
