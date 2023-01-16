use bongo::configuration::{Settings, get_local_configuration};
use sqlx::PgPool;
use std::net::TcpListener;
use bongo::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config:Settings = get_local_configuration().expect("Failed to read configuration file");
    let connection_pool= PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address:String = format!("127.0.0.1:{}", config.app_port);
    let listener:TcpListener = TcpListener::bind(address).expect("Failed to bind random port");
    
    run(listener, connection_pool)?.await
}