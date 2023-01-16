use actix_files as fs;
use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header::{ContentType, LOCATION},
    web::{self}, Error, HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::model::{NewRegistration, UserName};
use sqlx::types::chrono::Utc;

//validar entrada do usuario
pub fn validade_and_build(form: FormData) -> Result<NewRegistration, String> {
    let name = UserName::parse(form.name)?;
    let id =  Uuid::new_v4();
    Ok(NewRegistration { name, id })
}

//deserializar entrada do usuario pelo fomrulário
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
            insert_user_db(&register, connection).await;
            let url_to_redirect = "/lobby";
            let cookie = Cookie::build("uuid", register.id.to_string())
                .path(url_to_redirect)
                .max_age(Duration::hours(60))
                .http_only(true)
                .finish();
        
            HttpResponse::Found()
                .content_type(ContentType::html())
                .append_header((LOCATION, url_to_redirect))
                .cookie(cookie)
                .finish()
        },
        Err(_) => return HttpResponse::BadRequest().finish(),
    }
}

async fn insert_user_db(new_user: &NewRegistration, connection: web::Data<PgPool>){
    match sqlx::query!(
        r#"INSERT INTO users (id, name, subscribed_at) 
        VALUES ($1, $2, $3)"#,
        new_user.id,
        new_user.name.as_ref(),
        Utc::now()
    )
    .execute(connection.get_ref())
    .await{
        Ok(_)=>println!("Sucess at insertion of user",),
        Err(e)=>{
            println!("Failed to execute query: {}", e);
        }
    };
}