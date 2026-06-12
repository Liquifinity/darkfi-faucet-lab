# Evidence And Decision Index

This index explains which findings were promoted into the public repository and
which remain private.

## Public Evidence

| Area | Public file | Purpose |
| --- | --- | --- |
| FaucetPool status | `docs/FAUCETPOOL_HISTORICAL_STATUS.md` | Sanitized record of historical deploy/top-up/resume and claim status |
| Clean-room continuation | `docs/CLEAN_ROOM_FAUCET_CYCLE.md` | Reproducible next cycle for contributors |
| Contract rules | `contracts/faucet-pool/SPEC.md` | FaucetPool invariants |
| Contract interface | `contracts/faucet-pool/interface.json` | Target public call surface |
| Executable model | `contracts/faucet-pool/pool-model.js` | Policy reference implementation |
| Conformance vectors | `contracts/faucet-pool/conformance-vectors.json` | Expected behavior for Rust/WASM port |
| Rust skeleton | `contracts/faucet-pool/rust-skeleton/` | Public contract implementation starting point |
| Publication security | `src/bin/publication_safety_check.rs` | Prevents accidental public tracking of private artifacts |
| Security boundary | `docs/SECURITY_BOUNDARY.md` | Defines what must never become public |

## Private Evidence Not Published

The private workspace contains operational reports and machine outputs for:

- WSL runtime discovery;
- VHDX backup and storage mapping;
- node sync recovery;
- wallet reconciliation;
- nullifier/OwnCoin inspection;
- raw transaction monitoring;
- local logs;
- local runtime paths.

Those are not published directly because they may contain machine-specific
paths, runtime state, wallet metadata, or raw operational context. Their
public value has been extracted into the documents listed above.

## Decisions Already Made

### Public Repository Boundary

The repository publishes source, specs, models, checklists, and sanitized
status. It does not publish maintainer runtime state.

### Historical FaucetPool

The historical contract is treated as research evidence, not as an active
claimable faucet.

### Next Engineering Track

The next credible path is a clean-room deploy/top-up/resume/claim cycle with
publicly reproducible evidence.

### GitHub Security

The public repository uses:

- Apache-2.0 license;
- branch protection;
- required publication safety check;
- secret scanning and push protection;
- private vulnerability reporting;
- explicit contribution and security policy.

## Evidence Quality Standard

Do not mark a faucet flow successful unless all of these are true:

- deploy transaction confirmed;
- top-up transaction confirmed;
- resume/unpause transaction confirmed;
- claim transaction confirmed;
- wallet scan observes receipt;
- duplicate claim is rejected;
- no private maintainer material is required to reproduce the proof.

