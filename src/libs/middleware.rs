use std::{
    future::{ready, Ready},
    pin::Pin,
};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error,
};
use futures_util::Future;

use crate::libs::{structs::AppState, utils::validate_api_key};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let app_state = req.app_data::<Data<AppState>>().unwrap();
        let headers = req.headers();

        let mut auth_success = app_state.api_keys.is_empty();
        if headers.contains_key("Authorization") && !app_state.api_keys.is_empty() {
            let key = headers.get("Authorization").unwrap().to_str().unwrap();
            auth_success = validate_api_key(app_state, &key.to_string());
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            if auth_success {
                let res = fut.await?;
                Ok(res)
            } else {
                let error = actix_web::error::ErrorUnauthorized("Unauthorized");
                Err(error)
            }
        })
    }
}
