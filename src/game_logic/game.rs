use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver};

use actix::Addr;

use super::game_actor_messages::{GameAction, GameNotification, RoundData, UserData};
use super::{ Deck, HashMapWinnersKey, Player, TeamWinnerValue,
    TurnManager, GameActor,
};
use crate::model::MessageRoomType;
use crate::websockets::{GameSocketInput};
use crate::{ websockets::lobby_messages::WsMessage};





#[derive(Debug)]
pub struct Game {
    pub round: u64,
    pub deck: Deck,
    pub players: VecDeque<Player>,
    pub round_winners: HashMap<HashMapWinnersKey, i32>,
    pub game_actor_addr: Addr<GameActor>
}
impl Game {
    pub fn new(players: VecDeque<Player>,addr: Addr<GameActor>) -> Game {
        let mut deck = Deck::default();
        deck.deck_setup();
      
        Game {
            round: 1,
            deck,
            players,
            round_winners: HashMap::new(),
            game_actor_addr: addr
        }
    }
    fn deal_cards(&mut self) {
        for player in &mut self.players {
            if let Some(_) = player.hand {
                player.hand.as_mut().unwrap().clear();
            }
            player.hand = Some(self.deck.draw_cards());
        }
    }
    fn set_new_starter(&mut self) {
        self.players.rotate_left(1);
    }
    fn refresh_deck(&mut self) {
        self.deck.refresh_deck();
    }
    fn increse_round_counter(&mut self) {
        self.round += 1;
    }
    fn next_round(&mut self) {
        self.refresh_deck();
        self.deal_cards();
        self.set_new_starter();
        self.increse_round_counter();
    }
    fn insert_round_winner(&mut self, info: Option<TeamWinnerValue>) {
        match info {
            Some(i) => {
                *self
                    .round_winners
                    .entry(HashMapWinnersKey::TeamId(i.team_id))
                    .or_insert(0) += i.turn_value;
            }
            None => {
                *self
                    .round_winners
                    .entry(HashMapWinnersKey::Draw)
                    .or_insert(0) += 1
            }
        };
    }
    fn evaluate_game_winner(&mut self) -> Option<HashMapWinnersKey> {
        for (key, value) in &self.round_winners {
            if *value >= 12 {
                return Some(key.clone());
            }
        }
        None
    }
    fn round_start(&mut self){
        self.deal_cards();
    }
    pub fn play(&mut self, rc: Arc<Mutex<Receiver<GameSocketInput>>>) {
        //initial setup
        self.round_start();
        //loop while no one win
        while self.evaluate_game_winner().is_none() {
            self.notify_players_round_start();
            let mut turn_m = TurnManager::new(self.players.clone(), Arc::clone(&rc), Arc::new(self.game_actor_addr.clone()));
            let round_winner = turn_m.play();
            self.insert_round_winner(Some(round_winner));
            self.next_round();
        }
        //notify game winner
        self.notify_game_winner();
    }

    fn notify_players_round_start(&mut self) {
        self.players.iter().enumerate().for_each(|(i, p)| {
            let hand = p.hand.as_ref().unwrap();
            let notification = GameNotification {
                msg_type: MessageRoomType::GameNotification,
                action: GameAction::RoundStartState,
                user_data: UserData {
                    id: p.id,
                    hand: hand.to_vec(),
                    team_id: p.team_id,
                    position: i,
                    is_allowed_to_truco:false
                },
                round_data: Some(RoundData {
                    manilha: self.deck.fliped_card.unwrap(),
                    round: self.round,
                }),
            };
            self.game_actor_addr.do_send(notification);
        });
    }
    fn notify_game_winner(&mut self){
        let winner = self.evaluate_game_winner().unwrap();
        let serialized_notification = serde_json::to_string(&winner).unwrap();
        self.players.iter().for_each(|p|{
            p.ws_addr.do_send(WsMessage(serialized_notification.clone()));
        });
    }
    fn notify_round_winner(&mut self){
        
    }
}


