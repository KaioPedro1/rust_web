use actix_files as fs;
use actix_web::{
    cookie::{time::Duration, Cookie},
    http::header::{ContentType, LOCATION},
    web::{self}, Error, HttpResponse,
};
use uuid::Uuid;
use crate::model::{NewRegistration, UserName};
//cofigurar as rotas
pub fn config_root(cfg: &mut web::ServiceConfig) {
    cfg.service( 
        web::resource("/")
            .route(web::get().to(root_get))
            .route(web::post().to(root_post))
    );
}
//validar entrada do usuario
pub fn validade_and_build(form: FormData) -> Result<NewRegistration, String> {
    let name = UserName::parse(form.name)?;
    Ok(NewRegistration { name })
}

//deserializar entrada do usuario pelo fomrulário
#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    pub name: String,
}
//rota get
async fn root_get() -> Result<fs::NamedFile, Error> {
    let file = fs::NamedFile::open_async("./static/index.html")
        .await?
        .use_last_modified(true)
        .use_etag(true);
    Ok(file)
}
//rota post TODO: inserir no banco de dados o uuid e o nome
async fn root_post(form: web::Form<FormData>) -> HttpResponse {    
    match validade_and_build(form.0) {
        Ok(register) => register,
        Err(_) => return HttpResponse::BadRequest().finish(),
    }; 

    let url_to_redirect = "/lobby";
    let id = Uuid::new_v4().to_string();

    HttpResponse::Found()
            .content_type(ContentType::html())
            .append_header((LOCATION, url_to_redirect))
            .cookie(
                Cookie::build("uuid", id)
                    .path(url_to_redirect)
                    .max_age(Duration::hours(60))
                    .http_only(true)
                    .finish(),
                )
            .finish()
}
