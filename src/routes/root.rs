use actix_web::{get,post, HttpResponse, Error, web};
use actix_files as fs;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[get("/")]
async fn index() -> Result<fs::NamedFile, Error> {
    let file = fs::NamedFile::open_async("./static/index.html")
        .await?
        .use_last_modified(true)
        .use_etag(true);
    Ok(file)
}
#[post("/")]
async fn submit_form(_form: web::Form<FormData>) -> HttpResponse {

    HttpResponse::Ok().body("REEEEEEEEEEE")
}