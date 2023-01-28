use std::sync::Mutex;

use crate::{
    database,
    redis_utils::RedisState,
    utils::{check_if_cookie_is_valid, open_file_return_http_response_with_cache, FilesOptions},
    websockets::{Lobby, RoomNotification, LobbyNotification}, model::{ConnectionMessage, self},
};
use actix::Addr;
use actix_web::{
    http::header::LOCATION,
    web::{self, Data},
    HttpRequest, HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RoomPath {
    pub room_uuid: Uuid,
}

pub async fn room_get(
    req: HttpRequest,
    connection: web::Data<PgPool>,
    info: web::Path<RoomPath>,
    redis: Data<Mutex<RedisState>>
) -> HttpResponse {
    let (user_uuid,name) = match check_if_cookie_is_valid(&req, connection.clone()).await {
        Ok(uuid) => uuid,
        Err(e) => return e,
    };

    let room_uuid = info.room_uuid;
    if let Err(_) =
        database::check_room_exist_in_available_rooms_table(room_uuid, connection.clone()).await
    {
        return HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, "/lobby"))
            .finish();
    }

    match database::get_connection_by_room_and_user(room_uuid, user_uuid, connection.clone()).await
    {
        Ok(_) => open_file_return_http_response_with_cache(&req, FilesOptions::Room).await,
        Err(_) => {
            match database::insert_connection_db(room_uuid, user_uuid, connection.clone()).await {
                Ok(_) => {
                    let mut redis_unlock = redis
                        .lock()
                        .unwrap();    
                    let message = serde_json::to_string(&LobbyNotification{ 
                            msg_type: crate::model::MessageLobbyType::UpdatePlayer, 
                            action:Some(crate::model::ActionLobbyType::Enter), 
                            room: model::RoomTypes::Uuid(room_uuid), 
                            user: Some(model::UserTypes::Connection(ConnectionMessage { user_id: user_uuid, room_id: room_uuid, is_admin: false, name:name.clone() })), 
                            sender_uuid: user_uuid
                            }
                        )
                        .unwrap();
                    redis_unlock
                        .insert_connection(ConnectionMessage{ 
                            user_id: user_uuid, 
                            room_id: room_uuid, 
                            is_admin: false, 
                            name})
                        .unwrap();
                    redis_unlock
                        .publish_connection_to_lobby(message)
                        .unwrap();
                        
                    HttpResponse::Ok()
                        .append_header(("Cache-control", "no-cache"))
                        .body(include_str!("../../static/room.html"))
                    },
                Err(_) => HttpResponse::TemporaryRedirect()
                    .append_header((LOCATION, "/lobby"))
                    .finish(),
            }
        }
    }
}

pub async fn room_delete(
    req: HttpRequest,
    connection: web::Data<PgPool>,
    info: web::Path<RoomPath>,
    lobby_srv: Data<Addr<Lobby>>,
    redis: Data<Mutex<RedisState>>,
) -> HttpResponse {
    let (user_uuid,_) = match check_if_cookie_is_valid(&req, connection.clone()).await {
        Ok(uuid) => uuid,
        Err(e) => return e,
    };
    match database::get_connection_by_room_and_user(info.room_uuid, user_uuid, connection.clone())
        .await
    {
        Ok(conn_tuple) => {
            if conn_tuple.is_admin == true {
                let conn_pull = redis.lock().unwrap().pg_pool.clone();
                match database::delete_room_connections_close_room(conn_tuple.room_id, conn_pull)
                    .await
                {
                    Ok(_) => {
                        /* remoção da sala é feita por redirecionamento do usuario, se o usuario falhar em receber a mensagem abaixo, ou for um sacana vai bugar
                         */
                        let _ = lobby_srv
                            .send(RoomNotification {
                                msg_type: crate::model::MessageRoomType::Redirect,
                                action: crate::model::ActionRoomType::Delete,
                                user: conn_tuple.user_id,
                                room: conn_tuple.room_id,
                                redirect: Some("lobby".to_string()),
                            })
                            .await;
                        HttpResponse::NoContent().finish()
                    }
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Unable to found user and room in connection"),
    }
}
