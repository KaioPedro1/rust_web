use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{ Error};
use actix_web::{FromRequest, HttpMessage};
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;
use std::future::{ready, Ready};

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::configuration::Jwt;
use crate::model::Claims;

use super::MyError;

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
        let jwt_config = req.app_data::<Data<Jwt>>().unwrap();
        req.cookie("jwt").and_then(|cookie| {
            Some({
                decode::<Claims>(
                    &cookie.value(),
                    &DecodingKey::from_secret(jwt_config.secret_key.as_ref()),
                    &Validation::new(Algorithm::HS256),
                )
                .and_then(|token| Ok(req.extensions_mut().insert(token.claims)))
            })
        });

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
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

impl Authenticated {
    pub fn parse(&self)->Option<(Uuid, String)>{
        let user_id = self.0.sub.clone(); 
   
        match Uuid::parse_str(&user_id){
            Ok(uuid) =>{
                let name = &self.0.name;
                Some((uuid,name.to_string()))
            },
            Err(_) => {
                None
            },
        }
    }
}