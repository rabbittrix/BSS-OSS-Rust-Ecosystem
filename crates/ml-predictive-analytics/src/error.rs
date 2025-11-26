//! Error types for ML Predictive Analytics

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MlPredictiveError {
    #[error("Invalid input data: {0}")]
    InvalidInput(String),

    #[error("Model not trained: {0}")]
    ModelNotTrained(String),

    #[error("Training failed: {0}")]
    TrainingFailed(String),

    #[error("Prediction failed: {0}")]
    PredictionFailed(String),

    #[error("Data processing error: {0}")]
    DataProcessing(String),

    #[error("Model serialization error: {0}")]
    Serialization(String),
}
