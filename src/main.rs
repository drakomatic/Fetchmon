use std::fs;

use serde::Deserialize;

use serenity::Client;
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

mod commands;
mod discord_generic;
mod read_language;

struct Handler;

#[derive(Deserialize)]
struct Config {
    token: String,
    guild_id: u64,
}

async fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let contents: String = fs::read_to_string("config/config.json")?;
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "fetch" => commands::fetch::run(&command, &ctx).await,
                "sleep" => commands::sleep::run(&command, &ctx).await,
                _ => discord_generic::make_command_response(&command, &ctx, Some("not implemented".to_string()), None).await,
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);

        let config: Config;

        match get_config().await {
            Ok(getconfig) => config = getconfig,
            Err(why) => {
                eprintln!("An error occured: {}", why);
                return;
            }
        }

        // This is good practice right???
        let guild_id = GuildId::new(config.guild_id);

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![commands::fetch::register(), commands::sleep::register()],
            )
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config().await?;

    //this will be recieve from the config which is a json file
    let token = config.token;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MODERATION;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Could not create client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    Ok(())
}
