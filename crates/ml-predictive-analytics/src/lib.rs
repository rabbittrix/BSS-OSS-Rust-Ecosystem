//! Machine Learning Predictive Analytics
//!
//! Provides ML-based predictive analytics capabilities including:
//! - Demand forecasting
//! - Churn prediction
//! - Revenue forecasting
//! - Anomaly detection
//! - Customer lifetime value prediction

pub mod error;
pub mod models;
pub mod predictor;
pub mod training;

pub use error::MlPredictiveError;
pub use models::*;
pub use predictor::PredictiveAnalyticsService;
pub use training::ModelTrainer;
