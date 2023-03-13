use crate::model::{User, UserName, AvatarId};
use actix_web::web;
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_user_db(new_user: &User, connection: web::Data<PgPool>) {
    match sqlx::query!(
        r#"INSERT INTO users (id, name, avatar_id, subscribed_at) 
        VALUES ($1, $2, $3, $4)"#,
        new_user.id,
        new_user.name.as_ref(),
        new_user.avatar_id.as_ref(),
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
    name: &str,
    connection: web::Data<PgPool>,
) -> Result<User, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT name,id, avatar_id FROM users WHERE id = $1 AND name= $2"#,
        user_uuid,
        name
    )
    .fetch_one(connection.get_ref())
    .await?;
    Ok(User {
        name: UserName(result.name.to_string()),
        id: result.id,
        avatar_id: AvatarId(result.avatar_id),
    })
}
