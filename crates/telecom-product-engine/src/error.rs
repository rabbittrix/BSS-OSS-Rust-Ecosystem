//! Product engine errors

use thiserror::Error;

pub type ProductResult<T> = Result<T, ProductError>;

#[derive(Debug, Error)]
pub enum ProductError {
    #[error("customer not found: {0}")]
    CustomerNotFound(String),

    #[error("party not found: {0}")]
    PartyNotFound(String),

    #[error("insufficient balance: need {needed}, have {available} {unit}")]
    InsufficientBalance {
        needed: f64,
        available: f64,
        unit: String,
    },

    #[error("payment failed: {0}")]
    PaymentFailed(String),

    #[error("activation failed: {0}")]
    ActivationFailed(String),

    #[error("identity error: {0}")]
    Identity(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
