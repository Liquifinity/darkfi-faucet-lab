# Open Source Release Checklist

## Blocking Items

- [ ] Choose and commit a license.
- [ ] Confirm no secrets are tracked.
- [ ] Confirm no VHDX, wallet DB, chain DB, raw tx, or local config is tracked.
- [ ] Decide whether this repository is for runbooks only or also contract/source code.
- [ ] Decide whether historical evidence should remain private.
- [ ] Configure GitHub branch protection.
- [ ] Enable GitHub secret scanning and push protection.
- [ ] Add maintainers to `MAINTAINERS.md`.

## Recommended GitHub Settings

- Public repository only after secret scan.
- Pull requests required.
- At least one approving review.
- No direct pushes to `main`.
- No force pushes.
- Issues enabled.
- Discussions optional.
- Security advisories enabled.

## Pre-Publication Commands

```powershell
git status --short
git ls-files
git ls-files | Select-String -Pattern "vhdx|wallet.db|drk-testnet.toml|darkfid-testnet.toml|secret|password|seed|private"
```

## Release Decision

Do not make the repository public until the blocking items are resolved.
