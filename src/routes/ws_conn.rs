use actix::Addr;
use actix_web::{
    web::{self, Data, Payload},
    HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{
    middleware::Authenticated,
    utils::LOBBY_UUID,
    websockets::{ws::WsConn, Lobby},
};

use super::RoomPath;

pub async fn ws_lobby_get(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<Lobby>>,
    auth: Authenticated,
) -> HttpResponse {
    let user_uuid = Uuid::parse_str(&auth.0.sub).unwrap();
    let lobby_room_uuid = Uuid::parse_str(LOBBY_UUID).unwrap();
    let ws = WsConn::new(user_uuid, lobby_room_uuid, srv.get_ref().clone());
    ws::start(ws, &req, stream).unwrap()
}

pub async fn ws_room_get(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<Lobby>>,
    info: web::Path<RoomPath>,
    auth: Authenticated,
) -> HttpResponse {
    let (user_uuid, _, _) = match auth.parse() {
        Some(sucess) => sucess,
        None => return HttpResponse::InternalServerError().finish(),
    };
    let lobby_room_uuid = Uuid::parse_str(info.room_uuid.to_string().as_str()).unwrap();
    let ws = WsConn::new(user_uuid, lobby_room_uuid, srv.get_ref().clone());
    ws::start(ws, &req, stream).unwrap()
}
