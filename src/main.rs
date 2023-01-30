use bongo::{configuration::{Settings, get_local_configuration}, redis_utils};
use env_logger::Env;

use sqlx::PgPool;

use std::net::TcpListener;
use bongo::startup::run;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config:Settings = get_local_configuration().expect("Failed to read configuration file");

    let mut redis_connection = redis::Client::open(config.redis.redis_url.clone())
        .expect("Failed to open redis, invalid ip")
        .get_connection()
        .expect("Failed to get a connection channel");

    let pubsub_conn = redis::Client::open(config.redis.redis_url)
        .expect("Failed to open redis, invalid ip")
        .get_connection()
        .expect("Failed to get a connection channel");

    let connection_pool= PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    

    redis_utils::set_initial_redis_state(&mut redis_connection, connection_pool.clone()).await.expect("Failed to fetch initial data");

    let address:String = format!("127.0.0.1:{}", config.app_port);
    let listener:TcpListener = TcpListener::bind(address).expect("Failed to bind random port");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    run(listener, connection_pool, redis_connection, pubsub_conn, config.jwt)?.await
}

