use std::{
    cmp::Ordering,
    collections::VecDeque,
    rc::Rc,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use actix::Addr;

use crate::websockets::GameSocketInput;

use super::{
    game_actor_messages::{GameNotificationPlayedCard, UserResponse},
    GameActor, PlayedCard, Player, PlayerAnswerTruco, TeamWinnerValue, Truco, TurnWinner,
    UserAction, WinnerType,
};
const WAITING_TIME: u64 = 15;
pub struct TurnManager {
    turn: i32,
    players: VecDeque<Player>,
    turn_winners: Vec<TurnWinner>,
    truco_state: Arc<Mutex<Truco>>,
    msg_receiver: Arc<Mutex<Receiver<GameSocketInput>>>,
    game_actor_addr: Arc<Addr<GameActor>>,
}
impl TurnManager {
    pub fn new(
        players: VecDeque<Player>,
        rx: Arc<Mutex<Receiver<GameSocketInput>>>,
        addr: Arc<Addr<GameActor>>,
    ) -> TurnManager {
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
            game_actor_addr: addr,
        }
    }
    pub fn play(&mut self) -> Result<TeamWinnerValue, &str> {
        for n in 0..3 {
            self.play_one_turn();
            let truco_state = self.truco_state.lock().unwrap();
            let turn_value = truco_state.truco_value;

            if truco_state.is_fold {
                return Ok(TeamWinnerValue {
                    team_id: truco_state.get_truco_caller_team_id(),
                    turn_value,
                });
            }
            if self.turn == 2 {
                if self.turn_winners[n - 1].is_draw {
                    if !self.turn_winners[n].is_draw {
                        return Ok(TeamWinnerValue {
                            team_id: self.turn_winners[n].team_id.unwrap(),
                            turn_value,
                        });
                    }
                } else {
                    if self.turn_winners[n].is_draw {
                        return Ok(TeamWinnerValue {
                            team_id: self.turn_winners[n - 1].team_id.unwrap(),
                            turn_value,
                        });
                    } else if self.turn_winners[n].team_id == self.turn_winners[n - 1].team_id {
                        return Ok(TeamWinnerValue {
                            team_id: self.turn_winners[n].team_id.unwrap(),
                            turn_value,
                        });
                    }
                }
            } else if self.turn == 3 {
                if self.turn_winners[n].is_draw {
                    return Ok(TeamWinnerValue {
                        team_id: self.turn_winners[n - 2].team_id.unwrap(),
                        turn_value,
                    });
                } else {
                    return Ok(TeamWinnerValue {
                        team_id: self.turn_winners[n].team_id.unwrap(),
                        turn_value,
                    });
                }
            }
        }
        //error
        return Err("No winner found");
    }
    pub fn play_one_turn(&mut self) {
        let mut new_turn = Turn::new(
            self.players.clone(),
            Arc::clone(&self.truco_state),
            Arc::clone(&self.msg_receiver),
            Arc::clone(&self.game_actor_addr),
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
    pub msg_receiver: Arc<Mutex<Receiver<GameSocketInput>>>,
    pub addr_actor: Arc<Addr<GameActor>>,
}

impl Turn {
    pub fn new(
        players: VecDeque<Player>,
        truco_state: Arc<Mutex<Truco>>,
        msg_receiver: Arc<Mutex<Receiver<GameSocketInput>>>,
        addr: Arc<Addr<GameActor>>,
    ) -> Turn {
        Turn {
            played_cards: vec![],
            players,
            truco: truco_state,
            winner: WinnerType::Draw,
            msg_receiver,
            addr_actor: addr,
        }
    }
    pub fn play(&mut self) {
        let players_clone = Arc::new(self.players.clone());

        for (position, player) in players_clone.iter().enumerate() {
            player.ask_player_action(Arc::clone(&self.truco));

            let jh = self.handle_user_input(player);
            let result = jh.join().unwrap();
            match result {
                Ok(action) => match action {
                    UserAction::PlayCard(card) => {
                        let played_card = PlayedCard {
                            player: player.clone(),
                            card,
                            position_in_table: position,
                        };
                        self.insert_played_card(played_card.clone());
                        self.notify_player_answer(player, played_card, "Sucess".to_string());
                    }
                    UserAction::AskForTruco => todo!(),
                },
                Err(e) => {
                    //case player disconnect or dont play anything
                    let played_card = PlayedCard {
                        player: player.clone(),
                        card: player.get_first_card_from_hand(),
                        position_in_table: position,
                    };
                    self.insert_played_card(played_card.clone());
                    self.notify_player_answer(player, played_card, e);
                }
            }
        }

        self.evaluate_turn();
    }
    fn handle_user_input(&self, player: &Player) -> JoinHandle<Result<UserAction, String>> {
        let msg_rec_clone = Arc::clone(&self.msg_receiver);
        let playerturnclone = player.clone();
        let addrclone = Arc::clone(&self.addr_actor);
        let trucoclone = Arc::clone(&self.truco);

        let handle = thread::spawn(move || {
            let msg_rec_clone = msg_rec_clone.lock().unwrap();
            let duration = Duration::from_secs(WAITING_TIME);
            loop {
                let msg = msg_rec_clone.recv_timeout(duration);
                if let Err(e) = msg {
                    return Err(e.to_string());
                }
                let msg = msg.unwrap();
                if let None = msg.player_input {
                    panic!("Error, user input is none");
                }

                let user_input = msg.player_input.unwrap();

                if msg.user != playerturnclone.id {
                    addrclone.do_send(UserResponse {
                        user_id: msg.user,
                        msg: "Its not your turn".to_owned(),
                    });
                    continue;
                }
                match playerturnclone.verify_user_input(user_input, trucoclone.to_owned()) {
                    Ok(act) => return Ok(act),
                    Err(e) => addrclone.do_send(UserResponse {
                        user_id: msg.user,
                        msg: e,
                    }),
                }
            }
        });
        return handle;
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

            let highest_playcard = self.played_cards.get(0);
            let second_highest_playcard = self.played_cards.get(1);
            match (highest_playcard, second_highest_playcard) {
                (Some(high), Some(low)) => {
                    self.winner = match high.card.rank.cmp(&low.card.rank) {
                        Ordering::Equal => WinnerType::Draw,
                        Ordering::Greater => WinnerType::CardWin(low.clone()),
                        Ordering::Less => WinnerType::CardWin(high.clone()),
                    };
                }
                (_, _) => {
                    panic!("Error, its not possible to find a highest card or second highest card");
                }
            };
        } else {
            let highest_manilha = manilhas.into_iter().min_by_key(|c| c.card.suit).unwrap();

            self.winner = WinnerType::CardWin(highest_manilha);
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
    fn insert_played_card(&mut self, playedcard: PlayedCard) {
        self.played_cards.push(playedcard);
    }
    fn notify_player_answer(&mut self, player: &Player, playedcard: PlayedCard, msg: String) {
        player.send_message(msg);
        match self
            .addr_actor
            .try_send(GameNotificationPlayedCard::new(playedcard))
        {
            Ok(_) => (),
            Err(e) => println!("Error sending message to actor: {}", e),
        };
    }
}
