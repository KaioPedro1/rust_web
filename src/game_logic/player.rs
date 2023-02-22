use std::{
    io,
    sync::{Arc, Mutex},
};

use actix::Recipient;

use uuid::Uuid;

use crate::model::MessageRoomType::GameNotification as gn;

use super::{
    game_actor_messages::{GameNotification, UserData},
    Card, PlayerAnswerTruco, Truco, UserAction,
};
use crate::game_logic::game_actor_messages::GameAction::PlayerTurn;
use crate::websockets::lobby_messages::WsMessage;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub id: Uuid,
    pub hand: Option<Vec<Card>>,
    pub team_id: i32,
    pub ws_addr: Recipient<WsMessage>,
}
impl Player {
    pub fn new(id: Uuid, team: i32, addr: Recipient<WsMessage>) -> Player {
        Player {
            id,
            hand: None,
            team_id: team,
            ws_addr: addr,
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
            println!("3: Truco");
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
    pub fn verify_user_input(&self, input: i32, state: Arc<Mutex<Truco>>) -> Result<UserAction, String> {
        let limit = self.hand.as_ref().unwrap().len();
        let is_allowed = self.verify_player_allowed_to_truco(state);
        let hand = self.hand.as_ref().unwrap();
        
        if (input >= limit.try_into().unwrap() || input<0) && input != 3{
            return Err("Invalid input".to_string());
        }
        else {
            if input == 3 && !is_allowed {
                return Err("Invalid input".to_string());
            }
            else if input == 3 && is_allowed {
                return Ok(UserAction::AskForTruco);
            }
            else {
                return Ok(UserAction::PlayCard(hand[input as usize]));
            }
        }
    }
    pub fn ask_player_action(&self, truco_state: Arc<Mutex<Truco>>) {
        let is_allowed = self.verify_player_allowed_to_truco(truco_state);

        let udata = UserData {
            id: self.id,
            hand: self.hand.as_ref().unwrap().to_vec(),
            team_id: self.team_id,
            position: 0,
            is_allowed_to_truco: is_allowed,
        };
        let notification = GameNotification {
            msg_type: gn,
            action: PlayerTurn,
            user_data: udata,
            round_data: None,
        };
        //envia msg
        let serialized_notification = serde_json::to_string(&notification).unwrap();
        self.ws_addr.do_send(WsMessage(serialized_notification));
    }
  
    /*fn get_user_input(is_allowed: bool) -> i32 {
        let mut max_input = 2;
        if is_allowed {
            max_input = 3;
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let input = input.trim().parse().unwrap_or(-1);
        if input >= 0 && input <= max_input {
            input
        } else {
            println!(
                "Invalid input, please enter a number between 0 and {:?}.",
                max_input
            );
            Self::get_user_input(is_allowed)
        }
    }*/
    pub fn send_message(&self, msg: String) {
        self.ws_addr.do_send(WsMessage(msg));
    }
    pub fn remove_card(&mut self, card: Card) {
        self.hand.as_mut().unwrap().retain(|&x| x != card)
    }
    pub fn get_first_card_from_hand(&self) -> Card {
        self.hand.as_ref().unwrap().get(0).unwrap().to_owned()
    }
}
