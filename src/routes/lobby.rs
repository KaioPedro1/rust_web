use actix_web::{
    http::header::LOCATION,
    web::{self, Data},
    HttpRequest, HttpResponse,
};
use sqlx::PgPool;
use std::sync::Mutex;
use uuid::Uuid;

use crate::{
    database,
    middleware::Authenticated,
    model::{self, AvailableRooms, ConnectionMessage, Room, RoomCapacity, RoomName},
    redis_utils::RedisState,
    utils::{open_file_return_http_response_with_cache, FilesOptions},
    websockets::lobby_messages::LobbyNotification,
};

pub async fn lobby_get(req: HttpRequest, _: Authenticated) -> HttpResponse {
    open_file_return_http_response_with_cache(&req, FilesOptions::Lobby).await
}

#[derive(serde::Deserialize, Debug)]
pub struct UserInput {
    pub name: String,
    pub room_capacity: i32,
}

pub async fn lobby_post(
    connection: web::Data<PgPool>,
    user_input: web::Form<UserInput>,
    redis: Data<Mutex<RedisState>>,
    auth: Authenticated,
) -> HttpResponse {
    let (user_uuid, name, avatar_id) = match auth.parse() {
        Some(sucess) => sucess,
        None => return HttpResponse::InternalServerError().finish(),
    };

    let (new_room, new_available_room) = validade_and_build_room(user_input.0).unwrap();

    match database::insert_room_and_available_room_db(
        &new_room,
        &new_available_room,
        &user_uuid,
        connection,
    )
    .await
    {
        Ok(_) => {
            let room = new_room.clone();
            let user = ConnectionMessage {
                user_id: user_uuid,
                room_id: room.id,
                avatar_id,
                is_admin: true,
                name,
                position: 0,
            };
            let serialized_notification = serde_json::to_string(&LobbyNotification {
                msg_type: crate::model::MessageLobbyType::UpdateRoom,
                action: Some(crate::model::ActionLobbyType::Add),
                room: crate::model::RoomTypes::Room(new_room),
                user: Some(model::UserTypes::Connection(user.clone())),
                sender_uuid: user_uuid,
            })
            .unwrap();
            let serialized_room = serde_json::to_string(&room).unwrap();

            let mut redis_unlock = redis.lock().unwrap();
            redis_unlock
                .insert_room_publish_to_lobby(
                    room.id.to_string(),
                    serialized_room,
                    serialized_notification,
                )
                .unwrap();
            redis_unlock.insert_connection(user).unwrap();
            drop(redis_unlock);
            let url = format!("lobby/{}", room.id);
            HttpResponse::Found()
                .append_header((LOCATION, url))
                .finish()
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

fn validade_and_build_room(input: UserInput) -> Result<(Room, AvailableRooms), String> {
    let room_id = Uuid::new_v4();
    let room_name = RoomName::parse(input.name)?;
    let room_capacity = RoomCapacity::parse(input.room_capacity)?;
    let room = Room {
        id: room_id,
        name: room_name,
        room_capacity,
    };
    let new_available_room = AvailableRooms {
        id: Uuid::new_v4(),
        room_id,
        number_of_players: 1,
        is_open: true,
    };

    Ok((room, new_available_room))
}
