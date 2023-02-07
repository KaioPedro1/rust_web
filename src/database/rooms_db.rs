use crate::model::{AvailableRooms, MaxNumberOfPlayers, Room, RoomName};
use actix_web::web;
use sqlx::types::chrono::Utc;
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

pub async fn insert_room_and_available_room_db(
    new_room: &Room,
    new_available_room: &AvailableRooms,
    user_id: &Uuid,
    connection: web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
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

pub async fn check_room_exist_in_available_rooms_table(
    room_uuid: Uuid,
    connection: web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    let _result = sqlx::query!(
        r#"SELECT Rooms.id FROM Rooms, AvailableRooms 
        WHERE AvailableRooms.room_id = $1 
        AND AvailableRooms.number_of_players < Rooms.max_number_of_players 
        AND AvailableRooms.is_open = true"#,
        room_uuid
    )
    .fetch_one(connection.get_ref())
    .await?;
    Ok(())
}

pub async fn initial_rooms_state(pool: Pool<Postgres>) -> Result<Vec<Room>, sqlx::Error> {
    let result_query = sqlx::query!(
        r#"SELECT rooms.*
        FROM availablerooms, rooms 
        WHERE availablerooms.room_id = rooms.id 
        AND availablerooms.is_open=true"#
    )
    .fetch_all(&pool)
    .await?;

    let parse_to_room: Vec<Room> = result_query
        .into_iter()
        .map(|x| -> Room {
            Room {
                id: x.id,
                name: RoomName(x.name),
                max_number_players: MaxNumberOfPlayers(x.max_number_of_players),
            }
        })
        .collect();

    Ok(parse_to_room)
}
