"use strict";

const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "../..");
const refsRoot = path.join(root, "runtime/refs/darkfi-contracts");
const errors = [];

const port = read("contracts/faucet-pool/AUTH_MONEY_TRANSFER_PORT.md");
const entrypoint = read("contracts/faucet-pool/rust-skeleton/src/entrypoint.rs");
const money = read("contracts/faucet-pool/rust-skeleton/src/money.rs");
const model = read("contracts/faucet-pool/rust-skeleton/src/model.rs");
const claimZk = read("contracts/faucet-pool/rust-skeleton/proof/faucet-claim.zk");

for (const required of [
  "src/contract/dao/src/client/auth_xfer.rs",
  "src/contract/dao/src/entrypoint/auth_xfer.rs",
  "src/contract/money/src/client/transfer_v1/builder.rs",
  "AuthMoneyTransferEncCoin",
  "AuthMoneyTransfer",
  "get_metadata emits ZK public inputs",
  "recipient address, output public key, and output coin blind remain outside",
  "parent `FaucetPool::Claim` with a child",
  "transfer_call_index"
]) {
  if (!port.includes(required)) {
    errors.push(`auth money transfer port missing required text: ${required}`);
  }
}

checkReference("src/contract/dao/src/entrypoint/auth_xfer.rs", [
  "DAO_CONTRACT_ZKAS_AUTH_MONEY_TRANSFER_ENC_COIN_NS",
  "DAO_CONTRACT_ZKAS_AUTH_MONEY_TRANSFER_NS",
  "zk_public_inputs.push",
  "MoneyFunction::TransferV1",
  "user_data_enc",
  "AuthXferWrongOutputCoin"
]);

checkReference("src/contract/dao/src/client/auth_xfer.rs", [
  "DaoAuthMoneyTransferCall",
  "ElGamalEncryptedNote",
  "Proof::create",
  "public_inputs",
  "input_user_data_enc"
]);

checkReference("src/contract/dao/src/model.rs", [
  "pub struct DaoAuthMoneyTransferParams",
  "enc_attrs",
  "dao_change_attrs"
]);

checkReference("src/contract/money/src/client/transfer_v1/proof.rs", [
  "create_transfer_burn_proof",
  "create_transfer_mint_proof"
]);

if (!/pub struct PrivateClaimAuthorization[\s\S]*public_inputs: Vec<u8>[\s\S]*proof: Vec<u8>/.test(model)) {
  errors.push("private claim authorization model changed; update port gate");
}

for (const required of [
  "claim_metadata_darkfi",
  "FAUCET_POOL_ZKAS_CLAIM_NS",
  "zkas_db_set",
  "transfer_params.outputs[0].coin.inner()",
  "transfer_params.outputs[1].coin.inner()"
]) {
  if (!entrypoint.includes(required)) {
    errors.push(`FaucetClaim metadata missing required text: ${required}`);
  }
}

for (const required of [
  "params.outputs.len() != 2",
  "expected_custody_spend_hook",
  "Ok(())"
]) {
  if (!money.includes(required)) {
    errors.push(`private claim verifier missing required text: ${required}`);
  }
}

for (const required of [
  "constant \"FaucetClaim\"",
  "recipient_coin",
  "change_coin",
  "constrain_instance(recipient_value)",
  "constrain_instance(recipient_token_id)"
]) {
  if (!claimZk.includes(required)) {
    errors.push(`FaucetClaim zkas missing required text: ${required}`);
  }
}

if (errors.length) {
  console.error("Auth money transfer port: FAILED");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log("Auth money transfer port: OK");

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  if (!fs.existsSync(filePath)) {
    errors.push(`missing file: ${relativePath}`);
    return "";
  }

  return fs.readFileSync(filePath, "utf8");
}

function checkReference(relativePath, needles) {
  const filePath = path.join(refsRoot, relativePath);
  if (!fs.existsSync(filePath)) {
    errors.push(`missing exported DarkFi reference: ${relativePath}`);
    return;
  }

  const content = fs.readFileSync(filePath, "utf8");
  for (const needle of needles) {
    if (!content.includes(needle)) {
      errors.push(`DarkFi reference ${relativePath} missing expected text: ${needle}`);
    }
  }
}
