use actix::prelude::{Message, Recipient};
use serde::Serialize;
use uuid::Uuid;

use crate::model::{MessageType, ActionType};

#[derive(Message)]
#[rtype(result = "()")]
pub struct EchoAvailableRoomsLobby {
    pub lobby_id: Uuid,
}
//WsConn responds to this to pipe it through to the actual client
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub lobby_id: Uuid,
    pub self_id: Uuid,
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: Uuid,
    pub id: Uuid,
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct RoomNotification {
    pub msg_type: MessageType,
    pub action: ActionType,
    pub user: Uuid,
    pub room: Uuid,
    pub redirect: Option<String>
}