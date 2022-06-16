#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
mod utils;
mod schema;
mod database;
mod models;
mod instructions;
mod imageutils;

use dotenv::dotenv;
use std::env::var;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::framework::standard::{
    StandardFramework,
    macros::group
};

use crate::instructions::{s::*, l::*};

#[group]
#[commands(s,l)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!("{} logged in...", data_about_bot.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new().configure(|c| c.prefixes(vec!["f!", "F!"]) ).group(&GENERAL_GROUP);

    let token = var("DISCORD_TOKEN").expect("discord token");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents).event_handler(Handler).framework(framework).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}