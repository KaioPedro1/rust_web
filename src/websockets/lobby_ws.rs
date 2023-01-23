use crate::model::AvailableRooms;
use crate::utils::LOBBY_UUID;
use crate::websockets::messages::{Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix_web::web::Data;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::{EchoAvailableRoomsLobby, RoomNotification};

type Socket = Recipient<WsMessage>;

#[derive(Serialize)]
struct RoomsState {
    available_rooms_state: Data<Arc<Mutex<Vec<AvailableRooms>>>>,
}

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,     //self id to self
    rooms: HashMap<Uuid, HashSet<Uuid>>, //room id  to list of users id
    rooms_state: RoomsState,
}

impl Lobby {
    pub fn new(available_rooms_state: Data<Arc<Mutex<Vec<AvailableRooms>>>>) -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            rooms_state: RoomsState {
                available_rooms_state,
            },
        }
    }
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}
use crate::model::MessageType::Notification;
use crate::model::ActionType::Enter;
impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) -> Self::Result {
        // create a room if necessary, and then add the id to it
        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);
        // store the address
        self.sessions.insert(msg.self_id, msg.addr);

        println!("{:?}",self.sessions);
        //send initial message
        if msg.lobby_id.to_string() == LOBBY_UUID {
            let serialized_rooms =
                serde_json::to_string(&self.rooms_state.available_rooms_state).unwrap();
            self.send_message(serialized_rooms.as_str(), &msg.self_id);
        }else{
           self.handle(RoomNotification{ msg_type: Notification, action: Enter, user: msg.self_id, room: msg.lobby_id, redirect: None }, ctx)
        }
        }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1  {
                    lobby.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}
impl Handler<EchoAvailableRoomsLobby> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: EchoAvailableRoomsLobby, _: &mut Context<Self>) -> Self::Result {
        let serialized_rooms =
            serde_json::to_string(&self.rooms_state.available_rooms_state).unwrap();

        match self.rooms.get(&msg.lobby_id) {
            Some(hset) => {
                hset.iter()
                    .for_each(|client| self.send_message(serialized_rooms.as_str(), client));
            }
            None => println!("Empty room"),
        }
    }
}

impl Handler<RoomNotification> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: RoomNotification, _: &mut Context<Self>) -> Self::Result {
        match self.rooms.get(&msg.room) {
            Some(hset) => {
                let room_notification_serialized = serde_json::to_string(&msg).unwrap();
                hset.iter()
                    .for_each(|client| self.send_message(room_notification_serialized.as_str(), client));
            }
            None => println!("Empty room"),
        }
    }
}
