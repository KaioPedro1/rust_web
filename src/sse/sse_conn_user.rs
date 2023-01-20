use std::time::{Instant,Duration};

use actix::{Actor, StreamHandler, System, Addr, AsyncContext, WrapFuture, ActorFutureExt, fut, ActorContext, ContextFutureSpawner, Running, Handler};
use actix_web::{web, Error, HttpResponse};
use actix_web_lab::sse;
use uuid::Uuid;

use crate::sse::Disconnect;

use super::{SseLobby, Connect, SendMsg};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct SseClient {
    client_id: Uuid,
    lobby_addr: Addr<SseLobby>,
    hb: Instant
}
impl SseClient{
    pub fn new(user_id:Uuid, lobby_addr:Addr<SseLobby>)->SseClient{
        SseClient { 
            client_id: user_id, 
            lobby_addr,
            hb: Instant::now()
        }
    }
    pub fn hb(&self, ctx: &mut actix::Context<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                let addr = ctx.address();
                act.lobby_addr.do_send(Disconnect {
                    client_addr:addr
                });
                ctx.stop();
                return;
            }
            act.hb=Instant::now();
        });
    }
}
impl Actor for SseClient {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr.send(Connect{
            client_id: self.client_id,
            client_addr: addr
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

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running { 
        let addr = ctx.address();
        self.lobby_addr.do_send(Disconnect {
            client_addr: addr,
        });
        Running::Stop
    }
}

impl StreamHandler<Result<web::Bytes, Error>> for SseClient {
    fn handle(&mut self, item: Result<web::Bytes, Error>, _: &mut Self::Context) {
        if let Ok(bytes) = item {
            let text = std::str::from_utf8(&bytes).unwrap();
            if text.trim() == "stop" {
                System::current().stop();
            }
        }
    }
}


impl Handler<SendMsg> for SseClient {
    type Result = ();
    fn handle(&mut self, msg: SendMsg, ctx: &mut Self::Context) {
        let data = format!("data: {}\n\n", msg.number);
        let (tx, rx) = sse::channel(10);
    }
}

