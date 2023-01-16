use std::net::TcpListener;
use actix::Actor;
use actix_web::{dev::Server, App, HttpServer, web};
use sqlx::{Pool, Postgres};
use crate::{routes::{root_get,root_post,lobby_get, ws_connection,}, websockets::Lobby};
use actix_files as fs;


pub fn run(listener: TcpListener, db_pool: Pool<Postgres>) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);

    let lobby_ws_server= web::Data::new(Lobby::default().start());

    let server: Server = HttpServer::new(move|| {
        App::new()
            .app_data(connection.clone())
            .app_data(lobby_ws_server.clone())
            .service(fs::Files::new("/static/css", "static/css"))
            .service(ws_connection)
            .service(web::resource("/")
                .route(web::get().to(root_get))
                .route(web::post().to(root_post)
            ))
            .service(lobby_get)
            
    })
        .listen(listener)?
        .run();

    Ok(server)
}
