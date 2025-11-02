use validator::Validate;
use crate::errors::{AppError, AppResult};

pub fn validate<T: Validate>(item: &T) -> AppResult<()> {
    item.validate()
        .map_err(|e| AppError::Validation(e.to_string()))
}

