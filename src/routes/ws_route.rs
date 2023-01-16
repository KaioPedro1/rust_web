use actix::Addr;
use actix_web::{get, HttpRequest, web::{Payload, Data}, HttpResponse};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::websockets::{Lobby, WsConn};


//melhorar error handling do retorno da funcao
#[get("/lobby/ws")]
async fn ws_connection(req: HttpRequest, stream: Payload,srv: Data<Addr<Lobby>>)-> HttpResponse{
    let cookie_uuid = match req.cookie("uuid") {
        Some(c) => c,
        None => {println!{"No cookie found"}; return HttpResponse::Conflict().finish()},
    };
    let user_uuid = match Uuid::parse_str(cookie_uuid.value()) {
        Ok(u) => u,
        Err(e) => {println!{"Invalid Uuid{:?}", e}; return HttpResponse::Conflict().finish()},
    };
    let ws = WsConn::new(
        user_uuid,
        srv.get_ref().clone(),
    );
    ws::start(ws, &req, stream).unwrap()
}