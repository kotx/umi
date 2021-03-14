use chrono::Utc;
use serenity::framework::standard::{help_commands, Args, CommandGroup, HelpOptions};
use serenity::framework::standard::{macros::*, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::collections::HashSet;

#[command]
#[aliases("p", "!")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong.").await?;

    Ok(())
}

#[command]
#[aliases("info", "a", "i")]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let user = ctx.http.get_current_user().await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| { e
                .author(|a| {
                    a.name(format!("About {}", &user.name))
                        .icon_url(&user.avatar_url().unwrap_or(user.default_avatar_url()))
                })
                .description("TODO")
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

    Ok(())
}

#[help("help", "h", "?")]
#[max_levenshtein_distance(2)]
#[lacking_permissions = "Strike"]
#[lacking_role = "Strike"]
#[wrong_channel = "Strike"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
