/* eslint-disable no-console */

type PayoutMode = "wallet" | "contract";

function assertContractPayoutReady(): never {
  throw new Error("Contract payout execution is not implemented");
}

function main(): void {
  const payoutMode = (process.env.FAUCET_PAYOUT_MODE ?? "wallet") as PayoutMode;

  if (payoutMode === "contract") {
    assertContractPayoutReady();
  }

  console.log("Faucet worker is running in fail-closed mode.");
}

main();

// Contract payout execution is not implemented.
// The production worker must only switch to contract payouts after proof gates pass.
// payoutMode === "contract"
