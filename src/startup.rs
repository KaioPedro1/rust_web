use std::net::TcpListener;
use actix_web::{dev::Server, App, HttpServer};
use crate::routes::{index,submit_form};


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server: Server = HttpServer::new(move || 
    App::new()
        .service(index)
        .service(submit_form))
        .listen(listener)?
        .run();

    Ok(server)
}
