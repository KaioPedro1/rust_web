use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use super::game_actor_messages::{
    GameAction, GameNotification, GameNotificationPlayedCard, UserResponse,
};
use super::game_actor_messages::{GameNotificationTurnWinner, GameStart};
use super::Player;

use crate::game_logic::Game;

use crate::websockets::{GameSocketInput, WsMessage};
use actix::{Actor, AsyncContext, Context, Handler};

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
    fn handle(&mut self, _: GameStart, ctx: &mut Self::Context) -> Self::Result {
        let (tx, rx): (Sender<GameSocketInput>, Receiver<GameSocketInput>) = mpsc::channel();
        self.msg_sender_ws = Some(tx);
        self.players.iter_mut().enumerate().for_each(|(i, x)| {
            x.set_player_position(i as i32);
        });
        let players: VecDeque<Player> = self.players.clone();
        let addr = ctx.address();
        thread::spawn(move || {
            Game::new(players, addr).play(Arc::new(Mutex::new(rx)));
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

impl Handler<UserResponse> for GameActor {
    type Result = ();
    fn handle(&mut self, msg: UserResponse, _: &mut Self::Context) -> Self::Result {
        let player = self.players.iter().find(|p| p.id == msg.user_id).unwrap();
        player.ws_addr.do_send(WsMessage(msg.msg));
    }
}

impl Handler<GameNotificationPlayedCard> for GameActor {
    type Result = ();

    fn handle(&mut self, msg: GameNotificationPlayedCard, _: &mut Self::Context) -> Self::Result {
        let serialized_message = serde_json::to_string(&msg).unwrap();
        for player in &self.players {
            player
                .ws_addr
                .do_send(WsMessage(serialized_message.clone()));
        }
    }
}
impl Handler<GameNotification> for GameActor {
    type Result = ();

    fn handle(&mut self, mut msg: GameNotification, _: &mut Self::Context) -> Self::Result {
        let serialized_private_message = serde_json::to_string(&msg).unwrap();
        //private message
        self.players
            .iter()
            .find(|x| x.id == msg.user_data.id)
            .unwrap()
            .ws_addr
            .do_send(WsMessage(serialized_private_message));

        if let GameAction::PlayerTurn = msg.action {
            msg.user_data.hand = None;
            let serialized_public_message = serde_json::to_string(&msg).unwrap();
            //public message
            for player in &self.players {
                if player.id != msg.user_data.id {
                    player
                        .ws_addr
                        .do_send(WsMessage(serialized_public_message.clone()));
                }
            }
        }
    }
}

impl Handler<GameNotificationTurnWinner> for GameActor {
    type Result = ();

    fn handle(&mut self, msg: GameNotificationTurnWinner, _: &mut Self::Context) -> Self::Result {
        let serialized_message = serde_json::to_string(&msg).unwrap();
        for player in &self.players {
            player
                .ws_addr
                .do_send(WsMessage(serialized_message.clone()));
        }
    }
}
