//! Rust-shaped FaucetPool contract skeleton.
//!
//! This module documents the intended DarkFi contract layout without claiming
//! deploy readiness. It must be wired into the upstream DarkFi workspace and
//! verified against the Money contract before testnet deployment.

pub mod entrypoint;
pub mod error;
pub mod money;
pub mod model;
pub mod policy;
pub mod storage;
pub mod time;

pub const CONTRACT_NAME: &str = "darkfi_testnet_faucet_pool";
pub const DB_VERSION: u32 = 1;

pub const TREE_INFO: &str = "info";
pub const TREE_CONFIG: &str = "config";
pub const TREE_STATE: &str = "state";
pub const TREE_LAST_CLAIM: &str = "last_claim";
pub const TREE_CUSTODY: &str = "custody";

pub const KEY_DB_VERSION: &[u8] = b"db_version";
pub const KEY_ADMIN_PUBLIC_KEY: &[u8] = b"admin_public_key";
pub const KEY_TOKEN_ID: &[u8] = b"token_id";
pub const KEY_CLAIM_AMOUNT_UNITS: &[u8] = b"claim_amount_units";
pub const KEY_DAILY_POOL_UNITS: &[u8] = b"daily_pool_units";
pub const KEY_COOLDOWN_SECONDS: &[u8] = b"cooldown_seconds";
pub const KEY_CURRENT_DAY: &[u8] = b"current_day";
pub const KEY_SPENT_TODAY_UNITS: &[u8] = b"spent_today_units";
pub const KEY_PAUSED: &[u8] = b"paused";
pub const KEY_CUSTODY_BALANCE_UNITS: &[u8] = b"custody_balance_units";
pub const KEY_CONFIG: &[u8] = b"faucet_config";
pub const KEY_STATE: &[u8] = b"faucet_state";
