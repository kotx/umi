pub mod models;

use serenity::prelude::TypeMapKey;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, error::Error};
use tracing::info;

pub async fn get_pool() -> Result<PgPool, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    info!("Connected to the database");
    Ok(pool)
}

pub struct PgPoolContainer;
impl TypeMapKey for PgPoolContainer {
    type Value = PgPool;
}
