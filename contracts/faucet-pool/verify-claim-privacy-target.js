"use strict";

const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "../..");
const errors = [];

const target = read("contracts/faucet-pool/CLAIM_PRIVACY_TARGET.md");
const model = read("contracts/faucet-pool/rust-skeleton/src/model.rs");
const claim = read("contracts/faucet-pool/rust-skeleton/src/entrypoint/claim.rs");
const entrypoint = read("contracts/faucet-pool/rust-skeleton/src/entrypoint.rs");
const worker = read("worker/src/index.ts");
const claimParams = extractStruct(model, "ClaimParams");

requireText(target, "must not contain `recipient_output: MoneyOutputAuth`");
requireText(target, "must not contain an output `public_key`");
requireText(target, "must not contain an output `coin_blind`");
requireText(target, "privacy-preserving proof");

if (/recipient_output: MoneyOutputAuth/.test(claimParams)) {
  errors.push("ClaimParams still contains recipient_output: MoneyOutputAuth");
}

if (/recipient_address: Vec<u8>/.test(claimParams)) {
  errors.push("ClaimParams still contains recipient_address");
}

if (/public_key: Vec<u8>/.test(claimParams)) {
  errors.push("ClaimParams still exposes output public_key");
}

if (/coin_blind: Vec<u8>/.test(claimParams)) {
  errors.push("ClaimParams still exposes output coin_blind");
}

if (/recipient_output/.test(claim) && /verify_money_flow_darkfi/.test(claim)) {
  errors.push("claim verifier still depends on recipient_output preimage");
}

const money = read("contracts/faucet-pool/rust-skeleton/src/money.rs");
const claimGenerator = read("scripts/generate-faucet-claim-call-wsl.sh");
const privateAuth = extractFunction(money, "verify_private_claim_authorization");
if (/^\s*Err\(FaucetPoolError::MoneyTransferNotVerified\)\s*$/.test(privateAuth.trim())) {
  errors.push("private claim authorization verifier is not implemented");
}
if (/_authorization: &PrivateClaimAuthorization/.test(money)) {
  errors.push("private claim authorization is ignored in the verifier");
}
if (
  !/authorization\.public_inputs\.is_empty\(\)[\s\S]*authorization\.proof\.is_empty\(\)/.test(privateAuth) ||
  !/return Err\(FaucetPoolError::MoneyTransferNotVerified\)/.test(privateAuth)
) {
  errors.push("payload-level claim authorization must be rejected when non-empty");
}

for (const required of [
  "claim_metadata_darkfi",
  "FAUCET_POOL_ZKAS_CLAIM_NS",
  "transfer_params.outputs[0].coin.inner()",
  "transfer_params.outputs[1].coin.inner()",
  "custody_spend_hook.inner()"
]) {
  if (!entrypoint.includes(required)) {
    errors.push(`claim privacy metadata missing required text: ${required}`);
  }
}

for (const required of [
  "create_faucet_claim_proof",
  "ContractCallImport::new(faucet_call, vec![claim_proof.as_ref().to_vec()], vec![])",
  "authorization: PrivateClaimAuthorization { public_inputs: vec![], proof: vec![] }"
]) {
  if (!claimGenerator.includes(required)) {
    errors.push(`claim generator missing required proof handling: ${required}`);
  }
}

if (
  !/payoutMode === "contract"[\s\S]*assertContractPayoutReady\(\)[\s\S]*Contract payout execution is not implemented/.test(worker)
) {
  errors.push("worker does not fail closed after contract proof validation");
}

if (errors.length) {
  console.error("Claim privacy target: BLOCKED");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log("Claim privacy target: OK");

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  if (!fs.existsSync(filePath)) {
    errors.push(`missing file: ${relativePath}`);
    return "";
  }

  return fs.readFileSync(filePath, "utf8");
}

function requireText(content, text) {
  if (!content.includes(text)) {
    errors.push(`claim privacy target missing required text: ${text}`);
  }
}

function extractStruct(content, name) {
  const match = content.match(new RegExp(`pub struct ${name} \\{([\\s\\S]*?)\\n\\}`));
  if (!match) {
    errors.push(`missing struct: ${name}`);
    return "";
  }

  return match[1];
}

function extractFunction(content, name) {
  const start = content.indexOf(`fn ${name}`);
  if (start === -1) {
    errors.push(`missing function: ${name}`);
    return "";
  }

  const bodyStart = content.indexOf("{", start);
  if (bodyStart === -1) {
    errors.push(`missing function body: ${name}`);
    return "";
  }

  let depth = 0;
  for (let i = bodyStart; i < content.length; i++) {
    if (content[i] === "{") depth += 1;
    if (content[i] === "}") depth -= 1;
    if (depth === 0) return content.slice(bodyStart + 1, i);
  }

  errors.push(`unterminated function body: ${name}`);
  return "";
}
