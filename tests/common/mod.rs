use actix_web::dev::Server;
use bongo::configuration::get_local_configuration;
use bongo::{startup::run, configuration::{Settings}};
//use uuid::Uuid;
use std::net::TcpListener;


pub struct TestApp{
    pub address: String,
}

pub async fn init_app() -> TestApp{
    /*pega config local do arquivo configuration.rs e cria uma conexão com o banco de dados, vou usar no futuro*/
    let _config: Settings= get_local_configuration().expect("Failed to read configuration file");

    /*Gera porta aleatoria, por isso é passado a porta 0 no TcpListener::bind("127.0.0.1:0") */
    let listener:TcpListener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port:u16 = listener.local_addr().unwrap().port();
    /*instancia o server em uma thread separada para não criar um server infinito que bloqueia a execução dos testes */
    let server:Server = run(listener).expect("Failed to initialize");
    let _ = tokio::spawn(server);
    /*retora o endereço*/
    let address:String= format!("http://127.0.0.1:{}",port);

    TestApp{
        address
    }
}