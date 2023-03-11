use std::{
    io,
    sync::{Arc, Mutex},
};

use actix::Recipient;

use uuid::Uuid;


use crate::websockets::WsMessage;

use super::{
    game_actor_messages::{ UserData},
    Card, PlayerAnswerTruco, Truco, UserAction, PlayerPublicData,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub id: Uuid,
    pub hand: Option<Vec<Card>>,
    pub team_id: i32,
    pub ws_addr: Recipient<WsMessage>,
    pub position: Option<i32>,
}
impl Player {
    pub fn new(id: Uuid, team: i32, addr: Recipient<WsMessage>) -> Player {
        Player {
            id,
            hand: None,
            team_id: team,
            ws_addr: addr,
            position: None,
        }
    }
    pub fn answer_truco_action(&self, asker: &String) -> PlayerAnswerTruco {
        println!(
            "Hey {:?}, {:?} is asking for truco do you accept it?",
            self.id, asker
        );
        println!("0: Yes");
        println!("1: No");
        Player::get_user_truco_answer()
    }
    pub fn get_user_truco_answer() -> PlayerAnswerTruco {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let input = input.trim().parse().unwrap_or(-1);

        match input {
            0 => PlayerAnswerTruco::Yes,
            1 => PlayerAnswerTruco::No,
            _ => {
                println!("Invalid input, please enter a number between 0 and 1.");
                Self::get_user_truco_answer()
            }
        }
    }
    fn verify_player_allowed_to_truco(&self, state: Arc<Mutex<Truco>>) -> bool {
        let state = state.lock().unwrap();

        if !state.is_truco {
            return true;
        } else if state.is_truco
            && state.truco_caller.as_ref().unwrap().team_id != self.team_id
            && state.truco_value < 12
        {
            println!("3:Raise to {:?}", state.truco_value + 3);
            return true;
        }
        false
    }
    pub fn verify_user_input(
        &self,
        input: i32,
        state: Arc<Mutex<Truco>>,
    ) -> Result<UserAction, String> { 
        let hand = self.hand.as_ref().unwrap();
        let limit = hand.len();
        let is_allowed = self.verify_player_allowed_to_truco(state);
       

        if ((input >= limit.try_into().unwrap() || input < 0) && input != 3)|| (input == 3 && !is_allowed) {
            Err("Invalid input".to_string())
        } else if input == 3 && is_allowed {
            return Ok(UserAction::AskForTruco);
        } else {
            return Ok(UserAction::PlayCard(hand[input as usize]));
        }
    }
    pub fn get_player_data(&self, truco_state: Arc<Mutex<Truco>>)->UserData {
        let is_allowed = self.verify_player_allowed_to_truco(truco_state);

        UserData {
            id: self.id,
            hand: Some(self.hand.as_ref().unwrap().to_vec()),
            team_id: self.team_id,
            position: self.position.unwrap() as usize,
            is_allowed_to_truco: is_allowed,
        }
    }
    pub fn send_message(&self, msg: String) {
        self.ws_addr.do_send(WsMessage(msg));
    }
    pub fn remove_card(&mut self, card: Card) {
        self.hand.as_mut().unwrap().retain(|&x| x != card)
    }
    pub fn get_first_card_from_hand(&self) -> Card {
        self.hand.as_ref().unwrap().get(0).unwrap().to_owned()
    }
    pub fn set_player_position(&mut self, position: i32) {
        self.position = Some(position);
    }
    pub fn get_player_public_data(&self) -> PlayerPublicData {
        PlayerPublicData {
            id: self.id,
            team_id: self.team_id,
            position: self.position.unwrap() as usize,
        }
    }
}
