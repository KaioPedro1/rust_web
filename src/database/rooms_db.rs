use crate::model::{Room, AvailableRooms};
use actix_web::web;
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;


pub async fn insert_room_and_available_room_db(new_room: &Room,
    new_available_room: &AvailableRooms,
    user_id:&Uuid, 
    connection: web::Data<PgPool>) 
->Result<(), sqlx::Error>{
    let mut tx = connection.get_ref().begin().await?;
    sqlx::query!(
        r#"INSERT INTO rooms (id, name, max_number_of_players, created_at) 
        VALUES ($1, $2, $3, $4)"#,
        new_room.id,
        new_room.name.as_ref(),
        new_room.max_number_players.as_ref(),
        Utc::now()
    )
    .execute(&mut tx)
    .await?;
    sqlx::query!(
        r#"INSERT INTO availablerooms (id, room_id, number_of_players, is_open) 
        VALUES ($1, $2, $3, $4)"#,
        new_available_room.id,
        new_room.id,
        new_available_room.number_of_players,
        new_available_room.is_open
    )
    .execute(&mut tx)
    .await?;
    sqlx::query!(
        r#"INSERT INTO connections (user_id, room_id, is_admin) 
        VALUES ($1, $2, $3)"#,
        user_id,
        new_room.id,
        true,
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await
}
