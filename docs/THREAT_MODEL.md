# Threat Model

## Assets

- Wallet DB and password.
- Private keys and seed material.
- DRK testnet funds.
- Contract admin keys.
- Faucet custody state.
- Railway/deployment secrets.
- Local WSL VHDX and backups.
- Raw transaction artifacts that spend real coins.

## Threats

### Secret Leakage

Risk:

- secrets committed to Git;
- secrets pasted into issues;
- raw artifacts exposing operational state.

Controls:

- `.gitignore`;
- secret scanning;
- PR checklist;
- security policy;
- no raw wallet artifacts in public repo.

### Malicious Pull Requests

Risk:

- weakening validation;
- changing scripts to broadcast or mutate wallet state;
- adding exfiltration logic.

Controls:

- branch protection;
- CODEOWNERS;
- required reviews;
- minimal permissions;
- CI safety checks.

### Supply Chain Changes

Risk:

- unreviewed dependency additions;
- malicious GitHub Actions;
- unsafe shell downloads.

Controls:

- require review for dependency changes;
- pin actions where practical;
- avoid new dependencies unless necessary.

### Operational Confusion

Risk:

- contributors accidentally use maintainer-specific paths;
- scripts mutate local wallet/chain state;
- stale reports are mistaken for live truth.

Controls:

- public/private boundary docs;
- explicit script classification;
- release checklist;
- evidence is private by default.

## Current Main Risk

The local workspace contains private operational material. The public repo must be curated, not published wholesale.
