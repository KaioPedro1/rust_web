use actix::Addr;
use actix_web::{HttpRequest, web::{Payload, Data, self}, HttpResponse};
use actix_web_actors::ws;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{websockets::{Lobby, WsConn}, utils::{check_if_cookie_is_valid, LOBBY_UUID}, middleware::Authenticated};

use super::RoomPath;





pub async fn ws_lobby_get(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<Lobby>>,
    auth:Authenticated
) -> HttpResponse {
    let user_uuid=  Uuid::parse_str(&auth.0.sub).unwrap();
    let lobby_room_uuid = Uuid::parse_str(LOBBY_UUID).unwrap();
    let ws = WsConn::new(
        user_uuid,
        lobby_room_uuid,
        srv.get_ref().clone(),
    );
    ws::start(ws, &req, stream).unwrap()
}

pub async fn ws_room_get(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<Lobby>>,
    conn: Data<PgPool>,
    info: web::Path<RoomPath>,
) -> HttpResponse {
    let (user_uuid,_) = match check_if_cookie_is_valid(&req, conn.clone()).await {
        Ok(uuid) => uuid,
        Err(e) => return e,
    };
    let lobby_room_uuid = Uuid::parse_str(info.room_uuid.to_string().as_str()).unwrap();
    let ws = WsConn::new(
        user_uuid,
        lobby_room_uuid,
        srv.get_ref().clone(),
    );
    ws::start(ws, &req, stream).unwrap()
}
