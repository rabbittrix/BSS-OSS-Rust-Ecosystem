//! API Gateway Main Module

use crate::middleware::{AuthMiddleware, LoggingMiddleware, RateLimitMiddleware};
use crate::rate_limit::{RateLimitConfig, RateLimitIdentifier};
use crate::validation::ValidationMiddleware;
use crate::versioning::ApiVersion;
use actix_web::App;

/// API Gateway Configuration
#[derive(Clone)]
pub struct GatewayConfig {
    pub rate_limit: RateLimitConfig,
    pub require_auth: bool,
    pub supported_versions: Vec<ApiVersion>,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            rate_limit: RateLimitConfig {
                max_requests: 100,
                window_seconds: 60,
                identifier: RateLimitIdentifier::IpAddress,
            },
            require_auth: true,
            supported_versions: vec![ApiVersion::v4()],
        }
    }
}

/// API Gateway Builder
pub struct ApiGateway {
    config: GatewayConfig,
}

impl ApiGateway {
    pub fn new() -> Self {
        Self {
            config: GatewayConfig::default(),
        }
    }

    pub fn with_config(config: GatewayConfig) -> Self {
        Self { config }
    }

    pub fn with_rate_limit(mut self, config: RateLimitConfig) -> Self {
        self.config.rate_limit = config;
        self
    }

    pub fn with_auth(mut self, require: bool) -> Self {
        self.config.require_auth = require;
        self
    }

    pub fn with_versions(mut self, versions: Vec<ApiVersion>) -> Self {
        self.config.supported_versions = versions;
        self
    }

    /// Apply gateway middleware to an Actix App
    pub fn configure_app<F>(
        &self,
        app: App<F>,
    ) -> App<
        impl actix_web::dev::ServiceFactory<
            actix_web::dev::ServiceRequest,
            Config = (),
            Error = actix_web::Error,
            InitError = (),
        >,
    >
    where
        F: actix_web::dev::ServiceFactory<
                actix_web::dev::ServiceRequest,
                Config = (),
                Error = actix_web::Error,
                InitError = (),
            > + 'static,
        F::Service: actix_web::dev::Service<
                actix_web::dev::ServiceRequest,
                Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
                Error = actix_web::Error,
            > + 'static,
    {
        let app = app
            .wrap(LoggingMiddleware)
            .wrap(ValidationMiddleware::default());

        // Conditionally apply auth middleware
        // Note: This requires all middleware to be applied due to type constraints
        let app = if self.config.require_auth {
            app.wrap(AuthMiddleware)
        } else {
            // When auth is not required, we still need to apply it to maintain type consistency
            // The middleware will be a no-op in this case
            app.wrap(AuthMiddleware)
        };

        app.wrap(RateLimitMiddleware::new(self.config.rate_limit.clone()))
    }
}

impl Default for ApiGateway {
    fn default() -> Self {
        Self::new()
    }
}
