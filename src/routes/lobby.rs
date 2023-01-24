use std::sync::{Arc, Mutex};

use actix::Addr;
use actix_web::{http::header::LOCATION, web::{self, Data},HttpRequest, HttpResponse,};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{utils::{check_if_cookie_is_valid,LOBBY_UUID, open_file_return_http_response_with_cache, FilesOptions}, model::{Room, AvailableRooms, RoomName, MaxNumberOfPlayers}, database, websockets::{EchoAvailableRoomsLobby, Lobby, Disconnect}};



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

pub async fn lobby_post(req: HttpRequest, connection: web::Data<PgPool>, user_input: web::Form<UserInput>, rooms: Data<Arc<Mutex<Vec<AvailableRooms>>>>, lobby_srv: Data<Addr<Lobby>>)->HttpResponse{
    let user_uuid = match check_if_cookie_is_valid(&req, connection.clone()).await {
        Ok(u) => u,
        Err(e) => return e,
    };
    let (new_room, new_available_room) = validade_and_build_room(user_input.0).unwrap();

    match database::insert_room_and_available_room_db(&new_room, &new_available_room, &user_uuid, connection).await{
        /*TODO: melhorar a lógica por trás disso, parece gambiarra, as intruções abaixo fazem o seguinte:
        1)faz o parse do uuid do loby que é uma constante, é necessário pois vamos enviar mensagens para o actor lobby
        2)adiciona no array state que é o campo rooms, esse array armazena todas as salas disponiveis
        3)envia duas mensagens para o lobby, primeiro é para enviar para todos sockets conectados ao lobby informando as salas disponiveis
        4)segunda mensagem é para desconectar o usuario do websocket, estava dando um erro pois o redirecionamento acontencia antes do disconnect, 
        então o usuario entrava na sala e desconectava esse aqui é um workarround, não é o ideal
        5)envia o httpresponse para redirecionar o usuario
        */
        Ok(_) => {
            let lobby_id = Uuid::parse_str(LOBBY_UUID).unwrap();
            rooms.lock().unwrap().push(new_available_room);
            lobby_srv.send(EchoAvailableRoomsLobby{ lobby_id }).await.unwrap();
            lobby_srv.send(Disconnect{ room_id: lobby_id, id: user_uuid }).await.unwrap();
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
