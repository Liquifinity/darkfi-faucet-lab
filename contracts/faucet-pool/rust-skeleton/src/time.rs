use crate::error::{FaucetPoolError, Result};

pub fn current_time_seconds() -> Result<u64> {
    // DarkFi port:
    // - prefer approved block timestamp if contract VM exposes one;
    // - otherwise use block height * configured target block time;
    // - reject if neither source is available.
    Err(FaucetPoolError::TimeSourceUnavailable)
}

#[cfg(feature = "darkfi")]
pub fn current_time_seconds_darkfi() -> Result<u64> {
    let height = darkfi_sdk::wasm::util::get_verifying_block_height()?;
    let target = darkfi_sdk::wasm::util::get_block_target()?;

    Ok((height as u64) * (target as u64))
}

pub fn day_id(timestamp_seconds: u64) -> u64 {
    timestamp_seconds / 86_400
}
