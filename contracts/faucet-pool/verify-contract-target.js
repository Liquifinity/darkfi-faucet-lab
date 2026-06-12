"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const {
  CLAIM_AMOUNT_UNITS,
  COOLDOWN_SECONDS,
  DAILY_POOL_UNITS,
  drkToUnits
} = require("./pool-model");

const root = __dirname;
const interfaceSpec = readJson("interface.json");
const vectors = readJson("conformance-vectors.json");

assert.equal(interfaceSpec.name, "darkfi_testnet_faucet_pool");
assert.equal(interfaceSpec.policy.claim_amount, "0.00003");
assert.equal(interfaceSpec.policy.cooldown_seconds, COOLDOWN_SECONDS);
assert.equal(interfaceSpec.policy.daily_pool_amount, "0.33");
assert.equal(interfaceSpec.policy.carry_over, false);

assert.equal(drkToUnits(interfaceSpec.policy.claim_amount), CLAIM_AMOUNT_UNITS);
assert.equal(drkToUnits(interfaceSpec.policy.daily_pool_amount), DAILY_POOL_UNITS);

assert.equal(vectors.schema, "darkfi-faucet-pool-conformance-v1");
assert.equal(BigInt(vectors.constants.claim_amount_units), CLAIM_AMOUNT_UNITS);
assert.equal(vectors.constants.claim_amount_drk, interfaceSpec.policy.claim_amount);
assert.equal(vectors.constants.cooldown_seconds, COOLDOWN_SECONDS);
assert.equal(BigInt(vectors.constants.daily_pool_units), DAILY_POOL_UNITS);
assert.equal(vectors.constants.daily_pool_drk, interfaceSpec.policy.daily_pool_amount);

const callNames = new Set(interfaceSpec.calls.map((call) => call.name));
for (const requiredCall of [
  "top_up",
  "claim",
  "pause",
  "resume",
  "emergency_withdraw"
]) {
  assert.ok(callNames.has(requiredCall), `interface.json missing call: ${requiredCall}`);
}

const vectorNames = new Set(vectors.vectors.map((vector) => vector.name));
for (const requiredVector of [
  "successful claim transfers DRK",
  "duplicate wallet claim before 12h is rejected",
  "paused pool rejects claim",
  "claim above daily pool is rejected",
  "new day resets allowance without carrying unused budget",
  "insufficient custody balance rejects claim",
  "top-up increases custody balance without increasing daily allowance"
]) {
  assert.ok(vectorNames.has(requiredVector), `conformance-vectors.json missing vector: ${requiredVector}`);
}

for (const requiredSkeletonFile of [
  "rust-skeleton/README.md",
  "rust-skeleton/Cargo.toml",
  "rust-skeleton/Cargo.darkfi.toml",
  "rust-skeleton/proof/faucet-claim.zk",
  "rust-skeleton/src/lib.rs",
  "rust-skeleton/src/model.rs",
  "rust-skeleton/src/error.rs",
  "rust-skeleton/src/policy.rs",
  "rust-skeleton/src/storage.rs",
  "rust-skeleton/src/money.rs",
  "rust-skeleton/src/time.rs",
  "rust-skeleton/src/entrypoint.rs",
  "rust-skeleton/src/entrypoint/top_up.rs",
  "rust-skeleton/src/entrypoint/claim.rs",
  "rust-skeleton/src/entrypoint/admin.rs"
]) {
  assert.ok(
    fs.existsSync(path.join(root, requiredSkeletonFile)),
    `rust skeleton missing file: ${requiredSkeletonFile}`
  );
}

console.log("contract target verified");

function readJson(fileName) {
  return JSON.parse(fs.readFileSync(path.join(root, fileName), "utf8"));
}
