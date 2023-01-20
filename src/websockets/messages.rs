use std::sync::{Arc, Mutex};

use actix::prelude::{Message, Recipient};
use uuid::Uuid;

use crate::model::{AvailableRooms};

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
    pub initial_room_state: Arc<Mutex<Vec<AvailableRooms>>>
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: Uuid,
    pub id: Uuid,
}

//client sends this to the lobby for the lobby to echo out.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
    pub rooms_state: Arc<Mutex<Vec<AvailableRooms>>>,
    
}
// TODO: achar onde enfiar essa struct, não faz sentido deixar aqui
#[derive(serde::Deserialize, Debug)]
pub struct UserInput{
    pub name: String,
    pub number_of_players:i32,
}