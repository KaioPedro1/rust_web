use actix::{fut, ActorContext, Addr, WrapFuture, ActorFutureExt, ContextFutureSpawner};
use actix::{
    Actor, Running, StreamHandler,
};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;

use std::time::{Duration, Instant};
use uuid::Uuid;

use super::GameMessager;
use super::game_messages::{Connect, Disconnect, WsMessage};



const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Clone)]
pub struct WsGameConn {
    id: Uuid,
    room: Uuid,
    hb: Instant,
    game_addr: Addr<GameMessager>
}
impl WsGameConn {
    pub fn new(user_id: Uuid, room_id: Uuid, game: Addr<GameMessager>) -> WsGameConn {
        WsGameConn {
            id: user_id,
            room: room_id,
            hb: Instant::now(),
            game_addr:game
        }
    }
}

impl Actor for WsGameConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        
        self.game_addr
            .send(Connect {
                addr: addr.recipient(),
                room: self.room,
                ws_id: self.id,
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
        self.game_addr.do_send(Disconnect {
            id: self.id,
            room_id: self.room,
        });
        Running::Stop
    }
}

impl WsGameConn {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                act.game_addr.do_send(Disconnect {
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
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsGameConn {
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
            Ok(Text(_)) => {}
            Err(_) => panic!("e"),
        }
    }
}
impl Handler<WsMessage> for WsGameConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
