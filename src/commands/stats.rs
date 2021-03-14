use crate::utils::db::*;
use chrono::Utc;
use serenity::framework::standard::{ArgError, Args, CommandResult, macros::*};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("s", "%")]
async fn stats(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let pool = crate::utils::db::get_pool(ctx).await;

    let target = match args.single::<UserId>() {
        Ok(user) => user,
        Err(ArgError::Eos) => msg.author.id,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Could not get the target user. Check your arguments and try again").await?;
            return Ok(());
        }
    };

    let target_user = target.to_user(&ctx).await?;

    if let Some(user) = get_user(&pool, target.to_string()).await {
        let messages = get_messages(&pool, user.id).await?;

        msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| { e
                .author(|a| {
                    a.name(format!("Message statistics for {}#{}", target_user.name, &msg.author.discriminator))
                        .icon_url(&msg.author.avatar_url().unwrap_or(msg.author.default_avatar_url()))
                })
                .description("Here is some information about the messages they have sent")
                .field("Messages Tracked", messages.len(), true)
                .timestamp(Utc::now().to_rfc3339())
                .footer(|f| {
                    f.text(format!(
                        "Invoked by {}#{}",
                        msg.author.name, msg.author.discriminator
                    ))
                    .icon_url(
                        msg.author
                            .avatar_url()
                            .unwrap_or(msg.author.default_avatar_url()),
                    )
                })
            })
        })
        .await?;
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "Target hasn't been added to the database. Use the `?opt in` command for details.",
            )
            .await?;
    }

    Ok(())
}
