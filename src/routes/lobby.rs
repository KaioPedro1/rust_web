use actix_web::{
    http::{header::{ContentType, LOCATION}},
    web::{self}, HttpRequest, HttpResponse, routes, get,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::database;

//mover essa função para o model e validar os cookie por tipo
async fn validate_cookie_uuid(req:HttpRequest, conn:web::Data<PgPool>)->bool{
    let cookie_uuid = match req.cookie("uuid") {
        Some(c) => c,
        None => {println!{"No cookie found"}; return false},
    };
    let user_uuid = match Uuid::parse_str(cookie_uuid.value()) {
        Ok(u) => u,
        Err(e) => {println!{"Invalid Uuid{:?}", e}; return false},
    };
    match database::check_user_id_db(user_uuid, conn).await {
        Ok(_) => return true,
        Err(e) => {println!("Uuid doe snot exist in bd {:?}", e);return false},
    };
}
#[routes(GET,POST)]
#[get("/lobby")]
async fn lobby_get(req: HttpRequest, connection: web::Data<PgPool>) -> HttpResponse {
    if validate_cookie_uuid(req, connection).await {
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(include_str!("../../static/lobby.html"))
    }else{
        HttpResponse::TemporaryRedirect()
            .content_type(ContentType::html())
            .append_header((LOCATION, "/"))
        .body(include_str!("../../static/index.html"))
    }
}

#[derive(Deserialize)]
struct Info {
    room_id: String,
}

#[get("/lobby/{room_id}")] 
async fn lobby_room_get(info: web::Path<Info>) -> String {
    format!(
        "Welcome {}",
        info.room_id
    )
}