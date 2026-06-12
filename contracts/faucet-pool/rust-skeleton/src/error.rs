#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FaucetPoolError {
    InvalidFunction,
    InvalidToken,
    InvalidAmount,
    InvalidRecipient,
    Paused,
    CooldownActive,
    DailyPoolDepleted,
    InsufficientCustody,
    Unauthorized,
    MoneyTransferNotVerified,
    TimeSourceUnavailable,
    Serialization,
    Storage,
}

pub type Result<T> = core::result::Result<T, FaucetPoolError>;

#[cfg(feature = "darkfi")]
impl From<darkfi_sdk::error::ContractError> for FaucetPoolError {
    fn from(_error: darkfi_sdk::error::ContractError) -> Self {
        Self::Storage
    }
}

#[cfg(feature = "darkfi")]
impl From<FaucetPoolError> for darkfi_sdk::error::ContractError {
    fn from(error: FaucetPoolError) -> Self {
        match error {
            FaucetPoolError::InvalidFunction => Self::InvalidFunction,
            FaucetPoolError::InvalidToken => Self::Custom(1),
            FaucetPoolError::InvalidAmount => Self::Custom(2),
            FaucetPoolError::InvalidRecipient => Self::Custom(3),
            FaucetPoolError::Paused => Self::Custom(4),
            FaucetPoolError::CooldownActive => Self::Custom(5),
            FaucetPoolError::DailyPoolDepleted => Self::Custom(6),
            FaucetPoolError::InsufficientCustody => Self::Custom(7),
            FaucetPoolError::Unauthorized => Self::Custom(8),
            FaucetPoolError::MoneyTransferNotVerified => Self::Custom(9),
            FaucetPoolError::TimeSourceUnavailable => Self::Custom(10),
            FaucetPoolError::Serialization => Self::Custom(11),
            FaucetPoolError::Storage => Self::Custom(12),
        }
    }
}
