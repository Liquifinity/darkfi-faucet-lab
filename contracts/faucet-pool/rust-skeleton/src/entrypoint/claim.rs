use crate::error::{FaucetPoolError, Result};
use crate::money::{verify_money_flow, MoneyFlow};
use crate::model::{ClaimUpdate, PrivateClaimAuthorization};

#[cfg(feature = "darkfi")]
use crate::model::ClaimParams;

#[cfg(feature = "darkfi")]
use crate::{policy, storage, time};

#[cfg(feature = "darkfi")]
use darkfi_serial::{deserialize, serialize};

pub fn process(_payload: &[u8]) -> Result<Vec<u8>> {
    verify_money_flow(MoneyFlow::Claim {
        token_id: Vec::new(),
        amount_units: 0,
        custody_public_key: Vec::new(),
        transfer_call_index: 0,
        authorization: empty_private_claim_authorization(),
    })?;
    Err(FaucetPoolError::Serialization)
}

pub fn apply(_update: ClaimUpdate) -> Result<()> {
    Ok(())
}

#[cfg(feature = "darkfi")]
pub fn process_darkfi(
    cid: darkfi_sdk::crypto::ContractId,
    call_idx: usize,
    calls: &[darkfi_sdk::dark_tree::DarkLeaf<darkfi_sdk::ContractCall>],
    payload: &[u8],
) -> Result<Vec<u8>> {
    let params: ClaimParams = deserialize(payload).map_err(|_| FaucetPoolError::Serialization)?;
    if params.recipient_identity_commitment.is_empty() {
        return Err(FaucetPoolError::InvalidRecipient);
    }

    let config = storage::load_config_darkfi(cid)?;
    let state = storage::load_state_darkfi(cid)?;
    let now_seconds = time::current_time_seconds_darkfi()?;
    let last_claim = storage::load_last_claim_darkfi(cid, &params.recipient_identity_commitment)?;

    let decision = policy::validate_claim(
        &config,
        &state,
        last_claim,
        now_seconds,
        &params.token_id,
        params.amount_units,
    )?;

    crate::money::verify_money_flow_darkfi(
        MoneyFlow::Claim {
            token_id: params.token_id,
            amount_units: params.amount_units,
            custody_public_key: config.custody_public_key,
            transfer_call_index: params.transfer_call_index,
            authorization: params.authorization,
        },
        call_idx,
        calls,
    )?;

    let update = ClaimUpdate {
        recipient_identity_commitment: params.recipient_identity_commitment,
        claim_day: decision.next_state.current_day,
        claim_time_seconds: now_seconds,
        amount_units: config.claim_amount_units,
        next_state: decision.next_state,
    };
    Ok(serialize(&update))
}

fn empty_private_claim_authorization() -> PrivateClaimAuthorization {
    PrivateClaimAuthorization {
        public_inputs: Vec::new(),
        proof: Vec::new(),
    }
}

#[cfg(feature = "darkfi")]
pub fn apply_darkfi(cid: darkfi_sdk::crypto::ContractId, update: ClaimUpdate) -> Result<()> {
    storage::save_state_darkfi(cid, &update.next_state)?;
    storage::save_last_claim_darkfi(
        cid,
        &update.recipient_identity_commitment,
        update.claim_time_seconds,
    )
}
