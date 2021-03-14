use crate::utils::db::*;
use chrono::Utc;
use markov::Chain;
use petgraph::{Graph, dot::{Config, Dot}};
use serenity::framework::standard::{macros::*, ArgError, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::info;

fn get_chain(messages: &Vec<MessageContent>) -> Chain<String> {
    let mut chain: Chain<String> = Chain::new();

    for msg in messages {
        chain.feed_str(msg.content.as_str());
    }

    // chain.feed_str(messages.into_iter().map(|x| x.content.to_string()).collect::<Vec<String>>().join(" ").as_str());
    chain
}

fn get_markov(messages: &Vec<MessageContent>, token: Option<String>) -> String {
    let chain = get_chain(messages);

    match token {
        None => chain.generate_str(),
        Some(token) => chain.generate_str_from_token(token.as_str())
    }
}

fn get_graph(messages: &Vec<MessageContent>) -> Option<Graph<Vec<Option<String>>, f64>> {
    let chain = get_chain(messages);
    Some(chain.graph())
}

#[command]
#[aliases("m", "..")]
#[description("Processes all of your tracked messages through a markov chain and generates text.")]
async fn markov(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let pool = crate::utils::db::get_pool(ctx).await;

    let target = match args.single::<UserId>() {
        Ok(user) => user,
        Err(ArgError::Eos) => msg.author.id,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Could not get the target user. Check your arguments and try again",
                )
                .await?;
            return Ok(());
        }
    };

    let target_user = target.to_user(&ctx).await?;

    if let Some(user) = get_user(&pool, target.to_string()).await {
        let messages = get_messages_content(&pool, user.id).await?;

        let markov = get_markov(&messages, None);

        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.name(format!("{} once said...", target_user.name,))
                            .icon_url(
                                &target_user
                                    .avatar_url()
                                    .unwrap_or(msg.author.default_avatar_url()),
                            )
                    })
                    .description(markov)
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

#[command]
#[aliases("ms", "~")]
#[description("Processes all tracked messages through a markov chain and generates text starting with the given word.")]
async fn search(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let pool = crate::utils::db::get_pool(ctx).await;

    let target = match args.single::<UserId>() {
        Ok(user) => user,
        Err(ArgError::Eos) => msg.author.id,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Could not get the target user. Check your arguments and try again",
                )
                .await?;
            return Ok(());
        }
    };

    let target_user = target.to_user(&ctx).await?;

    let token = match args.single::<String>() {
        Ok(token) => token,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Please enter a valid word to start with")
                .await?;
            return Ok(());
        }
    };

    if let Some(user) = get_user(&pool, target.to_string()).await {
        let messages = get_messages_content(&pool, user.id).await?;

        let markov = get_markov(&messages, Some(token));

        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    if !markov.is_empty() {
                        e.author(|a| {
                            a.name(format!("{} once said...", target_user.name,))
                                .icon_url(
                                    &target_user
                                        .avatar_url()
                                        .unwrap_or(msg.author.default_avatar_url()),
                                )
                        })
                        .description(markov);
                    } else {
                        e.description("Sorry, I couldn't find anything...");
                    }

                    e.timestamp(Utc::now().to_rfc3339()).footer(|f| {
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

#[command]
#[aliases("g", "+")]
#[description("Creates a graph from the markov chain.")]
async fn graph(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let pool = crate::utils::db::get_pool(ctx).await;

    let target = match args.single::<UserId>() {
        Ok(user) => user,
        Err(ArgError::Eos) => msg.author.id,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "Could not get the target user. Check your arguments and try again",
                )
                .await?;
            return Ok(());
        }
    };

    let target_user = target.to_user(&ctx).await?;

    if let Some(user) = get_user(&pool, target.to_string()).await {
        let messages = get_messages_content(&pool, user.id).await?;

        let graph = get_graph(&messages).unwrap();

        {
            let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
            info!("```dot\n{:?}```", dot);
        }

        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.name(format!("Graph of {}'s markov chain", target_user.name))
                            .icon_url(
                                &msg.author
                                    .avatar_url()
                                    .unwrap_or(msg.author.default_avatar_url()),
                            )
                    })
                    // TODO: render dot graph image
                    // .description(format!("```dot\n{:?}```", dot))
                    // .image("")
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
