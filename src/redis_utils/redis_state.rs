use serde_json;
use std::sync::{Arc, Mutex};

use crate::{
    database,
    model::{ConnectionMessage, Room},
};
use redis::{Commands, Connection, RedisError};
use sqlx::{Pool, Postgres};

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
    pub fn get_all_rooms_from_redis(&mut self) -> Result<Vec<Room>, RedisError> {
        let vec: Vec<String> = self.connection.lock().unwrap().hvals("AvailableRooms")?;

        let deserialize_vec = vec
            .iter()
            .map(|x| {
                let y: Room = serde_json::from_str(x).unwrap();
                y
            })
            .collect::<Vec<Room>>();
        Ok(deserialize_vec)
    }
    pub fn get_all_connections_from_redis(
        &mut self,
    ) -> Result<Vec<ConnectionMessage>, RedisError> {
        let vec: Vec<String> = self.connection.lock().unwrap().hvals("Connections")?;

        let deserialize_vec = vec
            .iter()
            .map(|x| {
                let t: ConnectionMessage = serde_json::from_str(x).unwrap();
                t
            })
            .collect::<Vec<ConnectionMessage>>();
        Ok(deserialize_vec)
    }
    pub fn insert_room_publish_to_lobby(
        &mut self,
        field: String,
        value: String,
        message: String,
    ) -> Result<(), RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();
        let _: () = conn_locked.hset("AvailableRooms", field, value)?;
        let _: () = conn_locked.publish("lobby", message)?;

        Ok(())
    }
    pub fn remove_room_publish_to_lobby(
        &mut self,
        field: String,
        message: String,
    ) -> Result<(), RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();
        let _: () = conn_locked.hdel("AvailableRooms", field)?;
        let _: () = conn_locked.publish("lobby", message)?;

        Ok(())
    }
    pub fn insert_connection(
        &mut self,
       new_connection: ConnectionMessage
    ) -> Result<(), RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();
     
        let field = new_connection.user_id.to_string() + "/"+ &new_connection.room_id.to_string();
        let value = serde_json::to_string(&new_connection).unwrap();
        let _: () = conn_locked.hset("Connections", field,value)?;

        Ok(())
    }
    pub fn remove_connection(
        &mut self,
        field: String
    ) -> Result<(), RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();
        let _: () = conn_locked.hdel("Connections", field)?;
        Ok(())
    }
}

pub async fn set_initial_redis_state(
    con: &mut Connection,
    pool: Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    let vec_rooms = database::initial_rooms_state(pool.clone()).await?;
    let vec_connections = database::connections_initial_state(pool).await?;

    for room in vec_rooms {
        let serialized = serde_json::to_string(&room).unwrap();
        let _: () = con
            .hset("AvailableRooms", room.id.to_string(), serialized)
            .unwrap();
    }
    for conn in vec_connections {
        let serialized = serde_json::to_string(&conn).unwrap();
        let map_key = conn.user_id.to_string() + "/" + &conn.room_id.to_string();
        let _: () = con.hset("Connections", map_key, serialized).unwrap();
    }
    Ok(())
}
