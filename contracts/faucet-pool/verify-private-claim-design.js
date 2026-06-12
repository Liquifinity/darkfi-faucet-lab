"use strict";

const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "../..");
const darkfiSourceRoot = process.env.DARKFI_SOURCE_ROOT;
const errors = [];

const design = read("contracts/faucet-pool/PRIVATE_CLAIM_DESIGN.md");
const entrypoint = read("contracts/faucet-pool/rust-skeleton/src/entrypoint.rs");
const money = read("contracts/faucet-pool/rust-skeleton/src/money.rs");
const claimZk = read("contracts/faucet-pool/rust-skeleton/proof/faucet-claim.zk");

for (const required of [
  "dao/src/client/auth_xfer.rs",
  "dao/src/entrypoint/auth_xfer.rs",
  "money/src/client/transfer_v1/builder.rs",
  "money/src/client/transfer_v1/proof.rs",
  "money/src/entrypoint/transfer_v1.rs",
  "recipient_output: MoneyOutputAuth",
  "output coin blind",
  "change output remains locked back to FaucetPool custody",
  "Do not replace the contract claim with direct wallet transfer"
]) {
  if (!design.includes(required)) {
    errors.push(`private claim design missing required text: ${required}`);
  }
}

for (const relativePath of [
  "src/contract/dao/src/client/auth_xfer.rs",
  "src/contract/dao/src/entrypoint/auth_xfer.rs",
  "src/contract/money/src/client/transfer_v1/builder.rs",
  "src/contract/money/src/client/transfer_v1/proof.rs",
  "src/contract/money/src/entrypoint/transfer_v1.rs"
]) {
  const absolutePath = darkfiSourceRoot ? path.join(darkfiSourceRoot, relativePath) : "";
  if (!absolutePath || !fs.existsSync(absolutePath)) {
    errors.push(`missing DarkFi reference: set DARKFI_SOURCE_ROOT and provide ${relativePath}`);
  }
}

checkReference(
  "src/contract/dao/src/client/auth_xfer.rs",
  ["DaoAuthMoneyTransferCall", "input_user_data_enc", "spend_hook"]
);
checkReference(
  "src/contract/dao/src/entrypoint/auth_xfer.rs",
  ["DaoAuthMoneyTransferParams", "MoneyTransferParamsV1", "user_data_enc"]
);
checkReference(
  "src/contract/money/src/client/transfer_v1/builder.rs",
  ["TransferCallBuilder", "create_transfer_burn_proof", "create_transfer_mint_proof"]
);

for (const required of [
  "transfer_call_index as usize != actual_transfer_idx",
  "input_user_data_enc",
  "params.outputs.len() != 2",
  "expected_custody_spend_hook"
]) {
  if (!money.includes(required)) {
    errors.push(`private claim verifier missing required partial check: ${required}`);
  }
}

for (const required of [
  "FAUCET_POOL_ZKAS_CLAIM_NS",
  "zkas_db_set",
  "claim_metadata_darkfi",
  "transfer_params.outputs[0].coin.inner()",
  "transfer_params.outputs[1].coin.inner()",
  "custody_spend_hook.inner()"
]) {
  if (!entrypoint.includes(required)) {
    errors.push(`private claim metadata missing required text: ${required}`);
  }
}

for (const required of [
  "constant \"FaucetClaim\"",
  "recipient_coin",
  "change_coin",
  "constrain_instance(recipient_value)",
  "constrain_instance(recipient_token_id)",
  "constrain_instance(custody_public_x)",
  "constrain_instance(custody_public_y)"
]) {
  if (!claimZk.includes(required)) {
    errors.push(`FaucetClaim zkas missing required text: ${required}`);
  }
}

if (errors.length) {
  console.error("Private claim design: FAILED");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log("Private claim design: OK");

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  if (!fs.existsSync(filePath)) {
    errors.push(`missing file: ${relativePath}`);
    return "";
  }

  return fs.readFileSync(filePath, "utf8");
}

function checkReference(filePath, needles) {
  const content = readDarkfiReference(filePath);
  if (!content) return;
  for (const needle of needles) {
    if (!content.includes(needle)) {
      errors.push(`DarkFi reference ${filePath} missing expected text: ${needle}`);
    }
  }
}

function readDarkfiReference(relativePath) {
  if (!darkfiSourceRoot) {
    errors.push("DARKFI_SOURCE_ROOT is required for DarkFi reference checks");
    return "";
  }

  const filePath = path.join(darkfiSourceRoot, relativePath);
  if (!fs.existsSync(filePath)) {
    errors.push(`missing DarkFi reference: ${relativePath}`);
    return "";
  }

  return fs.readFileSync(filePath, "utf8");
}
