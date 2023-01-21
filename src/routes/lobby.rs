use std::sync::{Arc, Mutex};

use actix::Addr;
use actix_web::{http::header::LOCATION, web::{self, Data},HttpRequest, HttpResponse,};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{utils::{check_if_cookie_is_valid, open_file_return_http_response, FilesOptions}, model::{Room, AvailableRooms, RoomName, MaxNumberOfPlayers}, database, websockets::{EchoAvailableRoomsLobby, Lobby}};



pub async fn lobby_get(req: HttpRequest, connection: web::Data<PgPool>) -> HttpResponse {
    match check_if_cookie_is_valid(&req, connection).await {
        Ok(_) => open_file_return_http_response(&req, FilesOptions::Lobby).await,
        Err(e) => e,
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct UserInput{
    pub name: String,
    pub number_of_players:i32,
}

pub async fn lobby_post(req: HttpRequest, connection: web::Data<PgPool>, user_input: web::Form<UserInput>, rooms: Data<Arc<Mutex<Vec<AvailableRooms>>>>, lobby_srv: Data<Addr<Lobby>>)->HttpResponse{
    let user_uuid = match check_if_cookie_is_valid(&req, connection.clone()).await {
        Ok(u) => u,
        Err(e) => return e,
    };
    let (new_room, new_available_room) = validade_and_build_room(user_input.0).unwrap();

    match database::insert_room_and_available_room_db(&new_room, &new_available_room, &user_uuid, connection).await{
        Ok(_) => {
            rooms.lock().unwrap().push(new_available_room);
            lobby_srv.send(EchoAvailableRoomsLobby{ lobby_id: Uuid::parse_str("57a1396b-ac9d-4558-b356-1bf87246a14f").unwrap()}).await.unwrap();
            let url = format!("lobby/{}", new_room.id.to_string());
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
