use std::sync::{Arc, Mutex};
use actix::Addr;
use actix_web::{http::header::{LOCATION},HttpRequest, web::{self, Data}, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{database, websockets::{Lobby, EchoAvailableRoomsLobby, RoomNotification}, model::{AvailableRooms}, utils::{check_if_cookie_is_valid, open_file_return_http_response_with_cache, FilesOptions, LOBBY_UUID}};
use crate::model::MessageType::Redirect;
use crate::model::ActionType::Delete;


#[derive(Deserialize)]
pub struct RoomPath {
    pub room_uuid: Uuid,
}

pub async fn room_get(
    req: HttpRequest,
    connection: web::Data<PgPool>,
    info: web::Path<RoomPath>,
) -> HttpResponse {
    let user_uuid = match check_if_cookie_is_valid(&req, connection.clone()).await {
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

    match database::get_connection_by_room_and_user(room_uuid, user_uuid, connection.clone()).await {
        Ok(_) => open_file_return_http_response_with_cache(&req, FilesOptions::Room).await,
        Err(_) => {
            match database::insert_connection_db(room_uuid, user_uuid, connection.clone()).await {
                Ok(_) =>     HttpResponse::Ok().append_header(("Cache-control","no-cache")).body(include_str!("../../static/room.html")),
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
    available_rooms_state: Data<Arc<Mutex<Vec<AvailableRooms>>>>,
) -> HttpResponse {
    let user_uuid = match check_if_cookie_is_valid(&req, connection.clone()).await {
        Ok(uuid) => uuid,
        Err(e) => return e,
    };
    match database::get_connection_by_room_and_user(info.room_uuid,user_uuid,connection.clone()).await{
        Ok(conn_tuple) => {
            if conn_tuple.is_admin==true{
                match database::delete_room_connections_close_room(conn_tuple.room_id, connection).await{
                    Ok(_) =>{
                        available_rooms_state
                            .lock()
                            .unwrap()
                            .retain(|r| r.room_id != conn_tuple.room_id);
                        let _a = lobby_srv.send(EchoAvailableRoomsLobby{ lobby_id: Uuid::parse_str(LOBBY_UUID).unwrap()}).await;
                        let _b = lobby_srv.send(RoomNotification{ 
                            msg_type: Redirect,
                            action: Delete, 
                            user: conn_tuple.user_id, 
                            room: conn_tuple.room_id, 
                            redirect: Some("lobby".to_string()) 
                        }).await;
                        HttpResponse::NoContent().finish()
                    },
                    Err(_) => HttpResponse::InternalServerError().finish(),

                }
            }
            else{
                HttpResponse::Unauthorized().finish()
            }
        },
        Err(_) =>  HttpResponse::BadRequest().body("Unable to found user and room in connection"),
    }
}