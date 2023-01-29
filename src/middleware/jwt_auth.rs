use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{error, http::StatusCode};
use actix_web::{http::header::ContentType, Error, HttpResponse};
use actix_web::{FromRequest, HttpMessage};
use derive_more::{Display, Error};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::model::Claims;

//MIDDLEWARE
pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddleware { service }))
    }
}

pub struct JwtMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        req.cookie("jwt").and_then(|cookie| {
            Some({
                decode::<Claims>(
                    &cookie.value(),
                    &DecodingKey::from_secret("secret".as_ref()),
                    &Validation::new(Algorithm::HS256),
                )
                .and_then(|token| Ok(req.extensions_mut().insert(token.claims)))
            })
        });

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            println!("Hi from response");
            Ok(res)
        })
    }
}

//Extractor
pub struct Authenticated(pub Claims);

impl FromRequest for Authenticated {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let binding = req.extensions();
        let value = binding.get::<Claims>();

        let result = match value {
            Some(v) => Ok(Authenticated(v.clone())),
            None => {
                let error = Error::from(MyError::Unauthorized);
                Err(error)
            }
        };
        ready(result)
    }
}

//error customizado mover isso aqui para algum lugar 
#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "Invalid token, unauthorized!")]
    Unauthorized,

}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(include_str!("../../static/index.html"))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
