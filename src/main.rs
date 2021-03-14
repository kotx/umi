#[forbid(unsafe_code)]
#[deny(bad_style)]

mod utils;
mod commands;
mod data;
mod events;

use data::PgPoolContainer;
use events::{connection, message};
use serenity::{Client, client::bridge::gateway::ShardManager, framework::StandardFramework, http::Http, prelude::{Mutex, TypeMapKey}};
use std::{
    collections::HashSet,
    env,
    sync::Arc,
};
use tracing::error;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let pool = data::get_pool().await.unwrap();
    // let mut migrator = sqlx::migrate!();

    // Temporary workaround: remove reverse migrations (launchbadge/sqlx#863).
    // migrator.migrations.to_mut().retain(|migration| !migration.description.ends_with(".down"));
    // migrator.run(&pool).await.unwrap();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .prefix("u~")
                .with_whitespace(true)
                .no_dm_prefix(true)
        })
        .help(&commands::meta::HELP)
        .group(&commands::GENERAL_GROUP)
        .group(&commands::CONFIG_GROUP)
        .group(&commands::STATS_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(connection::Handler)
        .event_handler(message::Handler)
        .await
        .expect("Failed to create client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<PgPoolContainer>(pool.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register Control+C handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
