
mod commands;


use std::env;
use dotenv::dotenv;

use poise::{async_trait, Command, serenity_prelude as serenity};
use serenity::prelude::{Context, EventHandler, GatewayIntents};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use crate::commands::{Data, Error};
use crate::commands::hello::hello;
use crate::llmBridge::set_prompt;


#[cfg(test)]
mod test;
mod llmBridge;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        unsafe {
            if msg.mentions_me(&ctx.http).await.unwrap() {
                let mut prompt = msg.content;
                let typing = msg.channel_id.start_typing(&ctx.http);

                prompt = prompt.replace("<@459823496567193600>", "");
                println!("Reqeust: {}", prompt);

                let res = set_prompt(&*prompt).await;

                println!("Response: {}", res);
                typing.stop();
                if let Err(why) = msg.channel_id.say(&ctx.http, res).await {
                    println!("Error sending message: {why:?}");
                }
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