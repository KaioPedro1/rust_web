use actix_web::{http::header::LOCATION,HttpRequest, HttpResponse, web, Responder};
use actix_files as fs;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{database};

pub const LOBBY_UUID:&str = "57a1396b-ac9d-4558-b356-1bf87246a14f";


pub enum FilesOptions {
    Lobby,
    Room,
}

pub async fn open_file_return_http_response_with_cache(req: &HttpRequest, opt: FilesOptions) -> HttpResponse {
    let file_path = match opt {
        FilesOptions::Lobby => "./static/lobby.html",
        FilesOptions::Room => "./static/room.html",
    };
    match fs::NamedFile::open_async(file_path).await {
        Ok(file) => file.use_last_modified(true).use_etag(true).respond_to(req),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
pub async fn check_if_cookie_is_valid(
    req: &HttpRequest,
    conn: web::Data<PgPool>,
) -> Result<(Uuid,String), HttpResponse> {
    let cookie_uuid = req.cookie("uuid").ok_or(
        HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, "/"))
            .finish(),
    )?;
    let cookie_name = req.cookie("name").ok_or(
        HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, "/"))
            .finish(),
    )?;

    let user_uuid = Uuid::parse_str(cookie_uuid.value()).map_err(|_| {
        HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, "/"))
            .finish()
    })?;
    let name =cookie_name.value();

    match database::check_user_id_db(user_uuid, name,conn).await {
        Ok(_) => Ok((user_uuid,name.to_string())),
        Err(_) => Err(HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, "/"))
            .finish()),
    }
}