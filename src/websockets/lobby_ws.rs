use super::{LobbyNotification, RoomNotification};
use crate::database;
use crate::model::ActionRoomType::Enter;
use crate::model::ConnectionMessage;
use crate::model::MessageLobbyType::Initial;
use crate::model::MessageRoomType::Notification;

use crate::redis_utils::RedisState;
use crate::utils::LOBBY_UUID;
use crate::websockets::messages::{Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix_web::web::Data;

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use uuid::Uuid;

type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,     //self id to self
    rooms: HashMap<Uuid, HashSet<Uuid>>, //room id  to list of users id
    redis: Data<Mutex<RedisState>>,
}

impl Lobby {
    pub fn new(redis: Data<Mutex<RedisState>>) -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            redis,
        }
    }
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
    fn get_lobby_uuid(&self) -> Uuid {
        Uuid::parse_str(LOBBY_UUID).unwrap()
    }
}
impl Lobby {}
impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) -> Self::Result {
        self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);
        self.sessions.insert(msg.self_id, msg.addr);

        if msg.lobby_id == self.get_lobby_uuid() {
            let vec_rooms = self
                .redis
                .lock()
                .unwrap()
                .get_all_rooms_from_redis()
                .unwrap();
            let vec_connections = self
                .redis
                .lock()
                .unwrap()
                .get_all_connections_from_redis()
                .unwrap();

            self.handle(
                LobbyNotification {
                    msg_type: Initial,
                    action: None,
                    room: crate::model::RoomTypes::Rooms(vec_rooms),
                    user: Some(crate::model::UserTypes::Connections(vec_connections)),
                    sender_uuid: msg.self_id,
                },
                ctx,
            )
        } else {
            self.handle(
                RoomNotification {
                    msg_type: Notification,
                    action: Enter,
                    user: msg.self_id,
                    room: msg.lobby_id,
                    redirect: None,
                },
                ctx,
            )
        }
    }
}
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if msg.room_id == self.get_lobby_uuid() {
            if self.sessions.remove(&msg.id).is_some() {
                if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                    lobby.remove(&msg.id);
                }
            }
        } else {  
            let conn_pull = self.redis.lock().unwrap().pg_pool.clone();
            if self.sessions.remove(&msg.id).is_some() {
                if let Some(room) = self.rooms.get_mut(&msg.room_id) {
                    if room.len() > 1 {
                        let mut redis_lock = self.redis.lock().unwrap();
                        redis_lock.remove_connection(msg.id.to_string()+"/"+&msg.room_id.to_string()).unwrap();
                        room.remove(&msg.id);
                        let new_admin = room.iter().next().unwrap().clone();
                        tokio::spawn(async move {
                            database::disconnect_user_and_set_new_admin_if_needed(
                                msg.id,
                                new_admin,
                                msg.room_id,
                                conn_pull,
                            )
                            .await
                            .unwrap();
                        });
                    } else {
                        tokio::spawn(async move {
                            database::delete_room_connections_close_room(msg.room_id, conn_pull)
                                .await
                                .unwrap();
                        });  
                        let mut redis_lock = self.redis.lock().unwrap();
                        room
                            .iter()
                            .for_each(|user_id| {
                                redis_lock.remove_connection(user_id.to_string()+"/"+&msg.room_id.to_string()).unwrap();
                            });
                        
                        let serialized = serde_json::to_string(&LobbyNotification {
                            msg_type: crate::model::MessageLobbyType::Update,
                            action: Some(crate::model::ActionLobbyType::Delete),
                            room: crate::model::RoomTypes::Uuid(msg.room_id),
                            user: Some(crate::model::UserTypes::Uuid(msg.id)),
                            sender_uuid: msg.id
                        })
                        .unwrap();

                        redis_lock
                            .remove_room_publish_to_lobby(msg.room_id.to_string(), serialized)
                            .unwrap();

                        self.rooms.remove(&msg.room_id);
                    }
                }
            }
        }
    }
}

impl Handler<LobbyNotification> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: LobbyNotification, _: &mut Context<Self>) -> Self::Result {
        let serialized_lobby = serde_json::to_string(&msg).unwrap();

        if msg.msg_type == Initial {
            if let Some(user_uuid) = msg.user {
                match user_uuid{
                    crate::model::UserTypes::Connections(_) =>  self.send_message(serialized_lobby.as_str(), &msg.sender_uuid),
                    _=>println!("Invalid user id, check messages type")
                }
            } else {
                println!("Couldn't find user uuid")
            }
        } else {
            match self.rooms.get(&self.get_lobby_uuid()) {
                Some(hset) => {
                    hset.iter()
                        .for_each(|client| self.send_message(serialized_lobby.as_str(), client));
                }
                None => println!("Empty room"),
            }
        }
    }
}
impl Handler<RoomNotification> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: RoomNotification, _: &mut Context<Self>) -> Self::Result {
        match self.rooms.get(&msg.room) {
            Some(hset) => {
                let room_notification_serialized = serde_json::to_string(&msg).unwrap();
                hset.iter().for_each(|client| {
                    self.send_message(room_notification_serialized.as_str(), client)
                });
            }
            None => println!("Empty room"),
        }
    }
}
