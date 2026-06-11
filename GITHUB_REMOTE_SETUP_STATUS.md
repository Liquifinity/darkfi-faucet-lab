# GitHub Remote Setup Status

## Status

Local repository is ready.

Remote creation is blocked because GitHub CLI is not authenticated in this environment.

## Local Repository

- Branch: `main`
- Initial public commit: `a5a1d1e Prepare public collaboration repository`
- License: Apache-2.0
- Safety check: passed

## GitHub CLI

Portable GitHub CLI was downloaded to `.tmp/gh-portable`.

Detected version:

```text
gh version 2.94.0
```

Authentication status:

```text
not logged in
```

The attempted `gh auth login --web` did not complete inside the command timeout.

## Intended Remote

```text
https://github.com/Liquifinity/darkfi-faucet-lab.git
```

## Required Manual Step

Run:

```powershell
$gh = Get-Content -Raw .\.tmp\gh-path.txt
& $gh.Trim() auth login --hostname github.com --git-protocol https --web
```

After login succeeds:

```powershell
& $gh.Trim() repo create Liquifinity/darkfi-faucet-lab --public --source . --remote origin --push
```

If the repo already exists:

```powershell
git remote add origin https://github.com/Liquifinity/darkfi-faucet-lab.git
git push -u origin main
```

## GitHub Settings To Configure

- Secret scanning: enabled.
- Push protection: enabled.
- Dependabot alerts: enabled.
- Security advisories: enabled.
- Branch protection for `main`.
- Pull requests required.
- At least one review required.
- Force pushes disabled.
- Branch deletion disabled.

