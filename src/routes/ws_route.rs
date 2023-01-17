use std::sync::{ Arc, Mutex};

use actix::Addr;
use actix_web::{get, HttpRequest, web::{Payload, Data}, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{websockets::{Lobby, WsConn}, model::AvailableRooms};


//TODO: melhorar error handling do retorno da funcao
#[get("/lobby/ws")]
async fn ws_connection(req: HttpRequest, stream: Payload, srv: Data<Addr<Lobby>>, rooms: Data<Arc<Mutex<Vec<AvailableRooms>>>>)-> HttpResponse{
    let cookie_uuid = match req.cookie("uuid") {
        Some(c) => c,
        None => {println!{"No cookie found"}; return HttpResponse::Conflict().finish()},
    };
    let user_uuid = match Uuid::parse_str(cookie_uuid.value()) {
        Ok(u) => u,
        Err(e) => {println!{"Invalid Uuid{:?}", e}; return HttpResponse::Conflict().finish()},
    };
    println!("{:#?}", rooms.get_ref());
    let ws = WsConn::new(
        user_uuid,
        srv.get_ref().clone(),
        rooms.as_ref().clone()
    );
    ws::start(ws, &req, stream).unwrap()
}