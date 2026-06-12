use crate::error::{FaucetPoolError, Result};
use crate::model::{FaucetConfig, FaucetState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimDecision {
    pub day_changed: bool,
    pub next_state: FaucetState,
}

pub fn validate_claim(
    config: &FaucetConfig,
    state: &FaucetState,
    last_claim_at: Option<u64>,
    now_seconds: u64,
    requested_token_id: &[u8],
    requested_amount_units: u64,
) -> Result<ClaimDecision> {
    if state.paused {
        return Err(FaucetPoolError::Paused);
    }

    if requested_token_id != config.token_id {
        return Err(FaucetPoolError::InvalidToken);
    }

    if requested_amount_units != config.claim_amount_units {
        return Err(FaucetPoolError::InvalidAmount);
    }

    if let Some(last_claim_at) = last_claim_at {
        let cooldown_until = last_claim_at
            .checked_add(config.cooldown_seconds)
            .ok_or(FaucetPoolError::InvalidAmount)?;
        if now_seconds < cooldown_until {
            return Err(FaucetPoolError::CooldownActive);
        }
    }

    let current_day = crate::time::day_id(now_seconds);
    let mut next_state = state.clone();
    let day_changed = current_day != state.current_day;

    if day_changed {
        next_state.current_day = current_day;
        next_state.spent_today_units = 0;
    }

    let next_spent_today_units = next_state
        .spent_today_units
        .checked_add(config.claim_amount_units)
        .ok_or(FaucetPoolError::InvalidAmount)?;

    if next_spent_today_units > config.daily_pool_units {
        return Err(FaucetPoolError::DailyPoolDepleted);
    }

    if next_state.custody_balance_units < config.claim_amount_units {
        return Err(FaucetPoolError::InsufficientCustody);
    }

    next_state.spent_today_units = next_spent_today_units;
    next_state.custody_balance_units = next_state
        .custody_balance_units
        .checked_sub(config.claim_amount_units)
        .ok_or(FaucetPoolError::InsufficientCustody)?;

    Ok(ClaimDecision { day_changed, next_state })
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLAIM: u64 = 30_000;
    const DAILY: u64 = 330_000_000;
    const COOLDOWN: u64 = 43_200;

    #[test]
    fn valid_claim_spends_daily_pool_and_custody() {
        let config = config();
        let state = state(false, 0, 0, CLAIM * 2);

        let decision =
            validate_claim(&config, &state, None, 1_000, b"drk", CLAIM).expect("valid claim");

        assert_eq!(decision.next_state.spent_today_units, CLAIM);
        assert_eq!(decision.next_state.custody_balance_units, CLAIM);
    }

    #[test]
    fn duplicate_claim_inside_cooldown_is_rejected() {
        let config = config();
        let state = state(false, 0, 0, CLAIM * 2);

        let error = validate_claim(&config, &state, Some(1_000), 1_001, b"drk", CLAIM)
            .expect_err("cooldown should reject");

        assert_eq!(error, FaucetPoolError::CooldownActive);
    }

    #[test]
    fn new_day_resets_spent_without_carry_over() {
        let config = config();
        let state = state(false, 0, DAILY - CLAIM, CLAIM * 2);

        let decision = validate_claim(&config, &state, None, 86_400, b"drk", CLAIM)
            .expect("new day should reset spent");

        assert!(decision.day_changed);
        assert_eq!(decision.next_state.current_day, 1);
        assert_eq!(decision.next_state.spent_today_units, CLAIM);
    }

    #[test]
    fn top_up_does_not_expand_daily_pool() {
        let config = config();
        let state = state(false, 0, DAILY, DAILY);

        let error = validate_claim(&config, &state, None, 1_000, b"drk", CLAIM)
            .expect_err("daily pool should reject despite custody");

        assert_eq!(error, FaucetPoolError::DailyPoolDepleted);
    }

    fn config() -> FaucetConfig {
        FaucetConfig {
            admin_public_key: vec![1],
            custody_public_key: vec![2],
            token_id: b"drk".to_vec(),
            claim_amount_units: CLAIM,
            daily_pool_units: DAILY,
            cooldown_seconds: COOLDOWN,
        }
    }

    fn state(
        paused: bool,
        current_day: u64,
        spent_today_units: u64,
        custody_balance_units: u64,
    ) -> FaucetState {
        FaucetState { current_day, spent_today_units, custody_balance_units, paused }
    }
}
