use std::net::TcpListener;
use bongo::{startup::run, configuration::{Settings, DatabaseSettings}, model::AvailableRooms};
use actix_web::dev::Server;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use bongo::configuration::get_local_configuration;
use uuid::Uuid;
pub struct TestApp{
    pub address: String,
    pub db_pool:PgPool
}

pub async fn init_app() -> TestApp{
    /*pega config local do arquivo configuration.rs e cria uma conexão com o banco de dados*/
    let mut config: Settings= get_local_configuration().expect("Failed to read configuration file");
    //gera um nome aleatorio para o banco de dado, necessario para o ambiente de teste
    config.database.database_name=Uuid::new_v4().to_string();
    //cria uma pool de connection passando um nome aleatorio 
    let db_pool = configure_database_init(&config.database).await;
    
    let available_rooms = sqlx::query_as!(AvailableRooms,r#"SELECT * from availablerooms"#)
        .fetch_all(&db_pool)
        .await
        .expect("Failed to query available rooms");
        
    /*Gera porta aleatoria, por isso é passado a porta 0 no TcpListener::bind("127.0.0.1:0") */
    let listener:TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port:u16 = listener.local_addr().unwrap().port();
    /*instancia o server em uma thread separada para não criar um server infinito que bloqueia a execução dos testes */
    let server:Server = run(listener, db_pool.clone(), available_rooms).expect("Failed to initialize");
    let _ = tokio::spawn(server);
    /*retora o endereço*/
    let address = format!("http://127.0.0.1:{}",port);

    TestApp{
        address,
        db_pool
    }
}
async fn configure_database_init(config:&DatabaseSettings)->PgPool{
    //cria o banco de dados novo com o nome aleatorio que é alterado na função init_app
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection.execute(format!(r#"CREATE DATABASE "{}";"#,config.database_name).as_str())
        .await
        .expect("Failed to initilize a new db");
    
    //migra o banco de dados
    let db_pool= PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Failed to migrate the db");
    
    db_pool
}