"use strict";

const UNITS_PER_DRK = 100_000_000n;
const CLAIM_AMOUNT_UNITS = 3_000n;
const DAILY_POOL_UNITS = 33_000_000n;
const COOLDOWN_SECONDS = 12 * 60 * 60;

function drkToUnits(value) {
  if (!/^\d+(\.\d{1,8})?$/.test(value)) {
    throw new Error(`Invalid DRK amount: ${value}`);
  }

  const [whole, fraction = ""] = value.split(".");
  return BigInt(whole) * UNITS_PER_DRK + BigInt(fraction.padEnd(8, "0"));
}

function unitsToDrk(units) {
  const sign = units < 0n ? "-" : "";
  const abs = units < 0n ? -units : units;
  const whole = abs / UNITS_PER_DRK;
  const fraction = String(abs % UNITS_PER_DRK).padStart(8, "0").replace(/0+$/, "");
  return `${sign}${whole}${fraction ? `.${fraction}` : ""}`;
}

function dayId(timestampSeconds, timezoneOffsetSeconds = 0) {
  return Math.floor((timestampSeconds + timezoneOffsetSeconds) / 86_400);
}

class FaucetPoolModel {
  constructor(options = {}) {
    this.claimAmountUnits = options.claimAmountUnits ?? CLAIM_AMOUNT_UNITS;
    this.dailyPoolUnits = options.dailyPoolUnits ?? DAILY_POOL_UNITS;
    this.cooldownSeconds = options.cooldownSeconds ?? COOLDOWN_SECONDS;
    this.timezoneOffsetSeconds = options.timezoneOffsetSeconds ?? 0;
    this.balanceUnits = options.balanceUnits ?? 0n;
    this.currentDay = null;
    this.spentTodayUnits = 0n;
    this.lastClaimAt = new Map();
    this.paused = false;
  }

  topUp(amountUnits) {
    assertNonNegativeUnits(amountUnits);
    this.balanceUnits += amountUnits;
  }

  pause() {
    this.paused = true;
  }

  resume() {
    this.paused = false;
  }

  refreshDay(timestampSeconds) {
    const nextDay = dayId(timestampSeconds, this.timezoneOffsetSeconds);
    if (this.currentDay === null || this.currentDay !== nextDay) {
      this.currentDay = nextDay;
      this.spentTodayUnits = 0n;
    }
  }

  claim(recipient, timestampSeconds) {
    if (!recipient || typeof recipient !== "string") {
      return reject("invalid_recipient");
    }

    this.refreshDay(timestampSeconds);

    if (this.paused) {
      return reject("paused");
    }

    const lastClaim = this.lastClaimAt.get(recipient);
    if (lastClaim !== undefined && timestampSeconds - lastClaim < this.cooldownSeconds) {
      return reject("cooldown");
    }

    if (this.spentTodayUnits + this.claimAmountUnits > this.dailyPoolUnits) {
      return reject("daily_pool_depleted");
    }

    if (this.balanceUnits < this.claimAmountUnits) {
      return reject("insufficient_balance");
    }

    this.balanceUnits -= this.claimAmountUnits;
    this.spentTodayUnits += this.claimAmountUnits;
    this.lastClaimAt.set(recipient, timestampSeconds);

    return {
      ok: true,
      amountUnits: this.claimAmountUnits,
      spentTodayUnits: this.spentTodayUnits,
      balanceUnits: this.balanceUnits
    };
  }
}

function reject(reason) {
  return { ok: false, reason };
}

function assertNonNegativeUnits(value) {
  if (typeof value !== "bigint" || value < 0n) {
    throw new Error("Amount must be a non-negative bigint");
  }
}

module.exports = {
  COOLDOWN_SECONDS,
  CLAIM_AMOUNT_UNITS,
  DAILY_POOL_UNITS,
  FaucetPoolModel,
  UNITS_PER_DRK,
  dayId,
  drkToUnits,
  unitsToDrk
};
