pub mod custom_commands;
pub mod components;
pub mod commands;
pub mod database;

use tokio::sync::Mutex;
use dotenv_codegen::dotenv;
use sentry;
use serenity::http::Http;
use serenity::model::prelude::*;
use mongodb::{ Client, options::ClientOptions };

use once_cell::sync::Lazy;

pub static MONGO: Lazy<Mutex<Option<Client>>> = Lazy::new(|| Mutex::new(None));
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {} // User data, accessible through all command invocations

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
        commands: vec![commands::meta::stats::stats(), commands::profile::profile()],
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

        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!("Got an event in event handler: {:?}", event.name());
                Ok(())
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
                Ok(Data {

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