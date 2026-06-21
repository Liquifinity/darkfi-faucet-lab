# On-Chain Proof Status

## Summary

The repository is not able to produce a production on-chain faucet proof from the current local state.

Reason:

- the historical claim attempts in `evidence/` are abandoned/not confirmed;
- the required production proof file `runtime/state/faucet-contract-proof.json` does not exist;
- the worker and generator interface stubs are now present, but they do not create a real confirmed claim proof by themselves.

## Local Validation

Executed directly in this repository:

- `node contracts/faucet-pool/verify-contract-target.js` -> passed
- `node contracts/faucet-pool/verify-claim-privacy-target.js` -> passed
- `node contracts/faucet-pool/verify-deploy-proof.js` -> failed because the production proof file is missing

## Blocking Details

`verify-deploy-proof.js` reports:

- `Faucet contract proof file not found: runtime/state/faucet-contract-proof.json`

## Evidence State

The current claim attempt evidence in `evidence/claim-attempt-current/05-execution-report.md` records:

- tx hash `4edb3c2bb997ab0fd0145f05a573374ca03b5cfea3bbf0a281737faa831156d9`
- tx-from-calls: passed
- inspect: passed
- broadcast: done once
- `tx.pending`: contained the tx
- `fetch-tx`: not found
- `txs-history`: Broadcasted, block height `-`
- final classification: abandoned / relay failure probable

That is not a confirmed on-chain claim proof.

## Conclusion

The current repository state is evidence-ready but not proof-complete.
The missing production proof cannot be fabricated locally from the available artifacts.

## Next Required Step

Obtain a real confirmed FaucetPool claim on the DarkFi testnet, then generate:

`runtime/state/faucet-contract-proof.json`

Only after that should `contracts/faucet-pool/verify-deploy-proof.js` be expected to pass.
