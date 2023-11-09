pub mod custom_commands;
pub mod guildspecific;
pub mod components;
pub mod commands;
pub mod database;
pub mod events;
pub mod structs;

use std::sync::Arc;

use events::{interactions::handle_interaction_create, messages::handle_message_update};
use tokio::sync::Mutex;
use dotenv_codegen::dotenv;
use sentry;
use serenity::http::Http;
use serenity::model::prelude::*;
use mongodb::{ Client, options::ClientOptions };


use once_cell::sync::Lazy;

use crate::events::messages::handle_message_create;

pub static MONGO: Lazy<Mutex<Option<Client>>> = Lazy::new(|| Mutex::new(None));
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, SessionData, Error>;
pub struct SessionData {
    pub cache: database::cache::ProfileCache
} // User data, accessible through all command invocations

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file.");
    let _guard = sentry::init(dotenv!("SENTRY_INIT_URI"));

    let mut client_options = ClientOptions::parse(dotenv!("MONGO_DB_URI")).await.unwrap();
    client_options.app_name = Some("Ether".to_string());
    let client = Client::with_options(client_options).unwrap();
    *MONGO.lock().await = Some(client);

    let token = dotenv!("DISCORD_TOKEN");
    let http = Http::new(&token);
    http.set_application_id(1027323480456298506 as u64);

    let (_owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = std::collections::HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };


    // ! POISE CONFIGURATION
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::meta::stats::stats(), commands::profile::profile(),
            commands::administration::warnings::warn(),
            commands::administration::warnings::warnings()
        ],
        prefix_options: poise::PrefixFrameworkOptions::default(),

        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}..", ctx.command().qualified_name);
            })
        },

        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },

        command_check: Some(|_ctx| {
            Box::pin(async move {
                Ok(true)
            })
        }),

        event_handler: |ctx, event, _framework, data| {
            Box::pin(async move {
                return match event {
                    poise::Event::Message { new_message } => Ok(handle_message_create(ctx, new_message, data).await?),
                    poise::Event::InteractionCreate { interaction } => Ok(handle_interaction_create(ctx, interaction, data).await?),
                    poise::Event::MessageUpdate { old_if_available, new, event } => Ok(handle_message_update(ctx, old_if_available, Arc::new(new), event, data).await?),
                    _ => Ok(())
                };
            })
        },

        ..Default::default()
    };

    poise::Framework::builder()
        .token(
            env!("DISCORD_TOKEN"))
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(SessionData {
                    cache: database::cache::populated_cache().await?
                })
            })
        })
        .options(options)
        .intents(
            GatewayIntents::GUILDS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::GUILD_MESSAGE_REACTIONS
            | GatewayIntents::MESSAGE_CONTENT
        ).run().await.unwrap();

}