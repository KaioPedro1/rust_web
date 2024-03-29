use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

pub struct ConnectionTuple {
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub is_admin: bool,
    pub position: i32,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub name: UserName,
    pub id: Uuid,
    pub avatar_id: AvatarId,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AvatarId(pub i32);
impl AvatarId {
    pub fn parse(n: i32) -> Result<AvatarId, String> {
        let is_above_ceil = n > 12;
        let is_beneath_floor = n < 1;

        if is_above_ceil || is_beneath_floor {
            Err(format!("{} is not in the interval [1,12].", n))
        } else {
            Ok(Self(n))
        }
    }
}
impl AsRef<i32> for AvatarId {
    fn as_ref(&self) -> &i32 {
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Room {
    pub id: Uuid,
    pub name: RoomName,
    pub room_capacity: RoomCapacity,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoomName(pub String);
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoomCapacity(pub i32);
impl RoomCapacity {
    pub fn parse(n: i32) -> Result<RoomCapacity, String> {
        let is_above_ceil = n > 4;
        let is_beneath_floor = n < 2;
        let is_not_even = n % 2 != 0;

        if is_above_ceil || is_beneath_floor || is_not_even {
            Err(format!("{} is not in the interval (2,24).", n))
        } else {
            Ok(Self(n))
        }
    }
}
impl AsRef<i32> for RoomCapacity {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ActionLobbyType {
    Add,
    Delete,
    Enter,
    Leave,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MessageLobbyType {
    UpdateRoom,
    UpdatePlayer,
    Initial,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RoomTypes {
    Room(Room),
    Uuid(Uuid),
    Rooms(Vec<Room>),
}
impl RoomTypes {
    pub fn get_uuid(&self) -> Uuid {
        match self {
            RoomTypes::Room(room) => room.id,
            RoomTypes::Uuid(uuid) => *uuid,
            RoomTypes::Rooms(_) => Uuid::nil(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum UserTypes {
    Uuid(Uuid),
    Connections(Vec<ConnectionMessage>),
    Connection(ConnectionMessage),
}
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageRoomType {
    RoomNotification,
    Redirect,
    GameNotification,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ActionRoomType {
    Update,
    Delete,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ConnectionMessage {
    pub user_id: Uuid,
    pub room_id: Uuid,
    pub is_admin: bool,
    pub name: String,
    pub avatar_id: i32,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub avatar_id: i32,
    pub exp: usize,
}
