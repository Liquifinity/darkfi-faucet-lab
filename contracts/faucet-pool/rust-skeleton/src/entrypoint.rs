pub mod admin;
pub mod claim;
pub mod top_up;

use crate::error::Result;
use crate::model::{FaucetPoolFunction, FaucetUpdate};
use crate::storage;

#[cfg(feature = "darkfi")]
use darkfi_sdk::{
    crypto::{ContractId, FuncRef, PublicKey, MONEY_CONTRACT_ID},
    dark_tree::DarkLeaf,
    error::ContractResult,
    pasta::pallas,
    wasm, ContractCall,
};

#[cfg(feature = "darkfi")]
use darkfi_serial::{deserialize, Encodable};

#[cfg(feature = "darkfi")]
const FAUCET_POOL_ZKAS_CLAIM_NS: &str = "FaucetClaim";

#[cfg(feature = "darkfi")]
darkfi_sdk::define_contract!(
    init: darkfi_init_contract,
    exec: darkfi_process_instruction,
    apply: darkfi_process_update,
    metadata: darkfi_get_metadata
);

pub fn init_contract(_payload: &[u8]) -> Result<()> {
    storage::init_trees()
}

pub fn get_metadata(function_code: u8, _payload: &[u8]) -> Result<Vec<u8>> {
    let function = FaucetPoolFunction::try_from(function_code)?;

    match function {
        FaucetPoolFunction::TopUp | FaucetPoolFunction::Claim => Ok(Vec::new()),
        FaucetPoolFunction::Pause
        | FaucetPoolFunction::Resume
        | FaucetPoolFunction::EmergencyWithdraw => Ok(Vec::new()),
    }
}

pub fn process_instruction(function_code: u8, payload: &[u8]) -> Result<Vec<u8>> {
    let function = FaucetPoolFunction::try_from(function_code)?;

    match function {
        FaucetPoolFunction::TopUp => top_up::process(payload),
        FaucetPoolFunction::Claim => claim::process(payload),
        FaucetPoolFunction::Pause => admin::process_pause(payload),
        FaucetPoolFunction::Resume => admin::process_resume(payload),
        FaucetPoolFunction::EmergencyWithdraw => admin::process_emergency_withdraw(payload),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FaucetPoolError;

    #[test]
    fn emergency_withdraw_requires_valid_payload_and_money_flow() {
        assert_eq!(
            get_metadata(FaucetPoolFunction::EmergencyWithdraw as u8, &[]),
            Ok(Vec::new())
        );
        assert_eq!(
            process_instruction(FaucetPoolFunction::EmergencyWithdraw as u8, &[]),
            Err(FaucetPoolError::Serialization)
        );
    }

    #[test]
    fn unknown_function_is_rejected() {
        assert_eq!(process_instruction(0xff, &[]), Err(FaucetPoolError::InvalidFunction));
    }
}

pub fn process_update(update: FaucetUpdate) -> Result<()> {
    match update {
        FaucetUpdate::TopUp(update) => top_up::apply(update),
        FaucetUpdate::Claim(update) => claim::apply(update),
        FaucetUpdate::Pause => admin::apply_pause(),
        FaucetUpdate::Resume => admin::apply_resume(),
        FaucetUpdate::EmergencyWithdraw { amount_units } => admin::apply_emergency_withdraw(amount_units),
    }
}

#[cfg(feature = "darkfi")]
fn darkfi_init_contract(cid: ContractId, ix: &[u8]) -> ContractResult {
    wasm::db::zkas_db_set(&include_bytes!("../proof/faucet-claim.zk.bin")[..])?;
    storage::init_trees_darkfi(cid, ix).map_err(Into::into)
}

#[cfg(feature = "darkfi")]
fn darkfi_get_metadata(_cid: ContractId, ix: &[u8]) -> ContractResult {
    let call_idx = wasm::util::get_call_index()? as usize;
    let calls: Vec<DarkLeaf<ContractCall>> = deserialize(ix)?;
    let self_call = &calls[call_idx].data;
    if self_call.data.is_empty() {
        return Err(crate::error::FaucetPoolError::Serialization.into());
    }
    let function = FaucetPoolFunction::try_from(self_call.data[0])?;
    let payload = &self_call.data[1..];
    let metadata = match function {
        FaucetPoolFunction::TopUp => encode_metadata(Vec::new())?,
        FaucetPoolFunction::Claim => claim_metadata_darkfi(_cid, call_idx, &calls, payload)?,
        FaucetPoolFunction::Pause | FaucetPoolFunction::Resume => {
            let params: crate::model::AdminParams = deserialize(payload)?;
            encode_metadata(vec![public_key_from_bytes(&params.admin_public_key)?])?
        }
        FaucetPoolFunction::EmergencyWithdraw => {
            let params: crate::model::EmergencyWithdrawParams = deserialize(payload)?;
            encode_metadata(vec![public_key_from_bytes(&params.admin_public_key)?])?
        }
    };

    wasm::util::set_return_data(&metadata)
}

#[cfg(feature = "darkfi")]
fn encode_metadata(signature_pubkeys: Vec<PublicKey>) -> Result<Vec<u8>> {
    let zk_public_inputs: Vec<(String, Vec<pallas::Base>)> = vec![];
    encode_metadata_with_zk(zk_public_inputs, signature_pubkeys)
}

#[cfg(feature = "darkfi")]
fn encode_metadata_with_zk(
    zk_public_inputs: Vec<(String, Vec<pallas::Base>)>,
    signature_pubkeys: Vec<PublicKey>,
) -> Result<Vec<u8>> {
    let mut metadata = vec![];
    zk_public_inputs
        .encode(&mut metadata)
        .map_err(|_| crate::error::FaucetPoolError::Serialization)?;
    signature_pubkeys
        .encode(&mut metadata)
        .map_err(|_| crate::error::FaucetPoolError::Serialization)?;
    Ok(metadata)
}

#[cfg(feature = "darkfi")]
fn claim_metadata_darkfi(
    cid: ContractId,
    call_idx: usize,
    calls: &[DarkLeaf<ContractCall>],
    payload: &[u8],
) -> Result<Vec<u8>> {
    use darkfi_money_contract::{model::MoneyTransferParamsV1, MoneyFunction};

    let params: crate::model::ClaimParams =
        deserialize(payload).map_err(|_| crate::error::FaucetPoolError::Serialization)?;
    let config = storage::load_config_darkfi(cid)?;
    calls
        .get(call_idx)
        .ok_or(crate::error::FaucetPoolError::MoneyTransferNotVerified)?;
    let mut transfer_matches = calls.iter().filter(|leaf| {
        leaf.data.contract_id == *MONEY_CONTRACT_ID
            && leaf.data.data.first().copied() == Some(MoneyFunction::TransferV1 as u8)
    });
    let transfer_leaf = transfer_matches
        .next()
        .ok_or(crate::error::FaucetPoolError::MoneyTransferNotVerified)?;
    if transfer_matches.next().is_some() {
        return Err(crate::error::FaucetPoolError::MoneyTransferNotVerified);
    }

    let transfer_params: MoneyTransferParamsV1 =
        deserialize(&transfer_leaf.data.data[1..])
            .map_err(|_| crate::error::FaucetPoolError::Serialization)?;
    if transfer_params.inputs.is_empty() || transfer_params.outputs.len() != 2 {
        return Err(crate::error::FaucetPoolError::MoneyTransferNotVerified);
    }

    let input_user_data_enc = transfer_params.inputs[0].user_data_enc;
    for input in &transfer_params.inputs[1..] {
        if input.user_data_enc != input_user_data_enc {
            return Err(crate::error::FaucetPoolError::MoneyTransferNotVerified);
        }
    }

    let token_id = token_id_base_from_bytes(&params.token_id)?;
    let custody_public_key = public_key_from_bytes(&config.custody_public_key)?;
    let (custody_x, custody_y) = custody_public_key.xy();
    let custody_spend_hook = FuncRef {
        contract_id: cid,
        func_code: FaucetPoolFunction::Claim as u8,
    }
    .to_func_id();

    let zk_public_inputs = vec![(
        FAUCET_POOL_ZKAS_CLAIM_NS.to_string(),
        vec![
            input_user_data_enc,
            transfer_params.outputs[0].coin.inner(),
            pallas::Base::from(params.amount_units),
            token_id,
            transfer_params.outputs[1].coin.inner(),
            custody_spend_hook.inner(),
            custody_x,
            custody_y,
        ],
    )];

    encode_metadata_with_zk(zk_public_inputs, Vec::new())
}

#[cfg(feature = "darkfi")]
fn public_key_from_bytes(bytes: &[u8]) -> Result<PublicKey> {
    let key_bytes: [u8; 32] = bytes
        .try_into()
        .map_err(|_| crate::error::FaucetPoolError::Unauthorized)?;
    PublicKey::from_bytes(key_bytes).map_err(|_| crate::error::FaucetPoolError::Unauthorized)
}

#[cfg(feature = "darkfi")]
fn token_id_base_from_bytes(bytes: &[u8]) -> Result<pallas::Base> {
    let token_bytes: [u8; 32] = bytes
        .try_into()
        .map_err(|_| crate::error::FaucetPoolError::InvalidToken)?;
    let token_id = darkfi_money_contract::model::TokenId::from_bytes(token_bytes)
        .map_err(|_| crate::error::FaucetPoolError::InvalidToken)?;
    Ok(token_id.inner())
}

#[cfg(feature = "darkfi")]
fn darkfi_process_instruction(_cid: ContractId, ix: &[u8]) -> ContractResult {
    let call_idx = wasm::util::get_call_index()? as usize;
    let calls: Vec<DarkLeaf<ContractCall>> = deserialize(ix)?;
    let self_call = &calls[call_idx].data;
    if self_call.data.is_empty() {
        return Err(crate::error::FaucetPoolError::Serialization.into());
    }
    let func = FaucetPoolFunction::try_from(self_call.data[0])?;
    let update = match func {
        FaucetPoolFunction::TopUp => top_up::process_darkfi(_cid, call_idx, &calls, &self_call.data[1..])?,
        FaucetPoolFunction::Claim => claim::process_darkfi(_cid, call_idx, &calls, &self_call.data[1..])?,
        FaucetPoolFunction::Pause => admin::process_pause_darkfi(_cid, &self_call.data[1..])?,
        FaucetPoolFunction::Resume => admin::process_resume_darkfi(_cid, &self_call.data[1..])?,
        FaucetPoolFunction::EmergencyWithdraw => {
            admin::process_emergency_withdraw_darkfi(_cid, call_idx, &calls, &self_call.data[1..])?
        }
    };

    wasm::util::set_return_data(&update)
}

#[cfg(feature = "darkfi")]
fn darkfi_process_update(_cid: ContractId, update_data: &[u8]) -> ContractResult {
    process_update_bytes(_cid, update_data).map_err(Into::into)
}

#[cfg(feature = "darkfi")]
fn process_update_bytes(cid: ContractId, update_data: &[u8]) -> Result<()> {
    use crate::model::{ClaimUpdate, FaucetState, TopUpUpdate};
    use darkfi_sdk::msg;

    msg!("[FaucetPool] apply update len={}", update_data.len());

    if update_data.is_empty() {
        msg!("[FaucetPool] apply rejected: empty update");
        return Err(crate::error::FaucetPoolError::Serialization);
    }

    let function = FaucetPoolFunction::try_from(update_data[0])?;
    msg!("[FaucetPool] apply function={:?}", function);

    match function {
        FaucetPoolFunction::TopUp => {
            msg!("[FaucetPool] top_up apply payload len={}", update_data[1..].len());
            let update: TopUpUpdate =
                darkfi_serial::deserialize(&update_data[1..]).map_err(|_| crate::error::FaucetPoolError::Serialization)?;
            msg!("[FaucetPool] top_up apply amount_units={}", update.amount_units);
            top_up::apply_darkfi(cid, update)
        }
        FaucetPoolFunction::Claim => {
            let update: ClaimUpdate =
                darkfi_serial::deserialize(&update_data[1..]).map_err(|_| crate::error::FaucetPoolError::Serialization)?;
            claim::apply_darkfi(cid, update)
        }
        FaucetPoolFunction::Pause => {
            let state: FaucetState =
                darkfi_serial::deserialize(&update_data[1..]).map_err(|_| crate::error::FaucetPoolError::Serialization)?;
            admin::apply_pause_darkfi(cid, state)
        }
        FaucetPoolFunction::Resume => {
            let state: FaucetState =
                darkfi_serial::deserialize(&update_data[1..]).map_err(|_| crate::error::FaucetPoolError::Serialization)?;
            admin::apply_resume_darkfi(cid, state)
        }
        FaucetPoolFunction::EmergencyWithdraw => {
            let state: FaucetState =
                darkfi_serial::deserialize(&update_data[1..]).map_err(|_| crate::error::FaucetPoolError::Serialization)?;
            admin::apply_emergency_withdraw_darkfi(cid, state)
        }
    }
}
