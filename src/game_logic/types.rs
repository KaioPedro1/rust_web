use serde::Serialize;

use super::Player;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Serialize)]
pub enum HashMapWinnersKey {
    TeamId(i32),
    Draw,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PlayedCard {
    pub position_in_table: usize,
    pub player: Player,
    pub card: Card,
}
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct Card {
    pub suit: u8,
    pub number: u8,
    pub rank: u8,
    pub is_manilha: bool,
}
#[derive(Clone, PartialEq, Debug)]
pub enum WinnerType {
    CardWin(PlayedCard),
    Draw,
}
pub enum PlayerAnswerTruco {
    Yes,
    No,
}
pub enum UserAction {
    PlayCard(Card),
    AskForTruco,
}
pub struct TeamWinnerValue {
    pub team_id: i32,
    pub turn_value: i32,
}
#[derive(Clone, PartialEq, Debug)]
pub struct TurnWinner {
    pub turn: i32,
    pub team_id: Option<i32>,
    pub is_draw: bool,
    pub winner_position: Option<i32>,
}
