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
