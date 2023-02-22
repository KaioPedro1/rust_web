use std::collections::{ VecDeque};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use super::Player;
use super::{
    game_actor_messages::GameStart
};

use crate::game_logic::Game;

use crate::websockets::GameSocketInput;
use actix::{Actor, Context, Handler};

#[derive(Debug)]
pub struct GameActor {
    pub players: VecDeque<Player>,
    pub msg_sender_ws: Option<Sender<GameSocketInput>>,
}
impl GameActor {
    pub fn new(players: VecDeque<Player>) -> GameActor {
        GameActor {
            players,
            msg_sender_ws: None,
        }
    }
}
impl Actor for GameActor {
    type Context = Context<Self>;
}

impl Handler<GameStart> for GameActor {
    type Result = ();
    fn handle(&mut self, _: GameStart, _: &mut Self::Context) -> Self::Result {
        let (tx, rx): (Sender<GameSocketInput>, Receiver<GameSocketInput>) = mpsc::channel();
        self.msg_sender_ws = Some(tx);
        let players = self.players.clone();
        thread::spawn(move || {
            Game::new(players).play(Arc::new(Mutex::new(rx)));
        });
    }
}

impl Handler<GameSocketInput> for GameActor {
    type Result = ();
    fn handle(&mut self, msg: GameSocketInput, _: &mut Self::Context) -> Self::Result {
        match self.msg_sender_ws.as_ref().unwrap().send(msg) {
            Ok(_) => println!("msg sent"),
            Err(e) => println!("error sending msg{:#?}", e),
        };
    }
}

