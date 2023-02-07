use actix_web::{
    error,
    http::{header::LOCATION, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

//error customizado mover isso aqui para algum lugar
#[derive(Debug, Display, Error)]
pub enum MyError {
    #[display(fmt = "Invalid token, unauthorized!")]
    Unauthorized,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .append_header((LOCATION, "/"))
            .finish()
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::Unauthorized => StatusCode::FOUND,
        }
    }
}
