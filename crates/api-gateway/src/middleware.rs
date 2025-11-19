//! Middleware for API Gateway

use actix_web::body::MessageBody;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
    time::Instant,
};
use tracing::{info, warn};
use uuid::Uuid;

use crate::auth::extract_auth_context;
use crate::rate_limit::{extract_identifier, RateLimitConfig, RateLimiter};

/// Request logging middleware
pub struct LoggingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct LoggingMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let path = req.path().to_string();
        let request_id = Uuid::new_v4();

        // Extract auth context if available
        let auth_context = extract_auth_context(req.request());
        let user_id = auth_context.as_ref().map(|ctx| ctx.user_id.clone());

        // Add request ID to extensions
        req.extensions_mut().insert(request_id);

        let service = Rc::clone(&self.service);
        Box::pin(async move {
            let res = service.call(req).await?;
            let duration = start.elapsed();
            let status = res.status();

            if status.is_success() {
                info!(
                    request_id = %request_id,
                    method = %method,
                    path = %path,
                    status = status.as_u16(),
                    duration_ms = duration.as_millis(),
                    user_id = ?user_id,
                    "Request completed"
                );
            } else {
                warn!(
                    request_id = %request_id,
                    method = %method,
                    path = %path,
                    status = status.as_u16(),
                    duration_ms = duration.as_millis(),
                    user_id = ?user_id,
                    "Request failed"
                );
            }

            Ok(res)
        })
    }
}

/// Rate limiting middleware
pub struct RateLimitMiddleware {
    limiter: RateLimiter,
    config: RateLimitConfig,
}

impl RateLimitMiddleware {
    pub fn new(config: RateLimitConfig) -> Self {
        let limiter = RateLimiter::new(config.clone());
        Self { limiter, config }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddlewareService {
            service: Rc::new(service),
            limiter: self.limiter.clone(),
            config: self.config.clone(),
        }))
    }
}

#[derive(Clone)]
pub struct RateLimitMiddlewareService<S> {
    service: Rc<S>,
    limiter: RateLimiter,
    config: RateLimitConfig,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let identifier = extract_identifier(&req, &self.config);
        let limiter = self.limiter.clone();
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Check rate limit
            match limiter.check(&identifier) {
                Ok(()) => {
                    // Forward the request and convert the response body to BoxBody
                    let res = service.call(req).await?;
                    Ok(res.map_into_boxed_body())
                }
                Err(e) => {
                    let http_resp: HttpResponse = e.into();
                    let (req, _) = req.into_parts();
                    Ok(ServiceResponse::new(req, http_resp.map_into_boxed_body()))
                }
            }
        })
    }
}

/// Authentication middleware
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract and validate auth context
        let auth_context = match crate::auth::validate_token(req.request()) {
            Ok(ctx) => ctx,
            Err(e) => {
                return Box::pin(async { Err(e) });
            }
        };

        // Add auth context to request extensions
        req.extensions_mut().insert(auth_context);

        let service = Rc::clone(&self.service);
        Box::pin(async move { service.call(req).await })
    }
}
