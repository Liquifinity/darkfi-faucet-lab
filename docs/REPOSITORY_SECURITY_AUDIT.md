# Repository Security Audit

## Audit Result

The tracked public repository contains only collaboration, governance, security, documentation, license, GitHub templates, and the publication safety check.

No wallet DB, VHDX, raw tx artifact, evidence directory, reference repo, or local config is tracked.

## Tracked Surface

Tracked files are intentionally limited to:

- repository metadata;
- security and contribution docs;
- GitHub templates;
- publication safety workflow;
- publication safety script.

## Ignored Private Surface

Ignored:

- `backups/`;
- `evidence/`;
- `refs/`;
- `reports/`;
- `patches/`;
- `tools/`;
- local operational scripts except the safety check;
- VHD/VHDX;
- wallet DB;
- raw tx/call artifacts;
- local DarkFi configs.

## Residual Risk

No repository can be declared 100% secure.

Current residual risks:

- maintainer may accidentally force-add ignored files;
- future contributors may add unsafe scripts;
- GitHub settings can drift;
- CODEOWNERS must reference valid maintainers;
- historical private files remain on the local machine outside tracked Git.

## Required Ongoing Control

Run the publication safety check in CI and before each push.
