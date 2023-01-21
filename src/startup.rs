use crate::{
    model::AvailableRooms,
    routes::{room_delete, lobby_get, room_get, root_get, root_post, ws_lobby_get, lobby_post},
    websockets::Lobby,
};
use actix::Actor;
use actix_files as fs;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{Pool, Postgres};
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
};

pub fn run(
    listener: TcpListener,
    db_pool: Pool<Postgres>,
    available_rooms: Vec<AvailableRooms>,
) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let available_rooms_mutex = web::Data::new(Arc::new(Mutex::new(available_rooms)));
    let lobby_ws_server = web::Data::new(Lobby::new(available_rooms_mutex.clone()).start());

    let server: Server = HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .app_data(lobby_ws_server.clone())
            .app_data(available_rooms_mutex.clone())
            .service(fs::Files::new("/static/css", "static/css"))
            .service(
                web::resource("/")
                    .route(web::get().to(root_get))
                    .route(web::post().to(root_post)),
            )
            .service(
                web::scope("/lobby")
                    .route("", web::get().to(lobby_get))
                    .route("", web::post().to(lobby_post))
                    .route("/ws", web::get().to(ws_lobby_get))
                    .service(
                        web::resource("/{room_uuid}")
                            .route(web::get().to(room_get))
                            .route(web::delete().to(room_delete)),
                    ),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
