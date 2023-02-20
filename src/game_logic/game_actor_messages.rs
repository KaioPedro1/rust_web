use actix::Message;
use serde::Serialize;
use uuid::Uuid;

use crate::model::MessageRoomType;

use super::Card;

#[derive(Message)]
#[rtype(result = "()")]
pub struct GameStart {
    pub teste: String,
}

#[derive(Debug, Serialize)]
pub struct GameNotification {
    pub msg_type: MessageRoomType,
    pub action: GameAction,
    pub user_data: UserData,
    pub round_data: Option<RoundData>,
}
#[derive(Debug, Serialize)]
pub enum GameAction {
    RoundStartState,
    CurrentScore,
    PlayerTurn,
}
#[derive(Debug, Serialize)]
pub struct UserData {
    pub id: Uuid,
    pub hand: Vec<Card>,
    pub team_id: i32,
    pub position: usize,
    pub is_allowed_to_truco: bool,
}
#[derive(Debug, Serialize)]
pub struct RoundData {
    pub manilha: Card,
    pub round: u64,
}
