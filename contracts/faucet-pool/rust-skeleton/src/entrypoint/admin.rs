use crate::error::{FaucetPoolError, Result};

#[cfg(feature = "darkfi")]
use crate::money::MoneyFlow;

#[cfg(feature = "darkfi")]
use crate::model::{EmergencyWithdrawParams, FaucetState};

#[cfg(feature = "darkfi")]
use crate::storage;

#[cfg(feature = "darkfi")]
use darkfi_serial::{deserialize, serialize};

pub fn process_pause(_payload: &[u8]) -> Result<Vec<u8>> {
    Err(FaucetPoolError::Serialization)
}

pub fn process_resume(_payload: &[u8]) -> Result<Vec<u8>> {
    Err(FaucetPoolError::Serialization)
}

pub fn process_emergency_withdraw(_payload: &[u8]) -> Result<Vec<u8>> {
    Err(FaucetPoolError::Serialization)
}

pub fn apply_pause() -> Result<()> {
    Ok(())
}

pub fn apply_resume() -> Result<()> {
    Ok(())
}

pub fn apply_emergency_withdraw(_amount_units: u64) -> Result<()> {
    Ok(())
}

#[cfg(feature = "darkfi")]
pub fn process_pause_darkfi(
    cid: darkfi_sdk::crypto::ContractId,
    payload: &[u8],
) -> Result<Vec<u8>> {
    require_admin_darkfi(cid, payload)?;
    let mut state = storage::load_state_darkfi(cid)?;
    state.paused = true;
    Ok(serialize(&state))
}

#[cfg(feature = "darkfi")]
pub fn process_resume_darkfi(
    cid: darkfi_sdk::crypto::ContractId,
    payload: &[u8],
) -> Result<Vec<u8>> {
    require_admin_darkfi(cid, payload)?;
    let mut state = storage::load_state_darkfi(cid)?;
    state.paused = false;
    Ok(serialize(&state))
}

#[cfg(feature = "darkfi")]
pub fn apply_pause_darkfi(cid: darkfi_sdk::crypto::ContractId, state: FaucetState) -> Result<()> {
    storage::save_state_darkfi(cid, &state)
}

#[cfg(feature = "darkfi")]
pub fn apply_resume_darkfi(cid: darkfi_sdk::crypto::ContractId, state: FaucetState) -> Result<()> {
    storage::save_state_darkfi(cid, &state)
}

#[cfg(feature = "darkfi")]
fn require_admin_darkfi(cid: darkfi_sdk::crypto::ContractId, payload: &[u8]) -> Result<()> {
    let params: crate::model::AdminParams =
        deserialize(payload).map_err(|_| FaucetPoolError::Serialization)?;
    require_admin_key_darkfi(cid, &params.admin_public_key)
}

#[cfg(feature = "darkfi")]
fn require_admin_key_darkfi(
    cid: darkfi_sdk::crypto::ContractId,
    admin_public_key: &[u8],
) -> Result<()> {
    let config = storage::load_config_darkfi(cid)?;

    if admin_public_key != config.admin_public_key {
        return Err(FaucetPoolError::Unauthorized);
    }

    Ok(())
}

#[cfg(feature = "darkfi")]
pub fn process_emergency_withdraw_darkfi(
    cid: darkfi_sdk::crypto::ContractId,
    call_idx: usize,
    calls: &[darkfi_sdk::dark_tree::DarkLeaf<darkfi_sdk::ContractCall>],
    payload: &[u8],
) -> Result<Vec<u8>> {
    let params: EmergencyWithdrawParams =
        deserialize(payload).map_err(|_| FaucetPoolError::Serialization)?;
    require_admin_key_darkfi(cid, &params.admin_public_key)?;

    if params.amount_units == 0 {
        return Err(FaucetPoolError::InvalidAmount);
    }

    let config = storage::load_config_darkfi(cid)?;
    let state = storage::load_state_darkfi(cid)?;
    if state.custody_balance_units < params.amount_units {
        return Err(FaucetPoolError::InsufficientCustody);
    }

    crate::money::verify_money_flow_darkfi(
        MoneyFlow::EmergencyWithdraw {
            recipient_address: params.recipient_address,
            token_id: config.token_id,
            amount_units: params.amount_units,
            custody_public_key: config.custody_public_key,
            recipient_output: params.recipient_output,
            change_outputs: params.change_outputs,
        },
        call_idx,
        calls,
    )?;

    let mut next_state = state;
    next_state.custody_balance_units = next_state
        .custody_balance_units
        .checked_sub(params.amount_units)
        .ok_or(FaucetPoolError::InsufficientCustody)?;

    Ok(serialize(&next_state))
}

#[cfg(feature = "darkfi")]
pub fn apply_emergency_withdraw_darkfi(
    cid: darkfi_sdk::crypto::ContractId,
    state: FaucetState,
) -> Result<()> {
    storage::save_state_darkfi(cid, &state)
}