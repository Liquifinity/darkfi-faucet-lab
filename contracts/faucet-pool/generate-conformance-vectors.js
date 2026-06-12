"use strict";

const fs = require("node:fs");
const path = require("node:path");
const assert = require("node:assert/strict");
const {
  CLAIM_AMOUNT_UNITS,
  COOLDOWN_SECONDS,
  DAILY_POOL_UNITS,
  FaucetPoolModel,
  drkToUnits
} = require("./pool-model");

const outputPath = path.join(__dirname, "conformance-vectors.json");

function main() {
  const vectors = buildVectors();
  const serialized = `${JSON.stringify(vectors, replacer, 2)}\n`;

  if (process.argv.includes("--check")) {
    const existing = fs.readFileSync(outputPath, "utf8");
    assert.equal(existing, serialized, "conformance-vectors.json is stale");
    return;
  }

  fs.writeFileSync(outputPath, serialized);
}

function buildVectors() {
  return {
    schema: "darkfi-faucet-pool-conformance-v1",
    generated_by: "contracts/faucet-pool/generate-conformance-vectors.js",
    constants: {
      claim_amount_units: CLAIM_AMOUNT_UNITS,
      claim_amount_drk: "0.00003",
      cooldown_seconds: COOLDOWN_SECONDS,
      daily_pool_units: DAILY_POOL_UNITS,
      daily_pool_drk: "0.33"
    },
    vectors: [
      successfulClaimVector(),
      cooldownVector(),
      pauseVector(),
      dailyPoolVector(),
      dailyResetVector(),
      insufficientBalanceVector(),
      topUpDoesNotIncreaseAllowanceVector()
    ]
  };
}

function successfulClaimVector() {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("1") });
  const result = pool.claim("wallet-a", 1_000);

  return {
    name: "successful claim transfers DRK",
    initial_state: { balance_units: drkToUnits("1") },
    call: { function: "claim", recipient: "wallet-a", timestamp_seconds: 1_000 },
    expected: normalizeResult(result),
    expected_state: snapshot(pool, "wallet-a")
  };
}

function cooldownVector() {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("1") });
  pool.claim("wallet-a", 1_000);
  const result = pool.claim("wallet-a", 1_000 + COOLDOWN_SECONDS - 1);

  return {
    name: "duplicate wallet claim before 12h is rejected",
    initial_operations: [
      { function: "claim", recipient: "wallet-a", timestamp_seconds: 1_000 }
    ],
    call: {
      function: "claim",
      recipient: "wallet-a",
      timestamp_seconds: 1_000 + COOLDOWN_SECONDS - 1
    },
    expected: normalizeResult(result),
    expected_state: snapshot(pool, "wallet-a")
  };
}

function pauseVector() {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("1") });
  pool.pause();
  const result = pool.claim("wallet-a", 1_000);

  return {
    name: "paused pool rejects claim",
    initial_operations: [{ function: "pause" }],
    call: { function: "claim", recipient: "wallet-a", timestamp_seconds: 1_000 },
    expected: normalizeResult(result),
    expected_state: snapshot(pool, "wallet-a")
  };
}

function dailyPoolVector() {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("1") });
  pool.refreshDay(1_000);
  pool.spentTodayUnits = DAILY_POOL_UNITS;
  const result = pool.claim("wallet-overflow", 1_000);

  return {
    name: "claim above daily pool is rejected",
    initial_state: {
      balance_units: drkToUnits("1"),
      current_day_timestamp_seconds: 1_000,
      spent_today_units: DAILY_POOL_UNITS
    },
    call: { function: "claim", recipient: "wallet-overflow", timestamp_seconds: 1_000 },
    expected: normalizeResult(result),
    expected_state: snapshot(pool, "wallet-overflow")
  };
}

function dailyResetVector() {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("1") });
  pool.refreshDay(1);
  pool.spentTodayUnits = drkToUnits("0.15");
  const result = pool.claim("wallet-next-day", 86_401);

  return {
    name: "new day resets allowance without carrying unused budget",
    initial_state: {
      balance_units: drkToUnits("1"),
      current_day_timestamp_seconds: 1,
      spent_today_units: drkToUnits("0.15")
    },
    call: { function: "claim", recipient: "wallet-next-day", timestamp_seconds: 86_401 },
    expected: normalizeResult(result),
    expected_state: snapshot(pool, "wallet-next-day")
  };
}

function insufficientBalanceVector() {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("0.00002") });
  const result = pool.claim("wallet-a", 1_000);

  return {
    name: "insufficient custody balance rejects claim",
    initial_state: { balance_units: drkToUnits("0.00002") },
    call: { function: "claim", recipient: "wallet-a", timestamp_seconds: 1_000 },
    expected: normalizeResult(result),
    expected_state: snapshot(pool, "wallet-a")
  };
}

function topUpDoesNotIncreaseAllowanceVector() {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("0.33") });
  pool.topUp(drkToUnits("10"));
  pool.refreshDay(1_000);
  pool.spentTodayUnits = DAILY_POOL_UNITS;
  const result = pool.claim("wallet-overflow", 1_000);

  return {
    name: "top-up increases custody balance without increasing daily allowance",
    initial_operations: [{ function: "top_up", amount_units: drkToUnits("10") }],
    initial_state: {
      balance_units: drkToUnits("10.33"),
      current_day_timestamp_seconds: 1_000,
      spent_today_units: DAILY_POOL_UNITS
    },
    call: { function: "claim", recipient: "wallet-overflow", timestamp_seconds: 1_000 },
    expected: normalizeResult(result),
    expected_state: snapshot(pool, "wallet-overflow")
  };
}

function snapshot(pool, recipient) {
  return {
    balance_units: pool.balanceUnits,
    current_day: pool.currentDay,
    spent_today_units: pool.spentTodayUnits,
    recipient_last_claim_at: pool.lastClaimAt.get(recipient) ?? null,
    paused: pool.paused
  };
}

function normalizeResult(result) {
  return Object.fromEntries(
    Object.entries(result).map(([key, value]) => [key, typeof value === "bigint" ? value.toString() : value])
  );
}

function replacer(_key, value) {
  return typeof value === "bigint" ? value.toString() : value;
}

main();
