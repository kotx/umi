use crate::utils::db::*;
use serenity::framework::standard::{macros::*, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[sub_commands(opt_in, opt_out)]
async fn opt(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if !args.is_empty() {
        return Ok(());
    }

    let pool = get_pool(ctx).await;

    if let Some(opt) = get_user_opt(&pool, msg.author.id.to_string()).await {
        msg.channel_id
            .say(
                &ctx.http,
                if opt {
                    "You are opted in for data collection."
                } else {
                    "You are not opted in for data collection."
                },
            )
            .await?;
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "You haven't been added to the database. Use the `?opt in` command for details.",
            )
            .await?;
    }

    Ok(())
}

#[command("in")]
#[description("Opts into data collection.")]
async fn opt_in(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = crate::utils::db::get_pool(ctx).await;

    let user = match get_user(&pool, msg.author.id.to_string()).await {
        Some(c) => c,
        None => {
            let id = create_user(&pool, msg.author.id.to_string()).await?;
            get_user(&pool, id).await.unwrap()
        }
    };

    set_user_opt(&pool, user.id, true).await?;

    msg.channel_id
        .say(&ctx.http, "You have opted in for data collection.")
        .await?;

    Ok(())
}

#[command("out")]
#[description(
    r#"Opts out of data collection.
NOTE: This does not remove user data from the database."#
)]
async fn opt_out(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = crate::utils::db::get_pool(ctx).await;

    if let Some(user) = get_user(&pool, msg.author.id.to_string()).await {
        set_user_opt(&pool, user.id, false).await?;
        msg.channel_id
            .say(&ctx.http, "You have opted out for data collection.")
            .await?;
    } else {
        msg.channel_id
            .say(
                &ctx.http,
                "You haven't been added to the database. Use the `?opt in` command for details.",
            )
            .await?;
    }

    Ok(())
}
