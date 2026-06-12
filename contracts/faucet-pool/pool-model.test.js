"use strict";

const assert = require("node:assert/strict");
const test = require("node:test");
const {
  COOLDOWN_SECONDS,
  CLAIM_AMOUNT_UNITS,
  DAILY_POOL_UNITS,
  FaucetPoolModel,
  drkToUnits,
  unitsToDrk
} = require("./pool-model");

test("amount constants match faucet policy", () => {
  assert.equal(CLAIM_AMOUNT_UNITS, drkToUnits("0.00003"));
  assert.equal(DAILY_POOL_UNITS, drkToUnits("0.33"));
  assert.equal(unitsToDrk(CLAIM_AMOUNT_UNITS), "0.00003");
  assert.equal(unitsToDrk(DAILY_POOL_UNITS), "0.33");
});

test("wallet can claim only once per 12 hours", () => {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("1") });

  assert.equal(pool.claim("wallet-a", 1_000).ok, true);
  assert.deepEqual(pool.claim("wallet-a", 1_000 + COOLDOWN_SECONDS - 1), {
    ok: false,
    reason: "cooldown"
  });
  assert.equal(pool.claim("wallet-a", 1_000 + COOLDOWN_SECONDS).ok, true);
});

test("different wallets share the same daily pool", () => {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("1") });
  const maxClaims = Number(DAILY_POOL_UNITS / CLAIM_AMOUNT_UNITS);

  for (let i = 0; i < maxClaims; i += 1) {
    assert.equal(pool.claim(`wallet-${i}`, 10_000).ok, true);
  }

  assert.deepEqual(pool.claim("wallet-overflow", 10_000), {
    ok: false,
    reason: "daily_pool_depleted"
  });
});

test("daily allowance resets without carrying unused budget", () => {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("1") });

  for (let i = 0; i < 5_000; i += 1) {
    assert.equal(pool.claim(`day-one-${i}`, 1).ok, true);
  }

  assert.equal(pool.spentTodayUnits, drkToUnits("0.15"));

  assert.equal(pool.claim("day-two-first", 86_401).ok, true);
  assert.equal(pool.spentTodayUnits, CLAIM_AMOUNT_UNITS);
});

test("pause blocks claims and resume restores valid claims", () => {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("1") });

  pool.pause();
  assert.deepEqual(pool.claim("wallet-a", 1), { ok: false, reason: "paused" });

  pool.resume();
  assert.equal(pool.claim("wallet-a", 1).ok, true);
});

test("insufficient custody balance blocks claims even if pool budget remains", () => {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("0.00002") });

  assert.deepEqual(pool.claim("wallet-a", 1), {
    ok: false,
    reason: "insufficient_balance"
  });

  pool.topUp(drkToUnits("0.00001"));
  assert.equal(pool.claim("wallet-a", 2).ok, true);
});

test("top-up increases custody balance without increasing daily allowance", () => {
  const pool = new FaucetPoolModel({ balanceUnits: drkToUnits("0.33") });
  const maxClaims = Number(DAILY_POOL_UNITS / CLAIM_AMOUNT_UNITS);

  pool.topUp(drkToUnits("10"));
  assert.equal(pool.balanceUnits, drkToUnits("10.33"));
  assert.equal(pool.dailyPoolUnits, DAILY_POOL_UNITS);

  for (let i = 0; i < maxClaims; i += 1) {
    assert.equal(pool.claim(`wallet-${i}`, 10_000).ok, true);
  }

  assert.deepEqual(pool.claim("wallet-overflow", 10_000), {
    ok: false,
    reason: "daily_pool_depleted"
  });
});
