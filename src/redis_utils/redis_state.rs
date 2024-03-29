use serde_json;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::{
    database,
    model::{ConnectionMessage, Room, UserTypes},
    websockets::Disconnect,
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
    pub fn get_all_connections_from_redis(&mut self) -> Result<Vec<ConnectionMessage>, RedisError> {
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
    pub fn get_room_by_id(&mut self, room_id: Uuid) -> Result<Room, RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();
        let field = room_id.to_string();
        let value: String = conn_locked.hget("AvailableRooms", field)?;
        let room: Room = serde_json::from_str(&value).unwrap();

        Ok(room)
    }
    pub fn insert_room_publish_to_lobby(
        &mut self,
        field: String,
        value: String,
        message: String,
    ) -> Result<(), RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();
        conn_locked.hset("AvailableRooms", field, value)?;
        conn_locked.publish("lobby", message)?;

        Ok(())
    }
    pub fn remove_room_publish_to_lobby(
        &mut self,
        field: String,
        message: String,
    ) -> Result<(), RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();
        conn_locked.hdel("AvailableRooms", field)?;
        conn_locked.publish("lobby", message)?;

        Ok(())
    }
    pub fn get_coonections_by_room_id(
        &mut self,
        room_id: Uuid,
    ) -> Result<Vec<ConnectionMessage>, RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();
        let pattern = format!("*/{}", room_id);

        let iter: redis::Iter<String> = conn_locked.hscan_match("Connections", pattern)?;
        let vec: Vec<ConnectionMessage> = iter
            .filter(|x| x.starts_with('{'))
            .map(|x| {
                let str = x;
                serde_json::from_str(&str).unwrap()
            })
            .collect();

        Ok(vec)
    }

    pub fn insert_connection(
        &mut self,
        new_connection: ConnectionMessage,
    ) -> Result<(), RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();

        let field = new_connection.user_id.to_string() + "/" + &new_connection.room_id.to_string();
        let value = serde_json::to_string(&new_connection).unwrap();
        conn_locked.hset("Connections", field, value)?;

        Ok(())
    }
    pub fn remove_connection(&mut self, field: String) -> Result<(), RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();
        conn_locked.hdel("Connections", field)?;
        Ok(())
    }
    pub fn publish_connection_to_lobby(&mut self, message: String) -> Result<(), RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();

        conn_locked.publish("lobby", message)?;
        Ok(())
    }
    pub fn update_admin(&mut self, room_id: Uuid, new_admin_id: Uuid) -> Result<(), RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();

        let vec: Vec<String> = conn_locked.hvals("Connections")?;
        // let t: ConnectionMessage = serde_json::from_str(x).unwrap();
        for conn_string in vec {
            let mut deser_conn: ConnectionMessage = serde_json::from_str(&conn_string).unwrap();
            if deser_conn.room_id == room_id && deser_conn.user_id == new_admin_id {
                deser_conn.is_admin = true;
                let msg = serde_json::to_string(&deser_conn).unwrap();
                conn_locked.hset(
                    "Connections",
                    deser_conn.user_id.to_string() + "/" + &deser_conn.room_id.to_string(),
                    msg,
                )?;
            }
        }
        Ok(())
    }
    pub fn remove_connection_publish_user(
        &mut self,
        data: Disconnect,
        new_admin: Option<UserTypes>,
        parsed_msg: String,
    ) {
        let r1 = self.remove_connection(data.id.to_string() + "/" + &data.room_id.to_string());
        let r3 = self.publish_connection_to_lobby(parsed_msg);
        if r1.is_err() || r3.is_err() {
            println!("Error on remove_connection_update_admin_publish_user");
        };
        if let Some(update_admin) = new_admin {
            match update_admin {
                UserTypes::Uuid(uuid) => {
                    if let Err(e) = self.update_admin(data.room_id, uuid) {
                        println!("Error on update_admin: {}", e);
                    }
                }
                _ => println!("Error on update_admin: new_admin is not Uuid"),
            }
        }
    }
    pub fn get_connection_by_id(
        &mut self,
        room_id: Uuid,
        user_id: Uuid,
    ) -> Result<ConnectionMessage, RedisError> {
        let mut conn_locked = self.connection.try_lock().unwrap();
        let field = user_id.to_string() + "/" + &room_id.to_string();
        let conn_data: String = conn_locked.hget("Connections", field)?;
        let parsed_data: ConnectionMessage = serde_json::from_str(&conn_data).unwrap();
        Ok(parsed_data)
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
