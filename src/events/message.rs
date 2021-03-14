use serenity::{async_trait, client::{Context, EventHandler}, model::prelude::*};
use crate::utils::db::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.is_private() { return };

        let content = msg.content_safe(&ctx.cache).await;
        if content.trim().is_empty() { return; }
        if content.starts_with("u~") { return; } // don't process commands

        let pool = get_pool(&ctx).await;
        if get_user_opt(&pool, msg.author.id.to_string()).await.unwrap_or_default() {
            create_message(&pool, msg.id.to_string(), msg.author.id.to_string(), content).await.ok();
        }
    }
}
