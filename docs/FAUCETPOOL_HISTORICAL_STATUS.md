# FaucetPool Historical Status

This document summarizes the sanitized public status of the FaucetPool work.
It intentionally excludes maintainer wallet databases, raw transaction files,
private configs, chain DBs, WSL disks, logs, and machine-specific evidence.

## Historical Contract

- Contract ID: `DfK7EJ9wNfFaxjUGPLtrGL187K4V6zc865HbjwWQL9Hz`
- Expected fingerprint: `0341532c05d2da8a2c09c07e74bc054892c139ac43c52e958b4bbc8109c2320e`
- DRK TokenId: `241vANigf1Cy3ytjM1KHXiVECxgxdK4yApddL8KcLssb`

## Historical Transactions

| Step | Tx | Historical height | Public status |
| --- | --- | ---: | --- |
| Deploy | `f1eb0209663a0c4d27b30957f621544469770acfbaacb78d90dff5aea36d4661` | 5849 | Confirmed historically |
| Top-up | `683b8eb7105b28a5b2c59eb3a253c1596d196aeb3ebc905011ef5d497b4741e0` | 5860 | Confirmed historically |
| Resume | `39e67840ce5b2b78373cd175dd5e27db45a4598f7cb126109bf4b01e1eb31fa7` | 5935 | Confirmed historically |

These transactions prove prior testnet work. They do not prove the historical
contract is currently funded, unpaused, or claimable.

## Claim Status

No FaucetPool claim has been proven end-to-end in the public repository.

Observed claim attempts reached local validation and broadcast/pending states,
but did not become final public proof because confirmation, wallet receipt, and
duplicate-claim rejection were not all demonstrated against the current chain
and accessible wallet state.

## Current Public Conclusion

The historical FaucetPool should be treated as research evidence, not as an
active public faucet.

The recommended public engineering path is a clean-room faucet cycle with:

1. accessible runtime;
2. explicit test wallet;
3. fresh deploy;
4. minimal top-up;
5. resume/unpause;
6. controlled claim;
7. confirmed chain inclusion;
8. wallet receipt proof;
9. duplicate-claim rejection.

## Publication Boundary

Public:

- contract model;
- Rust/WASM skeleton;
- target invariants;
- sanitized historical status;
- clean-room roadmap.

Private:

- wallet DBs;
- seed phrases;
- private keys;
- raw transaction artifacts;
- WSL disks;
- local runtime configs;
- logs with machine-specific state;
- maintainer evidence directories.
