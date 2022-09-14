use std::{future::{ready, Ready}, pin::Pin};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, web::Data, error::ErrorUnauthorized, Either, HttpResponse, ResponseError,
};
use futures_util::{future::LocalBoxFuture, Future};

use crate::libs::structs::AppState;

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
    // type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let app_state = req.app_data::<Data<AppState>>().unwrap();
        let headers = req.headers();

        if !headers.contains_key("Authorization") {
            let res = req.error_response(Error::error_response(&self));
            return Ok(res);
        }
        

        println!("{}", app_state.start_time);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })

    }
}
