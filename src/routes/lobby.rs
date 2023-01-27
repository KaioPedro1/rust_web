use std::sync::Mutex;
use actix_web::{http::header::LOCATION, web::{self, Data},HttpRequest, HttpResponse,};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{utils::{check_if_cookie_is_valid, open_file_return_http_response_with_cache, FilesOptions}, model::{Room, AvailableRooms, RoomName, MaxNumberOfPlayers, self, ConnectionMessage}, database, redis_utils::RedisState, websockets::LobbyNotification};



pub async fn lobby_get(req: HttpRequest, connection: web::Data<PgPool>) -> HttpResponse {
    match check_if_cookie_is_valid(&req, connection).await {
        Ok(_) =>open_file_return_http_response_with_cache(&req, FilesOptions::Lobby).await,
        Err(e) => e,
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct UserInput{
    pub name: String,
    pub number_of_players:i32,
}

pub async fn lobby_post(req: HttpRequest, connection: web::Data<PgPool>, user_input: web::Form<UserInput>, redis:Data<Mutex<RedisState>>)->HttpResponse{
    let user_uuid = match check_if_cookie_is_valid(&req, connection.clone()).await {
        Ok(u) => u,
        Err(e) => return e,
    };
    let (new_room, new_available_room) = validade_and_build_room(user_input.0).unwrap();

    match database::insert_room_and_available_room_db(&new_room, &new_available_room, &user_uuid, connection).await{    
        Ok(_) => {
            let room = new_room.clone();
            let user = ConnectionMessage{
                user_id: user_uuid,
                room_id: room.id,
                is_admin: true,
                name: "Vou vir dos cookies".to_string(),
                };
            let serialized_notification = serde_json::to_string(&LobbyNotification{
                msg_type:crate::model::MessageLobbyType::Update,
                action:Some(crate::model::ActionLobbyType::Add),
                room:crate::model::RoomTypes::Room(new_room),
                user:Some(model::UserTypes::Connection(user.clone())),
                sender_uuid: user_uuid,
            }).unwrap();
            let serialized_room = serde_json::to_string(&room).unwrap();
          
            let mut redis_unlock = redis.lock().unwrap();
            redis_unlock
                .insert_room_publish_to_lobby(room.id.to_string(), serialized_room,serialized_notification)
                .unwrap();
            redis_unlock
                .insert_connection(user)
                .unwrap();
            let url = format!("lobby/{}", room.id.to_string());
            HttpResponse::Found().append_header((LOCATION, url)).finish()
        },
        Err(_) => HttpResponse::Ok().finish(),
    }
}

fn validade_and_build_room(input: UserInput) -> Result<(Room, AvailableRooms), String> {
    let room_id = Uuid::new_v4();
    let room_name = RoomName::parse(input.name)?;
    let max_number_players = MaxNumberOfPlayers::parse(input.number_of_players)?;
    let room = Room {
        id: room_id,
        name: room_name,
        max_number_players,
    };
    let new_available_room = AvailableRooms {
        id: Uuid::new_v4(),
        room_id,
        number_of_players: 1,
        is_open: true,
    };

    Ok((room, new_available_room))
}
