use std::collections::{HashMap, VecDeque};

use actix::{Actor, Context, Handler};

use crate::websockets::lobby_messages::WsMessage;

use super::{Deck, Player, HashMapWinnersKey, TeamWinnerValue, TurnManager, game_actor_messages::NewGame};


#[derive(Debug)]
pub struct GameManager {
    pub round: u64,
    pub deck: Deck,
    pub players: VecDeque<Player>,
    pub round_winners: HashMap<HashMapWinnersKey, i32>,
}
impl GameManager {
    pub fn new(players: VecDeque<Player>) -> GameManager {
        let mut deck = Deck::default();
        deck.deck_setup();
        GameManager {
            round: 1,
            deck,
            players,
            round_winners: HashMap::new(),
        }
    }
    fn deal_cards(&mut self) {
        for player in &mut self.players {
            if let Some(_) = player.hand {
                player.hand.as_mut().unwrap().clear();
            }
            player.hand = Some(self.deck.draw_cards());
            let serialized = serde_json::to_string(&player.hand).unwrap();
            player.ws_addr.do_send(WsMessage(serialized));
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
        self.increse_round_counter()
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
            println!("time{:#?} placar{:#?}", key, value);
            if *value >= 12 {
                return Some(key.clone());
            }
        }
        None
    }
    pub fn play(&mut self) {
        //initial setup
        self.next_round();
        //loop while no one win
        while self.evaluate_game_winner().is_none() {
            let mut turn_m = TurnManager::new(self.players.clone());
            let round_winner = turn_m.play();
            self.insert_round_winner(Some(round_winner));
            self.next_round();
        }
    }
}

impl Actor for GameManager{
    type Context = Context<Self>;
}

impl Handler<NewGame> for GameManager {
    type Result = ();
    fn handle(&mut self, msg: NewGame, _: &mut Self::Context) -> Self::Result {
        self.deal_cards();
    }
}
