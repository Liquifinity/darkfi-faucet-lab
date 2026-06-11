# Security Policy

## Sensitive Material

Never commit:

- wallet databases;
- seed phrases;
- private keys;
- secret keys;
- wallet passwords;
- `.toml` configs containing wallet credentials;
- WSL `ext4.vhdx` or VM backups;
- raw transaction artifacts;
- local chain DB/state;
- Railway or deployment secrets;
- admin tokens.

## Reporting a Vulnerability

Open a private maintainer contact path before public disclosure. If no private channel exists yet, open a GitHub issue with minimal detail and no secrets, asking maintainers to establish a private channel.

Do not publish exploit details, private keys, wallet paths, logs containing credentials, or raw wallet artifacts.

## Maintainer Response

Expected handling:

1. Confirm receipt.
2. Reproduce safely.
3. Classify impact.
4. Patch or document mitigation.
5. Publish a postmortem only after sensitive material is removed.

## Required Checks Before Public Release

- Secret scan passes.
- `.gitignore` excludes local state and backups.
- No VHDX, wallet DB, raw tx artifacts, or local configs are tracked.
- Faucet claim flow is either proven or clearly marked experimental.
- License is selected and committed.
