//! Request Validation Middleware for API Gateway

use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

/// Request validation middleware
pub struct ValidationMiddleware {
    /// Maximum request body size in bytes
    pub max_body_size: usize,
    /// Require Content-Type header
    pub require_content_type: bool,
}

impl Default for ValidationMiddleware {
    fn default() -> Self {
        Self {
            max_body_size: 10 * 1024 * 1024, // 10MB default
            require_content_type: true,
        }
    }
}

impl ValidationMiddleware {
    pub fn new(max_body_size: usize, require_content_type: bool) -> Self {
        Self {
            max_body_size,
            require_content_type,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ValidationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = ValidationMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ValidationMiddlewareService {
            service: Rc::new(service),
            max_body_size: self.max_body_size,
            require_content_type: self.require_content_type,
        }))
    }
}

pub struct ValidationMiddlewareService<S> {
    service: Rc<S>,
    max_body_size: usize,
    require_content_type: bool,
}

impl<S, B> Service<ServiceRequest> for ValidationMiddlewareService<S>
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
        // Extract all validation info before moving req
        let method_str = req.method().as_str();
        let content_type_str = req
            .headers()
            .get("Content-Type")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        let content_length = req
            .headers()
            .get("Content-Length")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<usize>().ok());

        // Determine if validation should fail and prepare error response
        let error_response = if self.require_content_type
            && matches!(method_str, "POST" | "PUT" | "PATCH")
        {
            if let Some(ref ct_str) = content_type_str {
                if !ct_str.starts_with("application/json") && !ct_str.starts_with("application/") {
                    Some(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Invalid Content-Type",
                        "message": "Content-Type must be application/json or application/*",
                        "received": ct_str
                    })))
                } else {
                    None
                }
            } else {
                Some(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Missing Content-Type header",
                    "message": "Content-Type header is required for this request"
                })))
            }
        } else {
            None
        };

        // Check Content-Length
        let size_error = content_length.and_then(|size| {
            if size > self.max_body_size {
                Some(HttpResponse::PayloadTooLarge().json(serde_json::json!({
                    "error": "Request body too large",
                    "message": format!(
                        "Maximum body size is {} bytes, received {} bytes",
                        self.max_body_size, size
                    ),
                    "max_size": self.max_body_size,
                    "received_size": size
                })))
            } else {
                None
            }
        });

        // Return error if validation failed, otherwise proceed
        if let Some(resp) = error_response.or(size_error) {
            let (req, _) = req.into_parts();
            return Box::pin(
                async move { Ok(ServiceResponse::new(req, resp.map_into_boxed_body())) },
            );
        }

        let service = Rc::clone(&self.service);
        Box::pin(async move {
            let res = service.call(req).await?;
            Ok(res.map_into_boxed_body())
        })
    }
}
