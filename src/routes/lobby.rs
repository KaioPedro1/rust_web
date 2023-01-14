use actix_web::{
    http::header::{ContentType, LOCATION},
    web, Error, HttpRequest, HttpResponse,
};

pub fn lobby_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/lobby")
            .route(web::get().to(lobby_get)),
    );
}
//post method

//TODO: verifica se uuid existe no banco de dados pelo cookie recebido
async fn lobby_get(req: HttpRequest) -> Result<HttpResponse, Error> {
    match req.cookie("uuid") {
        Some(_) => 
            Ok(HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(include_str!("../../static/lobby.html"))),
        None => 
            Ok(HttpResponse::Found()
                .content_type(ContentType::html())
                .append_header((LOCATION, "/"))
                .body(include_str!("../../static/index.html"))),
    }
}
