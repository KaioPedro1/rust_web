use std::sync::Arc;

use actix_web::web::{self};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

use crate::model::{ConnectionMessage, ConnectionTuple};

pub async fn insert_connection_db(
    room_uuid: Uuid,
    user_uuid: Uuid,
    connection: web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO connections (user_id, room_id, is_admin) 
            VALUES ($1, $2, $3)"#,
        user_uuid,
        room_uuid,
        false
    )
    .execute(connection.get_ref())
    .await?;
    Ok(())
}

pub async fn get_connection_by_room_and_user(
    room_uuid: Uuid,
    user_uuid: Uuid,
    connection: web::Data<PgPool>,
) -> Result<ConnectionTuple, sqlx::Error> {
    let conn_result = sqlx::query_as!(
        ConnectionTuple,
        r#"SELECT * from Connections WHERE room_id = $1 AND user_id = $2"#,
        room_uuid,
        user_uuid
    )
    .fetch_one(connection.get_ref())
    .await?;
    Ok(conn_result)
}

pub async fn delete_room_connections_close_room(
    room_uuid: Uuid,
    connection: Arc<Pool<Postgres>>,
) -> Result<(), sqlx::Error> {
    let mut tx = connection.begin().await?;

    sqlx::query!(
        r#"UPDATE AvailableRooms SET is_open=false WHERE room_id = $1"#,
        room_uuid
    )
    .execute(&mut tx)
    .await?;
    sqlx::query!(r#"DELETE FROM Connections WHERE room_id = $1"#, room_uuid)
        .execute(&mut tx)
        .await?;

    tx.commit().await
}

pub async fn disconnect_user_and_set_new_admin_if_needed(
    user_uuid: Uuid,
    new_admin_uuid: Uuid,
    room_uuid: Uuid,
    connection: Arc<PgPool>,
) -> Result<(), sqlx::Error> {
    let mut tx = connection.begin().await?;
    let user = sqlx::query!(
        r#"SELECT is_admin FROM Connections WHERE user_id= $1 AND room_id = $2"#,
        user_uuid,
        room_uuid
    )
    .fetch_one(&mut tx)
    .await?;
    
    if user.is_admin {
        sqlx::query!(
            r#"UPDATE Connections SET is_admin=true WHERE user_id= $1 AND room_id = $2"#,
            new_admin_uuid,
            room_uuid
        )
        .execute(&mut tx)
        .await?;
    }

    sqlx::query!(
        r#"DELETE FROM Connections WHERE user_id = $1 AND room_id = $2"#,
        user_uuid,
        room_uuid,
    )
    .execute(&mut tx)
    .await?;
    tx.commit().await?;

    Ok(())
}

pub async fn connections_initial_state(
    pool: Pool<Postgres>,
) -> Result<Vec<ConnectionMessage>, sqlx::Error> {
    sqlx::query_as!(
        ConnectionMessage,
        r#"SELECT connections.*, users.name 
        FROM users, connections 
        WHERE users.id = connections.user_id"#
    )
    .fetch_all(&pool)
    .await
}
