#!/usr/bin/env bash
set -euo pipefail

# Fail-closed scaffold for the production claim generator.
# This file is intentionally explicit about the proof shape expected by the
# verifier, but it does not mint a production claim by itself.
#
# Required future flow:
# - create_faucet_claim_proof
# - ContractCallImport::new(faucet_call, vec![claim_proof.as_ref().to_vec()], vec![])
# - authorization: PrivateClaimAuthorization { public_inputs: vec![], proof: vec![] }
#
# In the production path the worker must call assertContractPayoutReady() before
# attempting a contract payout.
# payoutMode === "contract"

echo "Contract payout execution is not implemented." >&2
exit 1
