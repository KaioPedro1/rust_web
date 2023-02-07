pub mod ws_g;
mod game_messages;
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use self::game_messages::{WsMessage, Connect, Disconnect, GameStart};

type Socket = Recipient<WsMessage>;

pub struct GameMessager {
    socket_addr: HashMap<Uuid, Socket>,     //player id > socket addr
    rooms: HashMap<Uuid, HashSet<Uuid>>, //room id  to list of users id
}

impl GameMessager {
    pub fn new() -> GameMessager {
        GameMessager {
            socket_addr: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.socket_addr.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}
impl GameMessager {}
impl Actor for GameMessager {
    type Context = Context<Self>;
}

impl Handler<Connect> for GameMessager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.rooms
            .entry(msg.room)
            .or_insert_with(HashSet::new)
            .insert(msg.ws_id);
        self.socket_addr.insert(msg.ws_id, msg.addr);
    }
}
impl Handler<Disconnect> for GameMessager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.rooms.remove(&msg.room_id);   
    }
}
impl Handler<GameStart> for GameMessager{
    type Result = ();
    fn handle(&mut self, msg: GameStart, ctx: &mut Self::Context) -> Self::Result {
        
    }
}