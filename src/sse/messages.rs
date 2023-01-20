use actix::{prelude::Message, Addr};
use uuid::Uuid;

use super::SseClient;
//WsConn responds to this to pipe it through to the actual client
#[derive(Message)]
#[rtype(result = "()")]
pub struct SseMessage(pub String);
//WsConn sends this to a lobby to say "take me out please"
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub client_addr: Addr<SseClient>,
}

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Connect {
    pub client_id: Uuid,
    pub client_addr: Addr<SseClient>,
}

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct SendMsg {
    pub number: u32,
}
