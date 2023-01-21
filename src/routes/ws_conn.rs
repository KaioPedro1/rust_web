use actix::Addr;
use actix_web::{HttpRequest, web::{Payload, Data}, HttpResponse};
use actix_web_actors::ws;
use sqlx::PgPool;

use crate::{websockets::{Lobby, WsConn}, utils::check_if_cookie_is_valid};



pub async fn ws_lobby_get(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<Lobby>>,
    conn: Data<PgPool>,
) -> HttpResponse {
    let user_uuid = match check_if_cookie_is_valid(&req, conn.clone()).await {
        Ok(uuid) => uuid,
        Err(e) => return e,
    };

    let ws = WsConn::new(
        user_uuid,
        srv.get_ref().clone(),
    );
    ws::start(ws, &req, stream).unwrap()
}