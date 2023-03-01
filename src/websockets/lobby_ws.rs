use crate::game_logic::game_actor_messages::GameStart;
use crate::game_logic::{GameActor, Player};
use crate::model::ActionRoomType::Update;
use crate::model::MessageLobbyType::Initial;
use crate::model::{MessageRoomType, UserTypes};
use crate::redis_utils::RedisState;
use crate::utils::LOBBY_UUID;
use crate::{database, model};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix::{Addr, AsyncContext};
use actix_web::web::Data;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::{
    Connect, Disconnect, GameSocketInput, LobbyNotification, RoomNotification, UserRoomType,
    WsMessage,
};

type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,     //self id to self
    rooms: HashMap<Uuid, HashSet<Uuid>>, //room id  to list of users id
    redis: Data<Mutex<RedisState>>,
    games_initialized: HashMap<Uuid, Addr<GameActor>>,
}

impl Lobby {
    pub fn new(redis: Data<Mutex<RedisState>>) -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            redis,
            games_initialized: HashMap::new(),
        }
    }
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
    fn get_lobby_uuid(&self) -> Uuid {
        Uuid::parse_str(LOBBY_UUID).unwrap()
    }
    fn remove_room_and_game_initialized(&mut self, room_id: &Uuid) {
        if self.games_initialized.contains_key(room_id) {
            self.games_initialized.remove(room_id);
        }
        self.rooms.remove(room_id);
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
            //todo: send initial state of the room for the user
            let serialized_con = self
                .redis
                .lock()
                .unwrap()
                .get_coonections_by_room_id(msg.lobby_id)
                .unwrap();

            self.handle(
                RoomNotification {
                    msg_type: MessageRoomType::RoomNotification,
                    action: Update,
                    user: UserRoomType::UserVec(serialized_con),
                    room: msg.lobby_id,
                    redirect: None,
                },
                ctx,
            );
        }
    }
}
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Context<Self>) {
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
                        room.remove(&msg.id);
                        let new_admin = *room.iter().next().unwrap();
                        let redis_help = Arc::clone(&self.redis);
                        let self_addr = ctx.address();
                        tokio::spawn(async move {
                            let mut new_admin_needed: Option<UserTypes> = None;
                            if let Ok(()) = database::disconnect_user_and_set_new_admin_if_needed(
                                msg.id,
                                new_admin,
                                msg.room_id,
                                conn_pull,
                            )
                            .await
                            {
                                new_admin_needed = Some(model::UserTypes::Uuid(new_admin))
                            };

                            let mut mutable_redis = redis_help.lock().unwrap();
                            let lobby_notification = serde_json::to_string(&LobbyNotification {
                                msg_type: crate::model::MessageLobbyType::UpdatePlayer,
                                action: Some(crate::model::ActionLobbyType::Leave),
                                room: model::RoomTypes::Uuid(msg.room_id),
                                user: new_admin_needed.clone(),
                                sender_uuid: msg.id,
                            })
                            .unwrap();
                            mutable_redis.remove_connection_publish_user(
                                msg.clone(),
                                new_admin_needed,
                                lobby_notification,
                            );
                            let serialized_con =mutable_redis
                                .get_coonections_by_room_id(msg.room_id)
                                .unwrap();
                            let room_notification = RoomNotification {
                                msg_type: MessageRoomType::RoomNotification,
                                action: Update,
                                user: UserRoomType::UserVec(serialized_con),
                                room: msg.room_id,
                                redirect: None
                            };
                            self_addr.do_send(room_notification);
                        });
                    } else {
                        tokio::spawn(async move {
                            database::delete_room_connections_close_room(msg.room_id, conn_pull)
                                .await
                                .unwrap();
                        });
                        let mut redis_lock = self.redis.lock().unwrap();
                        room.iter().for_each(|user_id| {
                            redis_lock
                                .remove_connection(
                                    user_id.to_string() + "/" + &msg.room_id.to_string(),
                                )
                                .unwrap();
                        });

                        let serialized = serde_json::to_string(&LobbyNotification {
                            msg_type: crate::model::MessageLobbyType::UpdateRoom,
                            action: Some(crate::model::ActionLobbyType::Delete),
                            room: crate::model::RoomTypes::Uuid(msg.room_id),
                            user: Some(crate::model::UserTypes::Uuid(msg.id)),
                            sender_uuid: msg.id,
                        })
                        .unwrap();

                        redis_lock
                            .remove_room_publish_to_lobby(msg.room_id.to_string(), serialized)
                            .unwrap();

                        drop(redis_lock);
                        self.remove_room_and_game_initialized(&msg.room_id);
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
                match user_uuid {
                    crate::model::UserTypes::Connections(_) => {
                        self.send_message(serialized_lobby.as_str(), &msg.sender_uuid)
                    }
                    _ => println!("Invalid user id, check messages type"),
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
                    self.send_message(room_notification_serialized.as_str(), client);
                });
            }
            None => println!("Empty room"),
        }
    }
}

impl Handler<GameSocketInput> for Lobby {
    type Result = ();
    fn handle(&mut self, msg: GameSocketInput, _: &mut Self::Context) -> Self::Result {
        let r = self.rooms.get(&msg.room);

        match msg.action {
            super::GameSocketAction::StartGame => {
                if let Some(players_in_room) = r {
                    if players_in_room.len() >= 2 {
                        match self
                            .redis
                            .lock()
                            .unwrap()
                            .get_connection_by_id(msg.room, msg.user)
                        {
                            Ok(ms) => {
                                //verifica se o jogo já foi iniciado
                                if ms.is_admin && !self.games_initialized.contains_key(&msg.room) {
                                    //cria um novo actor para gerenciar o jogo
                                    let mut vecd: VecDeque<Player> = VecDeque::new();
                                    for (index, p) in players_in_room.iter().enumerate() {
                                        let addr = self.sessions.get(p).unwrap();
                                        let player = Player::new(
                                            *p,
                                            (index % 2).try_into().unwrap(),
                                            addr.clone(),
                                        );
                                        vecd.push_back(player);
                                    }
                                    let act = GameActor::new(vecd).start();
                                    self.games_initialized.insert(ms.room_id, act.clone());
                                    act.do_send(GameStart {
                                        teste: "HUE".to_string(),
                                    });
                                    println!("Game started for room {:?}", self.games_initialized);
                                }
                                //usuario n é admin ou jogo ja´foi iniciado, tratar no futuro
                            }
                            Err(e) => println!("{:?}", e),
                        };
                    }
                }
            }
            super::GameSocketAction::PlayerInput => match self.games_initialized.get(&msg.room) {
                Some(game) => {
                    game.do_send(msg);
                }
                None => println!("Game not initialized"),
            },
        }
    }
}
