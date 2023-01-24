use serde::Serialize;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

pub struct ConnectionTuple {
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub is_admin: bool,
}
#[derive(Debug, Serialize)]
pub struct User {
    pub name: UserName,
    pub id: Uuid,
}
#[derive(Debug, Serialize)]
pub struct UserName(pub String);
impl UserName {
    pub fn parse(s: String) -> Result<UserName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 50;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
//TODO: trocar numeros de players para u8 e implementar validação de todos os dados necessarios abaixo para inclusão no banco de dados
#[derive(Debug, Serialize)]
pub struct AvailableRooms {
    pub id: Uuid,
    pub room_id: Uuid,
    pub number_of_players: i32,
    pub is_open: bool,
}

#[derive(Debug, Serialize)]
pub struct Room {
    pub id: Uuid,
    pub name: RoomName,
    pub max_number_players: MaxNumberOfPlayers,
}

#[derive(Debug, Serialize)]
pub struct RoomName(String);
impl RoomName {
    pub fn parse(s: String) -> Result<RoomName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 100;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid room name.", s))
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for RoomName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Serialize)]
pub struct MaxNumberOfPlayers(i32);
impl MaxNumberOfPlayers {
    pub fn parse(n: i32) -> Result<MaxNumberOfPlayers, String> {
        let is_above_ceil = n > 24;
        let is_beneath_floor = n < 2;

        if is_above_ceil || is_beneath_floor {
            Err(format!("{} is not in the interval (2,24).", n))
        } else {
            Ok(Self(n))
        }
    }
}
impl AsRef<i32> for MaxNumberOfPlayers {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}
#[derive(Debug, Serialize)]
pub struct  LobbyNotification {
    msg_type: MessageLobbyType,
    action: ActionLobbyType,
    room: Room,
    users:Option<Vec<User>>
}

#[derive(Debug, Serialize)]
pub enum ActionLobbyType {
    Add,
    Delete,
}
#[derive(Debug, Serialize)]
pub enum MessageLobbyType {
    Update,
    Initial
}


#[derive(Debug, Serialize)]
pub enum MessageRoomType {
    Notification,
    Redirect,
}
#[derive(Debug, Serialize)]
pub enum ActionRoomType {
    Enter,
    Leave,
    Delete,
}

#[derive(Debug, Serialize)]
pub struct RoomsInitialState{
    pub id: Uuid,
    pub room_id:Uuid,
    pub number_of_players: i32,
    pub is_open: bool,
    pub name:String,
    pub max_number_of_players:i32
}
#[derive(Debug, Serialize)]
pub struct ConnectionsInitialState{
    pub user_id: Uuid,
    pub room_id:Uuid,
    pub is_admin: bool,
    pub name: String
}

