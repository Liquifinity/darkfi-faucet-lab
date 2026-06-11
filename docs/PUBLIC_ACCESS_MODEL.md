# Public Access Model

## Goal

Make the project usable by contributors without giving anyone access to the maintainer computer, wallet, node, Railway account, secrets, or local WSL runtime.

The public repository should contain reproducible source, documentation, runbooks, and test instructions. It should not contain live operational state.

## What Contributors Can Do

Contributors can:

- open issues;
- propose pull requests;
- improve documentation;
- improve scripts that do not mutate wallet or chain state by default;
- review contract logic;
- add tests;
- reproduce flows in their own DarkFi testnet environment;
- propose security improvements.

Contributors cannot:

- access the maintainer wallet;
- access local node state;
- use maintainer DRK;
- broadcast from maintainer infrastructure;
- deploy from maintainer credentials;
- change production/Railway settings;
- retrieve private evidence or backups.

## Permission Model

Recommended GitHub roles:

- Public users: issue and PR access only.
- Contributors: fork and PR workflow.
- Maintainers: merge rights after review.
- Admins: repository settings, secrets, branch protection.

Direct write access should be limited.

## Decentralized Operation

The project should be reproducible:

- each contributor runs their own node/wallet/testnet environment;
- claims/deploys are tested against explicit testnet contracts;
- no contributor depends on the maintainer machine;
- no private wallet material is shared;
- public artifacts are documentation and source, not custody state.

## Public Repository Boundary

Publish:

- source code;
- sanitized scripts;
- docs;
- tests;
- non-sensitive examples;
- architecture notes.

Do not publish:

- VHDX backups;
- wallet DBs;
- private keys;
- seed phrases;
- local configs with passwords;
- chain DB;
- raw tx artifacts from live wallets;
- Railway secrets;
- private evidence.
