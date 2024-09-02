use std::rc::Rc;

use actix_web::{Error, HttpMessage};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::{LocalBoxFuture, ok, Ready};
use serde_json::json;

use crate::common::models::response_message::ResponseMessage;

pub struct ApiKeyMiddleware {
    admin_key: String,
    user_key: String,
    public_routes: Vec<String>,
}

impl ApiKeyMiddleware {
    pub fn new(admin_key: String, user_key: String, public_routes: Vec<String>) -> Self {
        Self { admin_key, user_key, public_routes }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyMiddleware
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiKeyMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiKeyMiddlewareService {
            service: Rc::new(service),
            admin_key: self.admin_key.clone(),
            user_key: self.user_key.clone(),
            public_routes: self.public_routes.clone(),
        })
    }
}

pub struct ApiKeyMiddlewareService<S> {
    service: Rc<S>,
    admin_key: String,
    user_key: String,
    public_routes: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for ApiKeyMiddlewareService<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let admin_key = self.admin_key.clone();
        let user_key = self.user_key.clone();

        // Skip middleware for /swagger route or /data route or /file route
        for public_route in self.public_routes.clone() {
            if req.path().starts_with(public_route.as_str()) {
                let fut = self.service.call(req);
                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        }

        if req.path().starts_with("/swagger") || req.path().starts_with("/api-docs") || req.path().starts_with("/static") {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let auth_header = req.headers().get("X-API-KEY").and_then(|h| h.to_str().ok());

        let role = match auth_header {
            Some(key) if key == admin_key => Some("admin"),
            Some(key) if key == user_key => Some("user"),
            _ => None,
        };

        if let Some(role) = role {
            req.extensions_mut().insert(role.to_string());
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized(json!(ResponseMessage{message:String::from("Unauthorized")})))
            })
        }
    }
}
