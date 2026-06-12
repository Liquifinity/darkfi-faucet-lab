# Remote Repository Setup

Use a dedicated repository for public work. Do not publish from a parent
workspace that also contains private projects or operational material.

## Recommended Repository

Suggested GitHub repository:

```text
Liquifinity/darkfi-faucet-lab
```

Current visibility target: public, after safety checks and branch protection.

## If GitHub CLI Is Available

From the repository root:

```bash
gh auth login
gh repo create Liquifinity/darkfi-faucet-lab --public --source . --remote origin --push
```

Then configure:

```bash
gh repo edit Liquifinity/darkfi-faucet-lab --enable-issues --enable-wiki=false
```

Branch protection should be configured from GitHub UI or API:

- require pull request before merge;
- require at least one approval;
- require status checks;
- block force pushes;
- block branch deletion.

## If Using GitHub Web UI

1. Create a new empty repo: `darkfi-faucet-lab`.
2. Do not initialize with README/license/gitignore; these already exist locally.
3. Add remote:

```bash
git remote add origin https://github.com/Liquifinity/darkfi-faucet-lab.git
git push -u origin main
```

4. Enable:

- secret scanning;
- push protection;
- Dependabot alerts;
- Dependabot security updates;
- security advisories;
- branch protection for `main`.

## Do Not Push

Do not push:

- `backups/`;
- `evidence/`;
- `refs/`;
- VHDX files;
- wallet DBs;
- raw tx artifacts;
- local configs;
- chain DB/state;
- ignored operational reports.

The local `.gitignore` is configured to exclude those paths.
