use crate::{
    configuration::Jwt,
    database,
    model::{Claims, User, UserName},
};
use actix_files as fs;
use actix_web::cookie::time::Duration as dr;
use actix_web::{
    cookie::Cookie,
    http::header::{ContentType, LOCATION},
    web::{self, Data},
    Error, HttpResponse,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;

pub fn validade_and_build(form: FormData) -> Result<User, String> {
    let name = UserName::parse(form.name)?;
    let id = Uuid::new_v4();
    Ok(User { name, id })
}

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    pub name: String,
}

pub async fn root_get() -> Result<fs::NamedFile, Error> {
    let file = fs::NamedFile::open_async("./static/index.html")
        .await?
        .use_last_modified(true)
        .use_etag(true);
    Ok(file)
}

pub async fn root_post(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
    jwt_data: Data<Jwt>,
) -> HttpResponse {
    match validade_and_build(form.0) {
        Ok(register) => {
            database::insert_user_db(&register, connection).await;
            let claims = Claims {
                sub: register.id.to_string(),
                name: register.name.0.clone(),
                exp: (Utc::now() + Duration::hours(jwt_data.expiration)).timestamp() as usize,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_data.secret_key.as_ref()),
            )
            .unwrap();

            let url_to_redirect = "/lobby";

            let jwt_cookie = Cookie::build("jwt", token)
                .path(url_to_redirect)
                .max_age(dr::hours(60))
                .finish();

            let uuid_cookie = Cookie::build("uuid", register.id.to_string())
                .path(url_to_redirect)
                .max_age(dr::hours(60))
                .finish();

            let name_cookie = Cookie::build("name", register.name.as_ref())
                .path(url_to_redirect)
                .max_age(dr::hours(60))
                .finish();
            HttpResponse::Found()
                .content_type(ContentType::html())
                .append_header((LOCATION, url_to_redirect))
                .cookie(jwt_cookie)
                .cookie(uuid_cookie)
                .cookie(name_cookie)
                .finish()
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
