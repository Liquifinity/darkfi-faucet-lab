use crate::error::{FaucetPoolError, Result};
use crate::model::{MoneyOutputAuth, PrivateClaimAuthorization};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoneyFlow {
    TopUp {
        token_id: Vec<u8>,
        amount_units: u64,
        custody_public_key: Vec<u8>,
        custody_output: MoneyOutputAuth,
    },
    Claim {
        token_id: Vec<u8>,
        amount_units: u64,
        custody_public_key: Vec<u8>,
        transfer_call_index: u32,
        authorization: PrivateClaimAuthorization,
    },
    EmergencyWithdraw {
        recipient_address: Vec<u8>,
        token_id: Vec<u8>,
        amount_units: u64,
        custody_public_key: Vec<u8>,
        recipient_output: MoneyOutputAuth,
        change_outputs: Vec<MoneyOutputAuth>,
    },
}

pub fn verify_money_flow(_flow: MoneyFlow) -> Result<()> {
    // DarkFi port:
    // - inspect child ContractCall;
    // - require native Money contract id;
    // - require Money::Transfer function code;
    // - for top-up, prove funds enter contract-controlled custody;
    // - for claim, prove exact configured amount exits to the recipient;
    // - for emergency withdraw, prove admin-authorized transfer amount.
    Err(FaucetPoolError::MoneyTransferNotVerified)
}

#[cfg(feature = "darkfi")]
pub fn verify_money_flow_darkfi(
    flow: MoneyFlow,
    call_idx: usize,
    calls: &[darkfi_sdk::dark_tree::DarkLeaf<darkfi_sdk::ContractCall>],
) -> Result<()> {
    use darkfi_money_contract::{model::MoneyTransferParamsV1, MoneyFunction};
    use darkfi_sdk::{
        crypto::{FuncId, FuncRef, MONEY_CONTRACT_ID},
    };
    use darkfi_serial::deserialize;

    let faucet_call = calls.get(call_idx).ok_or(FaucetPoolError::MoneyTransferNotVerified)?;
    let top_up_mode = matches!(&flow, MoneyFlow::TopUp { .. });

    let mut transfer_matches = calls.iter().enumerate().filter(|(idx, leaf)| {
        if *idx == call_idx {
            return false;
        }

        let is_money_transfer = leaf.data.contract_id == *MONEY_CONTRACT_ID
            && leaf.data.data.first().copied() == Some(MoneyFunction::TransferV1 as u8);

        if top_up_mode {
            is_money_transfer && leaf.parent_index.is_none()
        } else {
            is_money_transfer
                && leaf.parent_index == Some(call_idx)
                && faucet_call.children_indexes.contains(idx)
        }
    });

    let (transfer_idx, transfer_child) =
        transfer_matches.next().ok_or(FaucetPoolError::MoneyTransferNotVerified)?;
    if transfer_matches.next().is_some() {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    let transfer_call = &transfer_child.data;

    let params: MoneyTransferParamsV1 =
        deserialize(&transfer_call.data[1..]).map_err(|_| FaucetPoolError::Serialization)?;
    if params.inputs.is_empty() || params.outputs.is_empty() {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    match flow {
        MoneyFlow::TopUp {
            token_id,
            amount_units,
            custody_public_key,
            custody_output,
        } => {
            if amount_units == 0 {
                return Err(FaucetPoolError::InvalidAmount);
            }
            verify_custody_output_coin(
                &params,
                &custody_output,
                &token_id,
                amount_units,
                &custody_public_key,
                FuncRef {
                    contract_id: faucet_call.data.contract_id,
                    func_code: crate::model::FaucetPoolFunction::Claim as u8,
                }
                .to_func_id(),
            )?;
        }
        MoneyFlow::Claim {
            token_id,
            amount_units,
            custody_public_key,
            transfer_call_index,
            authorization,
        } => {
            if amount_units == 0 {
                return Err(FaucetPoolError::InvalidAmount);
            }
            verify_private_claim_authorization(
                &params,
                &token_id,
                amount_units,
                &custody_public_key,
                transfer_call_index,
                transfer_idx,
                &authorization,
                faucet_call.data.contract_id,
            )?;
        }
        MoneyFlow::EmergencyWithdraw {
            token_id,
            amount_units,
            custody_public_key,
            recipient_output,
            change_outputs,
            ..
        } => {
            if amount_units == 0 {
                return Err(FaucetPoolError::InvalidAmount);
            }
            verify_output_coin(&params, &recipient_output, &token_id, amount_units, FuncId::none())?;
            verify_change_outputs(
                &params,
                &change_outputs,
                &token_id,
                &custody_public_key,
                faucet_call.data.contract_id,
            )?;
        }
    }

    Ok(())
}

#[cfg(feature = "darkfi")]
fn verify_private_claim_authorization(
    params: &darkfi_money_contract::model::MoneyTransferParamsV1,
    token_id: &[u8],
    amount_units: u64,
    custody_public_key: &[u8],
    transfer_call_index: u32,
    actual_transfer_idx: usize,
    authorization: &PrivateClaimAuthorization,
    contract_id: darkfi_sdk::crypto::ContractId,
) -> Result<()> {
    // Faucet claim proofs are supplied as ContractCallImport proofs and are
    // verified by the DarkFi runtime against FaucetClaim metadata public inputs.
    // The payload-level authorization field must stay empty so callers cannot
    // smuggle unverified proof material into contract state validation.
    if !authorization.public_inputs.is_empty() || !authorization.proof.is_empty() {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    if transfer_call_index as usize != actual_transfer_idx {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    if token_id.len() != 32 || amount_units == 0 || custody_public_key.len() != 32 {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    let input_user_data_enc = params.inputs[0].user_data_enc;
    for input in &params.inputs[1..] {
        if input.user_data_enc != input_user_data_enc {
            return Err(FaucetPoolError::MoneyTransferNotVerified);
        }
    }

    if params.outputs.len() != 2 {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    let expected_custody_spend_hook = darkfi_sdk::crypto::FuncRef {
        contract_id,
        func_code: crate::model::FaucetPoolFunction::Claim as u8,
    }
    .to_func_id();

    if expected_custody_spend_hook == darkfi_sdk::crypto::FuncId::none() {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    Ok(())
}

#[cfg(feature = "darkfi")]
fn verify_change_outputs(
    params: &darkfi_money_contract::model::MoneyTransferParamsV1,
    outputs: &[MoneyOutputAuth],
    token_id: &[u8],
    custody_public_key: &[u8],
    contract_id: darkfi_sdk::crypto::ContractId,
) -> Result<()> {
    if outputs.len() > 1 {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    for output in outputs {
        if output.value_units == 0 {
            return Err(FaucetPoolError::InvalidAmount);
        }
        verify_custody_output_coin(
            params,
            output,
            token_id,
            output.value_units,
            custody_public_key,
            darkfi_sdk::crypto::FuncRef {
                contract_id,
                func_code: crate::model::FaucetPoolFunction::Claim as u8,
            }
            .to_func_id(),
        )?;
    }

    Ok(())
}

#[cfg(feature = "darkfi")]
fn verify_custody_output_coin(
    params: &darkfi_money_contract::model::MoneyTransferParamsV1,
    auth: &MoneyOutputAuth,
    expected_token_id: &[u8],
    expected_value_units: u64,
    custody_public_key: &[u8],
    expected_spend_hook: darkfi_sdk::crypto::FuncId,
) -> Result<()> {
    if auth.public_key != custody_public_key {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    verify_output_coin(
        params,
        auth,
        expected_token_id,
        expected_value_units,
        expected_spend_hook,
    )
}

#[cfg(feature = "darkfi")]
fn verify_output_coin(
    params: &darkfi_money_contract::model::MoneyTransferParamsV1,
    auth: &MoneyOutputAuth,
    expected_token_id: &[u8],
    expected_value_units: u64,
    expected_spend_hook: darkfi_sdk::crypto::FuncId,
) -> Result<()> {
    use darkfi_money_contract::model::CoinAttributes;
    use darkfi_sdk::pasta::pallas;

    if auth.token_id != expected_token_id || auth.value_units != expected_value_units {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    let output = params
        .outputs
        .get(auth.output_index as usize)
        .ok_or(FaucetPoolError::MoneyTransferNotVerified)?;

    let coin_attrs = CoinAttributes {
        public_key: public_key_from_bytes(&auth.public_key)?,
        value: expected_value_units,
        token_id: token_id_from_bytes(expected_token_id)?,
        spend_hook: expected_spend_hook,
        user_data: pallas::Base::zero(),
        blind: base_blind_from_bytes(&auth.coin_blind)?,
    };

    if output.coin != coin_attrs.to_coin() {
        return Err(FaucetPoolError::MoneyTransferNotVerified);
    }

    Ok(())
}

#[cfg(feature = "darkfi")]
fn public_key_from_bytes(bytes: &[u8]) -> Result<darkfi_sdk::crypto::PublicKey> {
    let bytes: [u8; 32] = bytes.try_into().map_err(|_| FaucetPoolError::InvalidRecipient)?;
    darkfi_sdk::crypto::PublicKey::from_bytes(bytes).map_err(|_| FaucetPoolError::InvalidRecipient)
}

#[cfg(feature = "darkfi")]
fn token_id_from_bytes(bytes: &[u8]) -> Result<darkfi_money_contract::model::TokenId> {
    let bytes: [u8; 32] = bytes.try_into().map_err(|_| FaucetPoolError::InvalidToken)?;
    darkfi_money_contract::model::TokenId::from_bytes(bytes).map_err(|_| FaucetPoolError::InvalidToken)
}

#[cfg(feature = "darkfi")]
fn base_blind_from_bytes(bytes: &[u8]) -> Result<darkfi_sdk::crypto::BaseBlind> {
    use darkfi_sdk::crypto::pasta_prelude::PrimeField;

    let bytes: [u8; 32] = bytes.try_into().map_err(|_| FaucetPoolError::Serialization)?;
    let blind: Option<darkfi_sdk::pasta::pallas::Base> =
        darkfi_sdk::pasta::pallas::Base::from_repr(bytes).into();
    let blind = blind.ok_or(FaucetPoolError::Serialization)?;
    Ok(darkfi_sdk::crypto::Blind(blind))
}
