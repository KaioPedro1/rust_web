use actix_files as fs;
use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header::{ContentType, LOCATION},
    web::{self}, Error, HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{model::{User, UserName}, database};


pub fn validade_and_build(form: FormData) -> Result<User, String> {
    let name = UserName::parse(form.name)?;
    let id =  Uuid::new_v4();
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

pub async fn root_post(form: web::Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse { 
    match validade_and_build(form.0) {
        Ok(register) =>{ 
            database::insert_user_db(&register, connection).await;
            let url_to_redirect = "/lobby";
            let uuid_cookie = Cookie::build("uuid", register.id.to_string())
                .path(url_to_redirect)
                .max_age(Duration::hours(60))
                .finish();
            let name_cookie = Cookie::build("name", register.name.as_ref())
                .path(url_to_redirect)
                .max_age(Duration::hours(60))
                .finish();
            HttpResponse::Found()
                .content_type(ContentType::html())
                .append_header((LOCATION, url_to_redirect))
                .cookie(uuid_cookie)
                .cookie(name_cookie)
                .finish()
        },
        Err(_) => return HttpResponse::BadRequest().finish(),
    }
}

