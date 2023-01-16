use actix_web::{
    http::{header::{ContentType, LOCATION}},
    web::{self}, HttpRequest, HttpResponse, routes,
};
use sqlx::PgPool;
use uuid::Uuid;


async fn validate_cookie_uuid(req:HttpRequest, conn:web::Data<PgPool>)->bool{
    let cookie_uuid = match req.cookie("uuid") {
        Some(c) => c,
        None => {println!{"No cookie found"}; return false},
    };
    let user_uuid = match Uuid::parse_str(cookie_uuid.value()) {
        Ok(u) => u,
        Err(e) => {println!{"Invalid Uuid{:?}", e}; return false},
    };
    match check_user_id_db(user_uuid, conn).await {
        Ok(_) => return true,
        Err(e) => {println!("Uuid doe snot exist in bd {:?}", e);return false},
    };
}
#[routes(GET)]
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
struct UserId {
    name: String,
}
async fn check_user_id_db(
    user_uuid: Uuid,
    connection: web::Data<PgPool>,
) -> Result<UserId, sqlx::Error> {
    sqlx::query_as!(UserId, r#"SELECT name FROM users WHERE id = $1"#, user_uuid)
        .fetch_one(connection.get_ref())
        .await
}
