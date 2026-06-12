"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const root = __dirname;
const entrypoint = read("rust-skeleton/src/entrypoint.rs");
const admin = read("rust-skeleton/src/entrypoint/admin.rs");

assert.match(
  entrypoint,
  /FaucetPoolFunction::Pause \| FaucetPoolFunction::Resume[\s\S]*encode_metadata\(vec!\[public_key_from_bytes\(&params\.admin_public_key\)\?\]\)/,
  "pause/resume metadata must require an admin transaction signature"
);

assert.match(
  admin,
  /let config = storage::load_config_darkfi\(cid\)\?;[\s\S]*admin_public_key != config\.admin_public_key[\s\S]*FaucetPoolError::Unauthorized/,
  "admin exec path must compare signed admin key against stored config"
);

assert.doesNotMatch(
  admin,
  /Payload identity.*is not authorization/,
  "admin auth must not be marked as unresolved"
);

console.log("faucet admin authorization verified");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}
