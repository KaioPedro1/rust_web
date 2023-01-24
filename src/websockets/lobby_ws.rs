use super::{EchoAvailableRoomsLobby, NotifyInitialLobbyState, RoomNotification};
use crate::database;
use crate::model::ActionRoomType::Enter;
use crate::model::MessageLobbyType::Initial;
use crate::model::MessageRoomType::Notification;
use crate::model::{AvailableRooms, ConnectionsInitialState, RoomsInitialState};
use crate::utils::LOBBY_UUID;
use crate::websockets::messages::{Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix_web::web::Data;
use serde::Serialize;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

type Socket = Recipient<WsMessage>;

#[derive(Serialize)]
struct RoomsState {
    available_rooms_state: Data<Arc<Mutex<Vec<AvailableRooms>>>>,
}

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,     //self id to self
    rooms: HashMap<Uuid, HashSet<Uuid>>, //room id  to list of users id
    rooms_state: RoomsState,
    connection_pool: Data<PgPool>,
}

impl Lobby {
    pub fn new(
        available_rooms_state: Data<Arc<Mutex<Vec<AvailableRooms>>>>,
        connection_pool: Data<PgPool>,
    ) -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            rooms_state: RoomsState {
                available_rooms_state,
            },
            connection_pool,
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

impl Actor for Lobby {
    type Context = Context<Self>;
}

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

        //send initial message
        if msg.lobby_id == self.get_lobby_uuid() {
            let serialized_rooms =
                serde_json::to_string(&self.rooms_state.available_rooms_state).unwrap();
            self.send_message(serialized_rooms.as_str(), &msg.self_id);
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

/// Handler for Disconnect message.
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
            let conn_pull = self.connection_pool.clone();
            if self.sessions.remove(&msg.id).is_some() {
                if let Some(room) = self.rooms.get_mut(&msg.room_id) {
                    if room.len() > 1 {
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
                        /*TODO: REFATORAR NO FUTURO
                        caso só exista uma pessoa na sala e ela desconecte, faça o seguinte
                        1)remova a sala e as conexões do banco de dados
                        2)remova as sala do map que contem todas as salas criadas
                        3)remova a sala do array de estados
                        4)notifique a lobby principal
                         */
                        tokio::spawn(async move {
                            database::delete_room_connections_close_room(msg.room_id, conn_pull)
                                .await
                                .unwrap();
                        });
                        self.rooms.remove(&msg.room_id);
                        self.rooms_state
                            .available_rooms_state
                            .lock()
                            .unwrap()
                            .retain(|r| r.room_id != msg.room_id);
                        self.handle(
                            EchoAvailableRoomsLobby {
                                lobby_id: self.get_lobby_uuid(),
                            },
                            ctx,
                        );
                    }
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
                hset.iter().for_each(|client| {
                    self.send_message(room_notification_serialized.as_str(), client)
                });
            }
            None => println!("Empty room"),
        }
    }
}

impl Handler<NotifyInitialLobbyState> for Lobby {
    type Result = ();

    fn handle(&mut self, _: NotifyInitialLobbyState, _: &mut Context<Self>) -> Self::Result {
        
    }
}

async fn initial_state(conn: &PgPool) -> NotifyInitialLobbyState {
    let vec_rooms = sqlx::query_as!(
        RoomsInitialState,
        r#"SELECT availablerooms.*, rooms.name, rooms.max_number_of_players 
        FROM availablerooms, rooms 
        WHERE availablerooms.room_id = rooms.id 
        AND availablerooms.is_open=true"#
    )
    .fetch_all(conn)
    .await
    .expect("Failed to query available rooms");
    let vec_connections = sqlx::query_as!(
        ConnectionsInitialState,
        r#"SELECT connections.*, users.name 
    FROM users, connections 
    WHERE users.id = connections.user_id"#
    )
    .fetch_all(conn)
    .await
    .expect("Failed to query available rooms");

    NotifyInitialLobbyState {
        msg_type: Initial,
        rooms: vec_rooms,
        users: vec_connections,
    }
}
