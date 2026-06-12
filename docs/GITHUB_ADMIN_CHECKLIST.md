# GitHub Admin Checklist

## Repository Settings

- [ ] Create a dedicated repository for DarkFi Faucet work.
- [ ] Do not publish from a parent workspace by accident.
- [ ] Add license.
- [ ] Add maintainers.
- [ ] Enable Issues.
- [ ] Enable Discussions if useful.
- [ ] Enable Security Advisories.

## Branch Protection

For `main`:

- [ ] Require pull request before merge.
- [ ] Require at least one approval.
- [ ] Require conversation resolution.
- [ ] Require status checks.
- [ ] Block force pushes.
- [ ] Block deletions.
- [ ] Require linear history if desired.

## Security

- [ ] Enable secret scanning.
- [ ] Enable push protection.
- [ ] Enable Dependabot alerts.
- [ ] Enable Dependabot security updates.
- [ ] Review GitHub Actions permissions.
- [ ] Set default workflow token permission to read-only.

## Access

- [ ] Public users: issue/PR only.
- [ ] Contributors: fork-based PRs.
- [ ] Maintainers: merge only after review.
- [ ] Admin rights limited.

## Before Public Release

- [ ] Run `cargo run --locked --bin publication-safety-check`.
- [ ] Confirm no ignored sensitive files are tracked.
- [ ] Confirm no VHDX/wallet DB/raw tx/local config is tracked.
- [ ] Confirm historical reports are sanitized or excluded.
