use crate::error::{FaucetPoolError, Result};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaucetPoolFunction {
    TopUp = 0x00,
    Claim = 0x01,
    Pause = 0x02,
    Resume = 0x03,
    EmergencyWithdraw = 0x04,
}

impl TryFrom<u8> for FaucetPoolFunction {
    type Error = FaucetPoolError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0x00 => Ok(Self::TopUp),
            0x01 => Ok(Self::Claim),
            0x02 => Ok(Self::Pause),
            0x03 => Ok(Self::Resume),
            0x04 => Ok(Self::EmergencyWithdraw),
            _ => Err(FaucetPoolError::InvalidFunction),
        }
    }
}

#[cfg_attr(feature = "darkfi", derive(darkfi_serial::SerialEncodable, darkfi_serial::SerialDecodable))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaucetConfig {
    pub admin_public_key: Vec<u8>,
    pub custody_public_key: Vec<u8>,
    pub token_id: Vec<u8>,
    pub claim_amount_units: u64,
    pub daily_pool_units: u64,
    pub cooldown_seconds: u64,
}

#[cfg_attr(feature = "darkfi", derive(darkfi_serial::SerialEncodable, darkfi_serial::SerialDecodable))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaucetState {
    pub current_day: u64,
    pub spent_today_units: u64,
    pub custody_balance_units: u64,
    pub paused: bool,
}

#[cfg_attr(feature = "darkfi", derive(darkfi_serial::SerialEncodable, darkfi_serial::SerialDecodable))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopUpParams {
    pub token_id: Vec<u8>,
    pub amount_units: u64,
    pub custody_output: MoneyOutputAuth,
}

#[cfg_attr(feature = "darkfi", derive(darkfi_serial::SerialEncodable, darkfi_serial::SerialDecodable))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopUpUpdate {
    pub token_id: Vec<u8>,
    pub amount_units: u64,
    pub next_state: FaucetState,
}

#[cfg_attr(feature = "darkfi", derive(darkfi_serial::SerialEncodable, darkfi_serial::SerialDecodable))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimParams {
    pub recipient_identity_commitment: Vec<u8>,
    pub token_id: Vec<u8>,
    pub amount_units: u64,
    pub transfer_call_index: u32,
    pub authorization: PrivateClaimAuthorization,
}

#[cfg_attr(feature = "darkfi", derive(darkfi_serial::SerialEncodable, darkfi_serial::SerialDecodable))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimUpdate {
    pub recipient_identity_commitment: Vec<u8>,
    pub claim_day: u64,
    pub claim_time_seconds: u64,
    pub amount_units: u64,
    pub next_state: FaucetState,
}

#[cfg_attr(feature = "darkfi", derive(darkfi_serial::SerialEncodable, darkfi_serial::SerialDecodable))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminParams {
    pub admin_public_key: Vec<u8>,
}

#[cfg_attr(feature = "darkfi", derive(darkfi_serial::SerialEncodable, darkfi_serial::SerialDecodable))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmergencyWithdrawParams {
    pub admin_public_key: Vec<u8>,
    pub recipient_address: Vec<u8>,
    pub amount_units: u64,
    pub recipient_output: MoneyOutputAuth,
    pub change_outputs: Vec<MoneyOutputAuth>,
}

#[cfg_attr(feature = "darkfi", derive(darkfi_serial::SerialEncodable, darkfi_serial::SerialDecodable))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoneyOutputAuth {
    pub output_index: u32,
    pub public_key: Vec<u8>,
    pub token_id: Vec<u8>,
    pub value_units: u64,
    pub coin_blind: Vec<u8>,
}

#[cfg_attr(feature = "darkfi", derive(darkfi_serial::SerialEncodable, darkfi_serial::SerialDecodable))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateClaimAuthorization {
    pub public_inputs: Vec<u8>,
    pub proof: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FaucetUpdate {
    TopUp(TopUpUpdate),
    Claim(ClaimUpdate),
    Pause,
    Resume,
    EmergencyWithdraw { amount_units: u64 },
}
