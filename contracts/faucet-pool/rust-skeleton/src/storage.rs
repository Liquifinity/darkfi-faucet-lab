use crate::error::{FaucetPoolError, Result};
use crate::model::{FaucetConfig, FaucetState};

#[cfg(feature = "darkfi")]
use crate::{DB_VERSION, KEY_CONFIG, KEY_DB_VERSION, KEY_STATE, TREE_CONFIG, TREE_CUSTODY, TREE_INFO, TREE_LAST_CLAIM, TREE_STATE};

#[cfg(feature = "darkfi")]
use darkfi_sdk::{crypto::ContractId, wasm};

#[cfg(feature = "darkfi")]
use darkfi_serial::{deserialize, serialize};

pub fn init_trees() -> Result<()> {
    // DarkFi port:
    // - db_init/db_lookup for info, config, state, last_claim, custody;
    // - store DB version in info;
    // - store deploy-time FaucetConfig in config;
    // - initialize FaucetState with paused=true and zero custody.
    Ok(())
}

#[cfg(feature = "darkfi")]
pub fn init_trees_darkfi(cid: ContractId, payload: &[u8]) -> Result<()> {
    let config: FaucetConfig = deserialize(payload).map_err(|_| FaucetPoolError::Serialization)?;

    let info_db = lookup_or_init(cid, TREE_INFO)?;
    let config_db = lookup_or_init(cid, TREE_CONFIG)?;
    let state_db = lookup_or_init(cid, TREE_STATE)?;
    let _last_claim_db = lookup_or_init(cid, TREE_LAST_CLAIM)?;
    let _custody_db = lookup_or_init(cid, TREE_CUSTODY)?;

    let state = FaucetState {
        current_day: 0,
        spent_today_units: 0,
        custody_balance_units: 0,
        paused: true,
    };

    wasm::db::db_set(info_db, KEY_DB_VERSION, &serialize(&DB_VERSION))?;
    wasm::db::db_set(config_db, KEY_CONFIG, &serialize(&config))?;
    wasm::db::db_set(state_db, KEY_STATE, &serialize(&state))?;

    Ok(())
}

pub fn load_config() -> Result<FaucetConfig> {
    // DarkFi port: read and deserialize FaucetConfig from config tree.
    Err(FaucetPoolError::Storage)
}

#[cfg(feature = "darkfi")]
pub fn load_config_darkfi(cid: ContractId) -> Result<FaucetConfig> {
    let config_db = wasm::db::db_lookup(cid, TREE_CONFIG)?;
    let bytes = wasm::db::db_get(config_db, KEY_CONFIG)?.ok_or(FaucetPoolError::Storage)?;
    deserialize(&bytes).map_err(|_| FaucetPoolError::Serialization)
}

pub fn load_state() -> Result<FaucetState> {
    // DarkFi port: read and deserialize FaucetState from state tree.
    Err(FaucetPoolError::Storage)
}

#[cfg(feature = "darkfi")]
pub fn load_state_darkfi(cid: ContractId) -> Result<FaucetState> {
    let state_db = wasm::db::db_lookup(cid, TREE_STATE)?;
    let bytes = wasm::db::db_get(state_db, KEY_STATE)?.ok_or(FaucetPoolError::Storage)?;
    deserialize(&bytes).map_err(|_| FaucetPoolError::Serialization)
}

pub fn save_state(_state: &FaucetState) -> Result<()> {
    // DarkFi port: serialize and write FaucetState to state tree.
    Ok(())
}

#[cfg(feature = "darkfi")]
pub fn save_state_darkfi(cid: ContractId, state: &FaucetState) -> Result<()> {
    let state_db = wasm::db::db_lookup(cid, TREE_STATE)?;
    wasm::db::db_set(state_db, KEY_STATE, &serialize(state))?;
    Ok(())
}

pub fn load_last_claim(_recipient_identity: &[u8]) -> Result<Option<u64>> {
    // DarkFi port: read last_claim[recipient_identity].
    Ok(None)
}

#[cfg(feature = "darkfi")]
pub fn load_last_claim_darkfi(cid: ContractId, recipient_identity: &[u8]) -> Result<Option<u64>> {
    let last_claim_db = wasm::db::db_lookup(cid, TREE_LAST_CLAIM)?;
    match wasm::db::db_get(last_claim_db, recipient_identity)? {
        Some(bytes) => deserialize(&bytes).map(Some).map_err(|_| FaucetPoolError::Serialization),
        None => Ok(None),
    }
}

pub fn save_last_claim(_recipient_identity: &[u8], _timestamp_seconds: u64) -> Result<()> {
    // DarkFi port: write last_claim[recipient_identity].
    Ok(())
}

#[cfg(feature = "darkfi")]
pub fn save_last_claim_darkfi(
    cid: ContractId,
    recipient_identity: &[u8],
    timestamp_seconds: u64,
) -> Result<()> {
    let last_claim_db = wasm::db::db_lookup(cid, TREE_LAST_CLAIM)?;
    wasm::db::db_set(last_claim_db, recipient_identity, &serialize(&timestamp_seconds))?;
    Ok(())
}

#[cfg(feature = "darkfi")]
fn lookup_or_init(cid: ContractId, tree: &str) -> Result<u32> {
    match wasm::db::db_lookup(cid, tree) {
        Ok(db) => Ok(db),
        Err(_) => Ok(wasm::db::db_init(cid, tree)?),
    }
}
