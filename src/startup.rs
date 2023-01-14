use std::net::TcpListener;
use actix_web::{dev::Server, App, HttpServer, web};
use sqlx::{Pool, Postgres};
use crate::routes::{config_root,lobby_config};
use actix_files as fs;


pub fn run(listener: TcpListener, db_pool: Pool<Postgres>) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_pool);
    let server: Server = HttpServer::new(move|| {
        App::new()
            .service(fs::Files::new("/static/css", "static/css"))
            .configure(config_root)
            .configure(lobby_config)
            .app_data(connection.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}
