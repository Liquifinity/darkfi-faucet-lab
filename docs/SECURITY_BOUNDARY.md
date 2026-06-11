# Security Boundary

## Trust Boundary

The public repository is not a custody environment.

Anything committed to GitHub must be safe for strangers to clone, fork, run, inspect, and modify.

## Private Boundary

Keep private:

- maintainer wallet;
- local WSL runtime;
- local node state;
- VHDX backups;
- faucet funding wallet;
- deployment credentials;
- Railway account;
- admin secrets;
- raw transaction artifacts tied to live coins;
- logs containing local paths, addresses, or operational metadata that should not be public.

## Public Boundary

Allowed public material:

- code that can run without secrets;
- example configs with placeholders;
- tests;
- docs;
- runbooks;
- security policy;
- reproducible local setup instructions.

## Rule for Scripts

Scripts must be classified:

- read-only;
- local setup;
- node/miner control;
- wallet-affecting;
- chain DB-affecting;
- broadcast/deploy;
- recovery/destructive.

Wallet-affecting, chain DB-affecting, broadcast, deploy, and recovery scripts must require explicit human review before use.

## Rule for Examples

Use placeholders:

```text
CHANGE_ME
YOUR_WALLET_PATH
YOUR_RPC_ENDPOINT
YOUR_TESTNET_ADDRESS
```

Never use real maintainer credentials or live wallet paths as copy-paste defaults.
