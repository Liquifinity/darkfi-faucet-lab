"use strict";

const assert = require("node:assert/strict");
const test = require("node:test");
const { drkToUnits } = require("../faucet-pool/pool-model");
const { TopUpGuardModel } = require("./guard-model");

test("replenish only when FaucetPool is below minimum", () => {
  const guard = guardModel();

  assert.deepEqual(
    guard.replenish(replenishParams({ faucetPoolBalanceUnits: drkToUnits("0.20") })),
    { ok: false, reason: "pool_above_minimum" }
  );

  const result = guard.replenish(replenishParams({ faucetPoolBalanceUnits: drkToUnits("0.05") }));

  assert.equal(result.ok, true);
  assert.equal(result.amountUnits, drkToUnits("0.10"));
});

test("replenish rejects wrong destination and token", () => {
  const guard = guardModel();

  assert.deepEqual(
    guard.replenish(replenishParams({ destinationContractId: "attacker" })),
    { ok: false, reason: "invalid_destination" }
  );

  assert.deepEqual(
    guard.replenish(replenishParams({ tokenId: "not-drk" })),
    { ok: false, reason: "invalid_token" }
  );
});

test("replenish cannot exceed configured target balance", () => {
  const guard = guardModel();

  assert.deepEqual(
    guard.replenish(
      replenishParams({
        faucetPoolBalanceUnits: drkToUnits("0.09"),
        amountUnits: drkToUnits("0.25")
      })
    ),
    { ok: false, reason: "target_exceeded" }
  );
});

test("daily limit resets without carry-over", () => {
  const guard = guardModel({ dailyReplenishLimitUnits: drkToUnits("0.15") });

  assert.equal(
    guard.replenish(replenishParams({ amountUnits: drkToUnits("0.10"), timestampSeconds: 1 })).ok,
    true
  );
  assert.deepEqual(
    guard.replenish(replenishParams({ amountUnits: drkToUnits("0.10"), timestampSeconds: 2 })),
    { ok: false, reason: "daily_limit_exceeded" }
  );

  assert.equal(
    guard.replenish(
      replenishParams({ amountUnits: drkToUnits("0.10"), timestampSeconds: 86_401 })
    ).ok,
    true
  );
  assert.equal(guard.releasedTodayUnits, drkToUnits("0.10"));
});

test("pause blocks replenish", () => {
  const guard = guardModel();

  guard.pause();
  assert.deepEqual(guard.replenish(replenishParams()), { ok: false, reason: "paused" });

  guard.resume();
  assert.equal(guard.replenish(replenishParams()).ok, true);
});

test("reserve balance is enforced", () => {
  const guard = guardModel({ reserveBalanceUnits: drkToUnits("0.05") });

  assert.deepEqual(
    guard.replenish(replenishParams({ amountUnits: drkToUnits("0.10") })),
    { ok: false, reason: "insufficient_reserve" }
  );

  assert.equal(guard.deposit(drkToUnits("0.20")).ok, true);
  assert.equal(guard.replenish(replenishParams({ amountUnits: drkToUnits("0.10") })).ok, true);
});

function guardModel(overrides = {}) {
  return new TopUpGuardModel({
    faucetPoolContractId: "faucet-pool",
    tokenId: "drk",
    minPoolBalanceUnits: drkToUnits("0.10"),
    targetPoolBalanceUnits: drkToUnits("0.33"),
    dailyReplenishLimitUnits: drkToUnits("0.33"),
    reserveBalanceUnits: drkToUnits("1"),
    ...overrides
  });
}

function replenishParams(overrides = {}) {
  return {
    destinationContractId: "faucet-pool",
    tokenId: "drk",
    faucetPoolBalanceUnits: drkToUnits("0.05"),
    amountUnits: drkToUnits("0.10"),
    timestampSeconds: 1_000,
    ...overrides
  };
}
