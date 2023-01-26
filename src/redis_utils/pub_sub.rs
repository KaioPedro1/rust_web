use std::sync::mpsc::{self};

use actix::Addr;
use actix_web::web::Data;
use redis::Connection;

use crate::{websockets::{Lobby, LobbyNotification}};


pub fn create_channels_and_subscribe(mut pub_sub_conn:Connection, addr_actor_lobby: Data<Addr<Lobby>>){
    let (tx, rx) = mpsc::channel(); 
    std::thread::spawn(move || {    
        let mut pubsub = pub_sub_conn.as_pubsub();
        pubsub.subscribe("lobby").unwrap();
        loop {
            match pubsub.get_message() {
                Ok(msg) => {
                    let payload: String = msg.get_payload().unwrap();
                    let _ = tx.send(payload);
                }
                _ => {}
            }
        }
    }); 
     std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(payload) => {
                    let message_serialized: LobbyNotification = serde_json::from_str(&payload).unwrap();
                    match message_serialized.msg_type{
                        crate::model::MessageLobbyType::Update => {
                            addr_actor_lobby.do_send(message_serialized);
                           },
                        crate::model::MessageLobbyType::Initial => println!("Error, thats not suposse to be here"),
                    }
                }
                _ => {}
            }
        }
    });
   
}

