use std::sync::mpsc::{self};

use actix::Addr;
use actix_web::web::Data;
use redis::Connection;

use crate::websockets::{lobby_messages::LobbyNotification, Lobby};

pub fn create_channels_and_subscribe(
    mut pub_sub_conn: Connection,
    addr_actor_lobby: Data<Addr<Lobby>>,
) {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut pubsub = pub_sub_conn.as_pubsub();
        pubsub.subscribe("lobby").unwrap();
        loop {
            if let Ok(msg) = pubsub.get_message() {
                let payload: String = msg.get_payload().unwrap();
                let _ = tx.send(payload);
            }
        }
    });
    std::thread::spawn(move || loop {
        if let Ok(payload) = rx.recv() {
            let message_serialized: LobbyNotification = serde_json::from_str(&payload).unwrap();
            match message_serialized.msg_type {
                crate::model::MessageLobbyType::UpdateRoom => {
                    addr_actor_lobby.do_send(message_serialized)
                }
                crate::model::MessageLobbyType::Initial => {
                    println!("Error, thats not suposse to be here")
                }
                crate::model::MessageLobbyType::UpdatePlayer => {
                    addr_actor_lobby.do_send(message_serialized)
                }
            }
        }
    });
}
