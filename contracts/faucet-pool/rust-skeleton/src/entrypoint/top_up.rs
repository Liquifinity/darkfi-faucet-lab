use crate::error::{FaucetPoolError, Result};
use crate::money::{verify_money_flow, MoneyFlow};
use crate::model::{MoneyOutputAuth, TopUpUpdate};

#[cfg(feature = "darkfi")]
use crate::model::TopUpParams;

#[cfg(feature = "darkfi")]
use crate::storage;

#[cfg(feature = "darkfi")]
use darkfi_serial::{deserialize, serialize};

pub fn process(_payload: &[u8]) -> Result<Vec<u8>> {
    verify_money_flow(MoneyFlow::TopUp {
        token_id: Vec::new(),
        amount_units: 0,
        custody_public_key: Vec::new(),
        custody_output: empty_money_output_auth(),
    })?;
    Err(FaucetPoolError::Serialization)
}

pub fn apply(_update: TopUpUpdate) -> Result<()> {
    Ok(())
}

#[cfg(feature = "darkfi")]
pub fn process_darkfi(
    cid: darkfi_sdk::crypto::ContractId,
    call_idx: usize,
    calls: &[darkfi_sdk::dark_tree::DarkLeaf<darkfi_sdk::ContractCall>],
    payload: &[u8],
) -> Result<Vec<u8>> {
    let params: TopUpParams = deserialize(payload).map_err(|_| FaucetPoolError::Serialization)?;
    let config = storage::load_config_darkfi(cid)?;

    if params.token_id != config.token_id {
        return Err(FaucetPoolError::InvalidToken);
    }

    if params.amount_units == 0 {
        return Err(FaucetPoolError::InvalidAmount);
    }

    crate::money::verify_money_flow_darkfi(
        MoneyFlow::TopUp {
            token_id: params.token_id.clone(),
            amount_units: params.amount_units,
            custody_public_key: config.custody_public_key,
            custody_output: params.custody_output,
        },
        call_idx,
        calls,
    )?;

    let mut state = storage::load_state_darkfi(cid)?;
    state.custody_balance_units = state
        .custody_balance_units
        .checked_add(params.amount_units)
        .ok_or(FaucetPoolError::InvalidAmount)?;

    let update = TopUpUpdate {
        token_id: params.token_id,
        amount_units: params.amount_units,
        next_state: state,
    };

    Ok(serialize(&update))
}

fn empty_money_output_auth() -> MoneyOutputAuth {
    MoneyOutputAuth {
        output_index: 0,
        public_key: Vec::new(),
        token_id: Vec::new(),
        value_units: 0,
        coin_blind: Vec::new(),
    }
}

#[cfg(feature = "darkfi")]
pub fn apply_darkfi(cid: darkfi_sdk::crypto::ContractId, update: TopUpUpdate) -> Result<()> {
    darkfi_sdk::msg!(
        "[FaucetPool] top_up saving state custody_after={}",
        update.next_state.custody_balance_units
    );
    storage::save_state_darkfi(cid, &update.next_state)
}