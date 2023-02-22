use std::{
    collections::VecDeque,
    rc::Rc,
    sync::{mpsc::Receiver, Arc, Mutex},
    time::Duration, thread::{self, JoinHandle},
};

use uuid::Uuid;

use crate::websockets::{GameSocketInput, WsMessage};

use super::{
    PlayedCard, Player, PlayerAnswerTruco, TeamWinnerValue, Truco, TurnWinner, UserAction,
    WinnerType,
};

pub struct TurnManager {
    turn: i32,
    players: VecDeque<Player>,
    turn_winners: Vec<TurnWinner>,
    truco_state: Arc<Mutex<Truco>>,
    msg_receiver: Arc<Receiver<GameSocketInput>>,
}
impl TurnManager {
    pub fn new(players: VecDeque<Player>, rx: Arc<Receiver<GameSocketInput>>) -> TurnManager {
        TurnManager {
            turn: 0,
            players,
            turn_winners: vec![],
            truco_state: Arc::new(Mutex::new(Truco {
                is_truco: false,
                truco_caller: None,
                truco_value: 1,
                is_fold: false,
            })),
            msg_receiver: rx,
        }
    }
    pub fn play(&mut self) -> TeamWinnerValue {
        for n in 0..1 {
            self.play_one_turn();
            let truco_state = self.truco_state.lock().unwrap();
            let turn_value = truco_state.truco_value;

            if truco_state.is_fold {
                return TeamWinnerValue {
                    team_id: self
                        .truco_state
                        .lock()
                        .unwrap()
                        .truco_caller
                        .as_ref()
                        .unwrap()
                        .team_id,
                    turn_value,
                };
            }
            if self.turn == 2 {
                if self.turn_winners[n - 1].is_draw {
                    if !self.turn_winners[n].is_draw {
                        return TeamWinnerValue {
                            team_id: self.turn_winners[n].team_id.unwrap(),
                            turn_value,
                        };
                    }
                } else {
                    if self.turn_winners[n].is_draw {
                        return TeamWinnerValue {
                            team_id: self.turn_winners[n - 1].team_id.unwrap(),
                            turn_value,
                        };
                    } else if self.turn_winners[n].team_id == self.turn_winners[n - 1].team_id {
                        return TeamWinnerValue {
                            team_id: self.turn_winners[n].team_id.unwrap(),
                            turn_value,
                        };
                    }
                }
            } else if self.turn == 3 {
                if self.turn_winners[n].is_draw {
                    return TeamWinnerValue {
                        team_id: self.turn_winners[n - 2].team_id.unwrap(),
                        turn_value,
                    };
                } else {
                    return TeamWinnerValue {
                        team_id: self.turn_winners[n].team_id.unwrap(),
                        turn_value,
                    };
                }
            }
        }
        //teste pq não tem round 2
        TeamWinnerValue {
            team_id: 666,
            turn_value: 666,
        }
    }
    pub fn play_one_turn(&mut self) {
        let mut new_turn = Turn::new(
            self.players.clone(),
            self.truco_state.clone(),
            Arc::clone(&self.msg_receiver),
        );
        new_turn.play();
        match new_turn.winner {
            WinnerType::CardWin(card) => {
                self.setup_next_turn(TurnWinner {
                    turn: self.turn,
                    team_id: Some(card.player.team_id),
                    is_draw: false,
                    winner_position: Some(card.position_in_table as i32),
                });
            }
            WinnerType::Draw => {
                self.setup_next_turn(TurnWinner {
                    turn: self.turn,
                    team_id: None,
                    is_draw: true,
                    winner_position: None,
                });
            }
        }
        self.remove_card_from_player_hand(&new_turn.played_cards);
    }
    fn setup_next_turn(&mut self, played_winner: TurnWinner) {
        self.increment_turn_counter();
        self.insert_turn_winner(played_winner.clone());
        self.set_play_order(played_winner);
    }
    fn increment_turn_counter(&mut self) {
        self.turn += 1;
    }
    fn insert_turn_winner(&mut self, played_winner: TurnWinner) {
        self.turn_winners.push(played_winner);
    }
    fn set_play_order(&mut self, played_winner: TurnWinner) {
        if !played_winner.is_draw || played_winner.winner_position.is_some() {
            self.players
                .rotate_left(played_winner.winner_position.unwrap().try_into().unwrap());
        }
    }
    fn remove_card_from_player_hand(&mut self, played_cards: &Vec<PlayedCard>) {
        for played_card in played_cards {
            if let Some(player) = self
                .players
                .iter_mut()
                .find(|p| p.id == played_card.player.id)
            {
                player.remove_card(played_card.card);
            }
        }
    }
}

pub struct Turn {
    pub played_cards: Vec<PlayedCard>,
    pub players: VecDeque<Player>,
    pub truco: Arc<Mutex<Truco>>,
    pub winner: WinnerType,
    pub player_turn: Option<Player>,
    pub msg_receiver: Arc<Receiver<GameSocketInput>>,
    pub player_positon: Option<usize>,
}

impl Turn {
    pub fn new(
        players: VecDeque<Player>,
        truco_state: Arc<Mutex<Truco>>,
        msg_receiver: Arc<Receiver<GameSocketInput>>,
    ) -> Turn {
        Turn {
            played_cards: vec![],
            players,
            truco: truco_state,
            winner: WinnerType::Draw,
            player_turn: None,
            msg_receiver,
            player_positon: None,
        }
    }
    pub fn play(&mut self) {
        let players_clone = Arc::new(self.players.clone());

        for (position, player) in players_clone.iter().enumerate() {
            self.player_turn = Some(player.to_owned());
            self.player_positon = Some(position);
            self.player_turn
                .as_ref()
                .unwrap()
                .ask_player_action(Arc::clone(&self.truco));
            dbg!(self.player_turn.as_ref().unwrap().id);
            let jh = self.handle_user_input();
            };
        
        self.evaluate_turn();
    }
    fn handle_user_input(&mut self)-> JoinHandle<Result<UserAction, String>>{
        let msg = self.msg_receiver.recv_timeout(Duration::from_secs(30));
        let j = thread::spawn(move || {
            if msg.is_ok(){
                if msg.as_ref().unwrap().user ==Uuid::new_v4() {
                    return Ok(self
                        .player_turn
                        .as_ref()
                        .unwrap()
                        .verify_user_input(msg.unwrap().player_input.unwrap())
                        .unwrap()
                    );
                }
                else {
                    self.player_turn.as_ref().unwrap().ws_addr.do_send(WsMessage("Error at playing card! its not your turn".to_owned()));
                }
            };
            return Err("Error, you take to long to play".to_string());
        });
        return j
    }
    
    fn evaluate_turn(&mut self) {
        let truco_state = self.truco.lock().unwrap();
        if truco_state.is_fold {
            return;
        }
        let manilhas: Vec<PlayedCard> = self
            .played_cards
            .clone()
            .into_iter()
            .filter(|playedcard| playedcard.card.is_manilha)
            .collect();

        if manilhas.is_empty() {
            self.played_cards
                .sort_unstable_by(|a, b| a.card.rank.cmp(&b.card.rank));

            let highest_playcard = self.played_cards.get(0).unwrap().clone();
            let second_highest_playcard = self.played_cards.get(1).unwrap();

            if highest_playcard.card.rank == second_highest_playcard.card.rank {
                self.winner = WinnerType::Draw;
            } else {
                self.winner = WinnerType::CardWin(highest_playcard)
            }
        } else {
            let highest_manilha = manilhas.into_iter().min_by_key(|c| c.card.rank).unwrap();

            self.winner = WinnerType::CardWin(highest_manilha)
        }
    }
    fn handle_truco_call(
        &mut self,
        player_caller: &Player,
        players_rc: Rc<VecDeque<Player>>,
    ) -> PlayerAnswerTruco {
        let mut player_answers: Vec<(Player, PlayerAnswerTruco)> = Vec::new();

        //espera input user
        for p in players_rc.iter() {
            if p.team_id != player_caller.team_id {
                let player = p;
                let awnser = player.answer_truco_action(&player_caller.id.to_string());
                player_answers.push((player.clone(), awnser));
            }
        }
        //verifica vector
        let mut yes_counter = 0;
        let mut no_counter = 0;

        for (_, an) in player_answers {
            match an {
                PlayerAnswerTruco::Yes => yes_counter += 1,
                PlayerAnswerTruco::No => no_counter += 1,
            }
        }
        if yes_counter == no_counter {
            println!("empate");
            self.truco
                .lock()
                .unwrap()
                .update_truco_state(PlayerAnswerTruco::No, player_caller.clone());
            PlayerAnswerTruco::No
        } else if yes_counter > no_counter {
            println!("Continua o jogo normal");
            self.truco
                .lock()
                .unwrap()
                .update_truco_state(PlayerAnswerTruco::Yes, player_caller.clone());
            PlayerAnswerTruco::Yes
        } else {
            println!("Pare o round e sete o vencedor do turno");
            self.truco
                .lock()
                .unwrap()
                .update_truco_state(PlayerAnswerTruco::No, player_caller.clone());
            PlayerAnswerTruco::No
        }
    }
}
