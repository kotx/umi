use crate::utils::db::*;
use serenity::{
    client::Context,
    framework::standard::{macros::*, CommandResult},
    model::channel::{ChannelType, Message},
};
use std::time::Duration;
use tracing::{debug, info};

#[command]
pub async fn collect(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = get_pool(&ctx).await;

    if !get_user_opt(&pool, msg.author.id.to_string())
        .await
        .unwrap_or_default()
    {
        msg.channel_id
            .say(
                &ctx.http,
                "You haven't been added to the database. Use the `?opt in` command for details.",
            )
            .await?;

        return Ok(());
    }

    let mut q = msg
        .reply(
            &ctx,
            "This command will collect all of the messages in this server into the database. Are you sure you want to do this? (Y/n)",
        )
        .await?;

    if let Some(answer) = &msg
        .author
        .await_reply(&ctx)
        .timeout(Duration::from_secs(20))
        .await
    {
        if answer.content.to_lowercase() == "y" {
            let mut status = answer
                .reply(ctx, "Collecting data, this may take a long time...")
                .await?;

            let typing = msg.channel_id.start_typing(&ctx.http);

            let current_user = ctx.http.get_current_user().await?;
            let mut channel_count: usize = 0;
            let mut count: usize = 0;

            let channels = msg.guild_id.unwrap().channels(&ctx.http).await?;
            let channels_len = channels.len();

            for (_, channel) in channels {
                // TODO: channel blacklist
                if channel.kind != ChannelType::Text {
                    continue;
                }

                let permissions = channel
                    .permissions_for_user(&ctx.cache, current_user.id)
                    .await?;

                if permissions.read_message_history() {
                    let mut before_id = msg.id;

                    loop {
                        let messages_result = channel
                            .messages(&ctx.http, |m| m.before(before_id).limit(100))
                            .await;

                        if messages_result.is_err() {
                            break;
                        }
                        let messages = messages_result.unwrap();

                        for message in &messages {
                            if message.author.id != msg.author.id { continue; }

                            let content = message.content_safe(&ctx.cache).await;
                            if content.is_empty() {
                                continue;
                            }
                            if content.starts_with("u~") {
                                continue;
                            } // don't process commands

                            info!("{}/{}: {}", message.author.id, message.id, content);

                            let create_msg = create_message(
                                &pool,
                                message.id.to_string(),
                                message.author.id.to_string(),
                                content,
                            )
                            .await;

                            if let Some(e) = create_msg.err() {
                                debug!("Error adding message to database: {}", e);
                            } else {
                                count += 1;
                            }
                        }

                        if messages.len() < 100 {
                            break;
                        }

                        before_id = messages.first().unwrap().id;
                    }
                }

                channel_count += 1;

                status
                    .edit(&ctx, |s| {
                        s.embed(|f| {
                            f.description(format!(
                                "{} messages in {}/{} channels processed.",
                                count, channel_count, channels_len
                            ))
                        })
                    })
                    .await
                    .ok();
            }

            typing.ok().and_then(|t| t.stop());

            q.reply(&ctx, format!("Added {} messages.", count)).await?;
        } else {
            q.edit(&ctx, |m| m.content("Abort.")).await?;
        }
    } else {
        q.edit(&ctx, |m| m.content("Timed out.")).await?;
    };

    Ok(())
}
