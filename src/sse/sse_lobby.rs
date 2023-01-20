use std::{ sync::{Arc, Mutex}, time::Duration};
use actix::{Actor, Addr, Handler, Context, AsyncContext};
use actix_web::web::Data;
use crate::model::AvailableRooms;
use super::{SseClient, Connect, Disconnect, SendMsg};



pub struct SseLobby {
    clients: Vec<Addr<SseClient>>,
    available_rooms:Data<Arc<Mutex<Vec<AvailableRooms>>>>
}
impl SseLobby{
    pub fn new(rooms_state: Data<Arc<Mutex<Vec<AvailableRooms>>>>)-> SseLobby{
        SseLobby{
            clients: Vec::new(),
            available_rooms: rooms_state
        }
    }
    fn add_client(&mut self, addr: Addr<SseClient>) {
        self.clients.push(addr);
        println!("{:?}",self.clients.len());
    }
    fn remove_client(&mut self, addr: Addr<SseClient>){
        self.clients.retain(|x| x != &addr);
        println!("{:?}",self.clients.len());
    }
}


impl Actor for SseLobby {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.clients.clear();
        ctx.run_interval(Duration::from_secs(1), |actor, _| {
            let counter = (actor.clients.len() + 1) as u32;
            for client in actor.clients.iter() {
                client.do_send(SendMsg{number:55});
            }
        });
    }
}

impl Handler<Connect> for SseLobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {

       self.add_client(msg.client_addr);
    }
}
impl Handler<Disconnect> for SseLobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        self.remove_client(msg.client_addr);
    }
}
