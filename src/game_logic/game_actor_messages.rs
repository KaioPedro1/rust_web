use actix::Message;
use serde::Serialize;
use uuid::Uuid;

use crate::model::MessageRoomType;

use super::{Card, PlayedCard, TeamWinnerValue};

#[derive(Message)]
#[rtype(result = "()")]
pub struct GameStart {
    pub teste: String,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct UserResponse {
    pub user_id: Uuid,
    pub msg: String,
}

#[derive(Debug, Serialize, Message)]
#[rtype(result = "()")]
pub struct GameNotification {
    pub msg_type: MessageRoomType,
    pub action: GameAction,
    pub user_data: UserData,
    pub round_data: Option<RoundData>,
}
#[derive(Debug, Serialize)]
pub enum GameAction {
    RoundStartState,
    RoundWinner,
    CurrentScore,
    PlayerTurn,
    PlayerPlayedCard,
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
#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct GameNotificationPlayedCard {
    pub msg_type: MessageRoomType,
    pub action: GameAction,
    pub user_id: Uuid,
    pub position: usize,
    pub card: Card,
}
impl GameNotificationPlayedCard {
    pub fn new(player_data: PlayedCard) -> Self {
        Self {
            msg_type: MessageRoomType::GameNotification,
            action: GameAction::PlayerPlayedCard,
            user_id: player_data.player.id,
            position: player_data.position_in_table,
            card: player_data.card,
        }
    }
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct GameNotificationTurnWinner {
    pub msg_type: MessageRoomType,
    pub action: GameAction,
    pub turn_winner: TeamWinnerValue,
    pub current_score: [i32; 3],
}
