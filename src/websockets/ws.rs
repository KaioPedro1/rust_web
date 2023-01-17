use actix::{fut, ActorContext};
use crate::model::AvailableRooms;
use crate::websockets::messages::{Disconnect, Connect, WsMessage, ClientActorMessage}; 
use crate::websockets::lobby_ws::Lobby; 
use actix::{Actor, Addr, Running, StreamHandler, WrapFuture, ActorFutureExt, ContextFutureSpawner};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::UserInput;


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    room: Uuid,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    id: Uuid,
    available_rooms: Arc<Mutex<Vec<AvailableRooms>>>
}
//id da sala global lobby hardcodado
impl WsConn {
    pub fn new(user_id: Uuid, lobby: Addr<Lobby>, rooms_state: Arc<Mutex<Vec<AvailableRooms>>>) -> WsConn {
        WsConn {
            id: user_id,
            room:Uuid::parse_str("57a1396b-ac9d-4558-b356-1bf87246a14f").unwrap(),
            hb: Instant::now(),
            lobby_addr: lobby,
            available_rooms: rooms_state,
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
                initial_room_state: self.available_rooms.clone()
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
        self.lobby_addr.do_send(Disconnect { id: self.id, room_id: self.room });
        Running::Stop
    }
}

impl WsConn {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                act.lobby_addr.do_send(Disconnect { id: act.id, room_id: act.room });
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
                let teste: UserInput = serde_json::from_str(s.to_string().as_str()).unwrap();
                let testando = AvailableRooms{
                    id:Uuid::new_v4(),
                    room_id:Uuid::new_v4(),
                    number_of_players:teste.number_of_players,
                    is_open:true
                };
                let mut a = self.available_rooms.lock().unwrap();
                a.push(testando);

                self.lobby_addr.do_send(ClientActorMessage {
                id: self.id,
                msg: teste,
                room_id: self.room,
                rooms_state: self.available_rooms.clone()
            })},
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