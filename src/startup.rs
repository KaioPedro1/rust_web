use crate::{
    routes::{
        lobby_get, lobby_post, room_get, root_get, root_post, ws_lobby_get,
        ws_room_get, room_delete,
    },
    websockets::Lobby, redis_utils::{self},
};
use actix::Actor;
use actix_files as fs;
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};


use redis::Connection;
use sqlx::{Pool, Postgres};
use std::{
    net::TcpListener, sync::{Mutex}
};

pub fn run(
    listener: TcpListener,
    db_pool: Pool<Postgres>,
    redis_connection: Connection,
    pub_sub: Connection,
) -> Result<Server, std::io::Error> {
    let redis = web::Data::new (Mutex::new(redis_utils::RedisState::new( redis_connection, db_pool.clone())));
    let postgres_pool = web::Data::new(db_pool);
    let lobby_ws_server = web::Data::new(Lobby::new(redis.clone()).start());
    redis_utils::create_channels_and_subscribe(pub_sub, lobby_ws_server.clone());
    let server: Server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(postgres_pool.clone())
            .app_data(redis.clone())
            .app_data(lobby_ws_server.clone())
            //.app_data(available_rooms_mutex.clone())
            .service(fs::Files::new("/static/css", "static/css"))
            .service(fs::Files::new("/static/js", "static/js"))
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
                        web::scope("/{room_uuid}")
                            .route("", web::get().to(room_get))
                            .route("", web::delete().to(room_delete))
                            .route("/ws", web::get().to(ws_room_get)),
                    ),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
