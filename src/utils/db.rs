use crate::data::{models::*, PgPoolContainer};
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use sqlx::PgPool;

pub async fn get_pool(ctx: &Context) -> PgPool {
    let data = ctx.data.read().await;
    data.get::<PgPoolContainer>()
        .expect("Expected PgPoolContainer in TypeMap")
        .clone()
}

pub async fn get_user(pool: &PgPool, id: String) -> Option<UmiUser> {
    match sqlx::query_as!(UmiUser, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(pool)
        .await
        .ok()?
    {
        Some(user) => Some(user),
        None => None,
    }
}

pub async fn get_user_opt(pool: &PgPool, id: String) -> Option<bool> {
    match sqlx::query!("SELECT opt FROM users WHERE id = $1", id)
        .fetch_optional(pool)
        .await
        .ok()?
    {
        Some(user) => Some(user.opt),
        None => None,
    }
}

pub async fn set_user_opt(pool: &PgPool, id: String, opt: bool) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE users SET opt = $1 WHERE id = $2", opt, id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_messages(pool: &PgPool, user: String) -> Result<Vec<UmiMessage>, sqlx::Error> {
    Ok(
        sqlx::query_as::<_, UmiMessage>("SELECT * FROM messages WHERE author = $1")
            .bind(user)
            .fetch_all(pool)
            .await?,
    )
}

#[derive(Deserialize, Serialize, Debug)]
struct CreateUser {
    id: String,
}

pub async fn create_user(pool: &PgPool, id: String) -> Result<String, sqlx::Error> {
    Ok(sqlx::query_as!(
        CreateUser,
        "INSERT INTO users(id) VALUES($1) RETURNING id",
        id
    )
    .fetch_one(pool)
    .await?
    .id)
}

pub async fn create_message(
    pool: &PgPool,
    id: String,
    author: String,
    content: String,
) -> Result<String, sqlx::Error> {
    Ok(sqlx::query_as::<_, UmiMessage>(
        "INSERT INTO messages(id, author, content) VALUES($1, $2, $3) RETURNING id",
    )
    .bind(id)
    .bind(author)
    .bind(content)
    .fetch_one(pool)
    .await?
    .id)
}
