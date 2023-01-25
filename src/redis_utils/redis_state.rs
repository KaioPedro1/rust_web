use serde_json;
use std::{sync::{Arc, Mutex}};

use crate::{
    model::{ConnectionsInitialState, RoomsInitialState},
};
use redis::{Commands, Connection, RedisError};
use sqlx::{ Pool, Postgres};

pub struct RedisState {
    pub connection: Arc<Mutex<Connection>>,
    pub pg_pool: Arc<Pool<Postgres>>,
}

impl RedisState {
    pub fn new(conn: Connection, pool: Pool<Postgres>) -> RedisState {
        RedisState {
            connection: Arc::new(Mutex::new(conn)),
            pg_pool: Arc::new(pool),
        }
    }
     /* MELHORAR ISSO AUQI, ESTOU DESERIALIZANDO E DEPOIS SERIALIZANDO NOVAMENTE. ERRADO! */
    pub fn get_all_rooms_from_redis(&mut self)->Result<Vec<RoomsInitialState>, RedisError>{
        let vec : Vec<String> = self
            .connection
            .lock()
            .unwrap()
            .hvals("AvailableRooms")?;

        let deserialize_vec = vec
            .iter()
            .map(|x| {
                let y :RoomsInitialState = serde_json::from_str(x).unwrap();
                y
            })
            .collect::<Vec<RoomsInitialState>>();
        Ok(deserialize_vec)
    }
    pub fn get_all_connections_from_redis(&mut self)->Result<Vec<ConnectionsInitialState>, RedisError>{
        let vec : Vec<String> = self
            .connection
            .lock()
            .unwrap()
            .hvals("Connections")?;

    let deserialize_vec = vec
        .iter()
        .map(|x| {
            let t :ConnectionsInitialState = serde_json::from_str(x).unwrap();
            t
        })
        .collect::<Vec<ConnectionsInitialState>>();
    Ok(deserialize_vec)
    }
}

pub async fn set_initial_redis_state(con: &mut Connection, pool:Pool<Postgres>) {

    let vec_rooms = sqlx::query_as!(
        RoomsInitialState,
        r#"SELECT availablerooms.*, rooms.name, rooms.max_number_of_players 
    FROM availablerooms, rooms 
    WHERE availablerooms.room_id = rooms.id 
    AND availablerooms.is_open=true"#
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to query available rooms");
    let vec_connections = sqlx::query_as!(
        ConnectionsInitialState,
        r#"SELECT connections.*, users.name 
    FROM users, connections 
    WHERE users.id = connections.user_id"#
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to query available rooms");

    for room in vec_rooms{
        let serialized = serde_json::to_string(&room).unwrap();
        let _: () = con
            .hset("AvailableRooms",room.room_id.to_string(), serialized)
            .unwrap();
    }
   for conn in vec_connections{
        let serialized = serde_json::to_string(&conn).unwrap();
        let map_key = conn.user_id.to_string()+"/"+&conn.room_id.to_string();
        let _: () = con
            .hset("Connections",map_key, serialized)
            .unwrap();
   }
}

