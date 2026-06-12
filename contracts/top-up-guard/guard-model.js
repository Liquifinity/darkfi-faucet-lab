"use strict";

const { dayId } = require("../faucet-pool/pool-model");

class TopUpGuardModel {
  constructor(options = {}) {
    this.faucetPoolContractId = requiredString(options.faucetPoolContractId, "faucet");
    this.tokenId = requiredString(options.tokenId, "drk");
    this.minPoolBalanceUnits = requiredBigInt(options.minPoolBalanceUnits, 0n);
    this.targetPoolBalanceUnits = requiredBigInt(options.targetPoolBalanceUnits, 0n);
    this.dailyReplenishLimitUnits = requiredBigInt(options.dailyReplenishLimitUnits, 0n);
    this.reserveBalanceUnits = requiredBigInt(options.reserveBalanceUnits, 0n);
    this.timezoneOffsetSeconds = options.timezoneOffsetSeconds ?? 0;
    this.currentDay = null;
    this.releasedTodayUnits = 0n;
    this.paused = false;

    if (this.targetPoolBalanceUnits < this.minPoolBalanceUnits) {
      throw new Error("Target pool balance must be >= minimum pool balance");
    }
  }

  deposit(amountUnits, tokenId = this.tokenId) {
    assertPositiveUnits(amountUnits);
    if (tokenId !== this.tokenId) return reject("invalid_token");
    this.reserveBalanceUnits += amountUnits;
    return { ok: true, reserveBalanceUnits: this.reserveBalanceUnits };
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
      this.releasedTodayUnits = 0n;
    }
  }

  replenish(params) {
    const {
      destinationContractId,
      tokenId,
      faucetPoolBalanceUnits,
      amountUnits,
      timestampSeconds
    } = params;

    this.refreshDay(timestampSeconds);

    if (this.paused) return reject("paused");
    if (destinationContractId !== this.faucetPoolContractId) return reject("invalid_destination");
    if (tokenId !== this.tokenId) return reject("invalid_token");
    assertPositiveUnits(amountUnits);

    if (faucetPoolBalanceUnits >= this.minPoolBalanceUnits) {
      return reject("pool_above_minimum");
    }

    if (faucetPoolBalanceUnits + amountUnits > this.targetPoolBalanceUnits) {
      return reject("target_exceeded");
    }

    if (this.releasedTodayUnits + amountUnits > this.dailyReplenishLimitUnits) {
      return reject("daily_limit_exceeded");
    }

    if (this.reserveBalanceUnits < amountUnits) {
      return reject("insufficient_reserve");
    }

    this.reserveBalanceUnits -= amountUnits;
    this.releasedTodayUnits += amountUnits;

    return {
      ok: true,
      amountUnits,
      reserveBalanceUnits: this.reserveBalanceUnits,
      releasedTodayUnits: this.releasedTodayUnits
    };
  }
}

function reject(reason) {
  return { ok: false, reason };
}

function assertPositiveUnits(value) {
  if (typeof value !== "bigint" || value <= 0n) {
    throw new Error("Amount must be a positive bigint");
  }
}

function requiredBigInt(value, fallback) {
  if (value === undefined) return fallback;
  if (typeof value !== "bigint" || value < 0n) {
    throw new Error("Expected a non-negative bigint");
  }
  return value;
}

function requiredString(value, fallback) {
  if (value === undefined) return fallback;
  if (typeof value !== "string" || value.length === 0) {
    throw new Error("Expected a non-empty string");
  }
  return value;
}

module.exports = { TopUpGuardModel };
