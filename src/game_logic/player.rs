use std::{
    cell::{Ref, RefCell},
    io,
    rc::Rc,
};


use actix::{ Recipient};
use serde::Serialize;
use uuid::Uuid;

use crate::websockets::{lobby_messages::WsMessage};

use super::{Card, PlayerAnswerTruco, UserAction, Truco};



#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub id: Uuid,
    pub hand: Option<Vec<Card>>,
    pub team_id: i32,
    pub ws_addr: Recipient<WsMessage>
}
impl Player {
    pub fn new(id: Uuid, team: i32, addr: Recipient<WsMessage>) -> Player {
        Player {
            id,
            hand: None,
            team_id: team,
            ws_addr:addr
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
    fn verify_player_allowed_to_truco(&self, state: Ref<Truco>) -> bool {
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
    pub fn ask_player_action(&self, truco_state: Rc<RefCell<Truco>>) -> UserAction {
        let state = truco_state.borrow();
        println!(
            "{:?} team:{:?},  your options are:",
            self.id, self.team_id
        );
        for (index, card) in self.hand.as_ref().unwrap().iter().enumerate() {
            println!("{:?}:{:?}", index, card);
        }
        let is_allowed = self.verify_player_allowed_to_truco(state);

        let uinput = Player::get_user_input(is_allowed);

        match uinput {
            3 => UserAction::AskForTruco,
            0 => {
                let hand = self.hand.as_ref().unwrap();
                UserAction::PlayCard(hand[uinput as usize])
            }
            1 => {
                let hand = self.hand.as_ref().unwrap();
                UserAction::PlayCard(hand[uinput as usize])
            }
            2 => {
                let hand = self.hand.as_ref().unwrap();
                UserAction::PlayCard(hand[uinput as usize])
            }
            _ => self.ask_player_action(truco_state.clone()),
        }
    }
    fn get_user_input(is_allowed: bool) -> i32 {
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
    }
    pub fn remove_card(&mut self, card: Card) {
        self.hand.as_mut().unwrap().retain(|&x| x != card)
    }
}
