"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const proofPath =
  process.env.FAUCET_CONTRACT_PROOF ??
  path.join(__dirname, "../../runtime/state/faucet-contract-proof.json");

const requiredGates = [
  "wasm_build",
  "auth_money_transfer_port",
  "deployooor_deploy",
  "contract_id_recorded",
  "operator_top_up",
  "public_top_up",
  "top_up_does_not_expand_daily_pool",
  "successful_claim",
  "recipient_received_exact_claim_amount",
  "duplicate_claim_rejected",
  "daily_pool_depletion_rejected",
  "daily_reset_restores_allowance",
  "pause_rejects_claim",
  "resume_allows_claim",
  "emergency_withdraw_admin_only",
  "custody_inputs_verified"
];

if (!fs.existsSync(proofPath)) {
  console.error(`Faucet contract proof file not found: ${proofPath}`);
  process.exit(1);
}

const proof = JSON.parse(fs.readFileSync(proofPath, "utf8"));

assert.equal(proof.schema, "darkfi-faucet-contract-proof-v1");
assert.equal(proof.network, "DarkFi testnet");
assert.ok(isContractId(proof.contract_id), "contract_id must be a valid DarkFi base58 id");
assert.match(proof.source_fingerprint, /^[a-f0-9]{64}$/, "source_fingerprint must be a sha256 hex digest");
assert.ok(isIsoDate(proof.generated_at), "generated_at must be a valid ISO timestamp");
assert.ok(Array.isArray(proof.gates), "gates must be an array");

const buildProofPath = path.join(__dirname, "../../runtime/state/faucet-contract-build-proof.json");
assert.ok(fs.existsSync(buildProofPath), "build proof is required");
const buildProof = JSON.parse(fs.readFileSync(buildProofPath, "utf8"));
assert.equal(buildProof.schema, "darkfi-faucet-contract-build-proof-v1");
assert.match(buildProof.source_fingerprint, /^[a-f0-9]{64}$/, "build proof fingerprint must be a sha256 hex digest");
assert.equal(
  proof.source_fingerprint,
  buildProof.source_fingerprint,
  "deploy proof does not match current source fingerprint"
);

const gates = new Map(proof.gates.map((gate) => [gate.name, gate]));
assert.equal(gates.size, proof.gates.length, "deploy proof gates must not contain duplicates");

for (const gateName of requiredGates) {
  const gate = gates.get(gateName);
  assert.ok(gate, `missing proof gate: ${gateName}`);
  assert.equal(gate.status, "passed", `proof gate not passed: ${gateName}`);
  assert.ok(isRealEvidence(gate.evidence), `proof gate lacks real evidence: ${gateName}`);
}

console.log(`faucet contract deploy proof verified: ${proofPath}`);

function isContractId(value) {
  return typeof value === "string" && /^[1-9A-HJ-NP-Za-km-z]{32,64}$/.test(value);
}

function isIsoDate(value) {
  if (typeof value !== "string" || value.includes("<")) return false;
  const timestamp = Date.parse(value);
  return Number.isFinite(timestamp) && new Date(timestamp).toISOString() === value;
}

function isRealEvidence(value) {
  return typeof value === "string" && value.trim().length >= 24 && !/[<>]/.test(value);
}
