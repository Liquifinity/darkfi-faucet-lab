# Maintainer Security Operations

## Access Model

Use least privilege.

- Admin: repository settings only.
- Maintainer: merge after review.
- Contributor: fork and PR.
- Public user: issues and discussions only.

## Required GitHub Controls

- Branch protection on `main`.
- Required PR reviews.
- CODEOWNERS review.
- Secret scanning.
- Push protection.
- Dependabot alerts.
- Actions read-only by default.

## Release Hygiene

Before any release:

```powershell
.\scripts\publication-safety-check.ps1
git status --short --ignored
git ls-files
```

Confirm no tracked files match:

- VHDX;
- wallet DB;
- raw tx;
- local config;
- evidence;
- backups;
- private reports;
- secrets.

## Incident Response

If a secret is exposed:

1. Revoke it immediately.
2. Rotate affected credentials.
3. Remove the public reference.
4. Assume Git history is compromised.
5. Publish only sanitized remediation notes.

If wallet material is exposed:

1. Treat the wallet as compromised.
2. Move funds from a different trusted environment where possible.
3. Do not keep using the exposed wallet.
