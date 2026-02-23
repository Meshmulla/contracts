use soroban_sdk::Env;

use crate::storage::get_discharge_plan;
use crate::types::Error;

/// Validate that expected discharge date is after admission date
pub fn validate_dates(
    _env: &Env,
    admission_date: u64,
    expected_discharge_date: u64,
) -> Result<(), Error> {
    if expected_discharge_date <= admission_date {
        return Err(Error::InvalidDates);
    }
    Ok(())
}

/// Validate that a discharge plan exists
pub fn validate_plan_exists(env: &Env, plan_id: u64) -> Result<(), Error> {
    get_discharge_plan(env, plan_id)?;
    Ok(())
}
