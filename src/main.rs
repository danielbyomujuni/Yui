mod commands;


use std::env;
use dotenv::dotenv;

use poise::{async_trait, Command, serenity_prelude as serenity};
use serenity::prelude::{Context, EventHandler, GatewayIntents};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use crate::commands::{Data, Error};
use crate::commands::hello::hello;


#[cfg(test)]
mod test;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = serenity::GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;


    let command_list:Vec<Command<Data, Error>> = vec![
        hello()
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: command_list,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await;
    client.unwrap().start().await.unwrap();
}