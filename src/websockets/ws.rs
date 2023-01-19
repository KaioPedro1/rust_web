use crate::database;
use crate::model::{AvailableRooms, MaxNumberOfPlayers, Room, RoomName};
use crate::websockets::lobby_ws::Lobby;
use crate::websockets::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::{fut, ActorContext};
use actix::{
    Actor, ActorFutureExt, Addr, ContextFutureSpawner, Running, StreamHandler, WrapFuture,
};
use actix::{AsyncContext, Handler};
use actix_web::web::Data;
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use sqlx::PgPool;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::UserInput;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone)]
pub struct WsConn {
    room: Uuid,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    id: Uuid,
    available_rooms: Arc<Mutex<Vec<AvailableRooms>>>,
    connection_pull: Data<PgPool>,
}
//id da sala global lobby hardcodado
impl WsConn {
    pub fn new(
        user_id: Uuid,
        lobby: Addr<Lobby>,
        rooms_state: Arc<Mutex<Vec<AvailableRooms>>>,
        conn: Data<PgPool>,
    ) -> WsConn {
        WsConn {
            id: user_id,
            room: Uuid::parse_str("57a1396b-ac9d-4558-b356-1bf87246a14f").unwrap(),
            hb: Instant::now(),
            lobby_addr: lobby,
            available_rooms: rooms_state,
            connection_pull: conn,
        }
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.room,
                self_id: self.id,
                initial_room_state: self.available_rooms.clone(),
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            room_id: self.room,
        });
        Running::Stop
    }
}

impl WsConn {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                act.lobby_addr.do_send(Disconnect {
                    id: act.id,
                    room_id: act.room,
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"PING");
        });
    }
}
//TODO: tratar erro ou refatorar a deserialização da mensagem recebida
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => {
                let user_input: UserInput = serde_json::from_str(s.to_string().as_str()).unwrap();
                let (new_room, new_available_room) = validade_and_build_room(user_input).unwrap();
                let conn_pull = self.connection_pull.clone();
                let self_clone = self.clone();

                tokio::spawn(async move {
                    match database::insert_room_and_available_room_db(
                        &new_room,
                        &new_available_room,
                        &self_clone.id,
                        conn_pull.clone(),
                    )
                    .await
                    {
                        Ok(_) => {
                            self_clone.available_rooms.lock().unwrap().push(new_available_room);
                            self_clone.lobby_addr.do_send(ClientActorMessage {
                                id: self_clone.id,
                                msg: String::from(format!(
                                    "**Private message** Room created successfully!, redirect to ../{:?}",
                                    new_room.id
                                )),
                                room_id: self_clone.room,
                                rooms_state: self_clone.available_rooms.clone(),
                            })
                        }
                        Err(e) => {
                            println!("{:#?}", e);
                            self_clone.lobby_addr.do_send(ClientActorMessage {
                                id: self_clone.id,
                                msg: String::from("Failed to create a new room!"),
                                room_id: self_clone.room,
                                rooms_state: self_clone.available_rooms.clone(),
                            })
                        }
                    }
                });
            }
            Err(_) => panic!("e"),
        }
    }
}
impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

fn validade_and_build_room(input: UserInput) -> Result<(Room, AvailableRooms), String> {
    let room_id = Uuid::new_v4();
    let room_name = RoomName::parse(input.name)?;
    let max_number_players = MaxNumberOfPlayers::parse(input.number_of_players)?;
    let room = Room {
        id: room_id,
        name: room_name,
        max_number_players,
    };
    let new_available_room = AvailableRooms {
        id: Uuid::new_v4(),
        room_id,
        number_of_players: 0,
        is_open: true,
    };

    Ok((room, new_available_room))
}
