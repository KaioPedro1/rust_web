use actix::prelude::{Message, Recipient};
use serde::Serialize;
use uuid::Uuid;

use crate::model::{MessageRoomType, ActionRoomType, MessageLobbyType, RoomsInitialState, ConnectionsInitialState};

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
    pub msg_type: MessageRoomType,
    pub action: ActionRoomType,
    pub user: Uuid,
    pub room: Uuid,
    pub redirect: Option<String>
}


#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct NotifyInitialLobbyState{
    pub msg_type: MessageLobbyType,
    pub rooms:Vec<RoomsInitialState>,
    pub users:Vec<ConnectionsInitialState>
}