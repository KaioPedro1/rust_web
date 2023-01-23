use crate::model::{User, UserName};
use actix_web::web;
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_user_db(new_user: &User, connection: web::Data<PgPool>) {
    match sqlx::query!(
        r#"INSERT INTO users (id, name, subscribed_at) 
        VALUES ($1, $2, $3)"#,
        new_user.id,
        new_user.name.as_ref(),
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to execute query: {}", e);
        }
    };
}

pub async fn check_user_id_db(
    user_uuid: Uuid,
    connection: web::Data<PgPool>,
) -> Result<User, sqlx::Error> {
    let result = sqlx::query!(r#"SELECT name,id FROM users WHERE id = $1"#, user_uuid)
        .fetch_one(connection.get_ref())
        .await?;
    Ok(User {
        name: UserName(result.name.to_string()),
        id: result.id,
    })
}


