use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::database::*;

use crate::utils::{send_message_txt, get_discord, FishyError};

#[command]
pub async fn l(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let usage = "Incorrect usage. Try f!l [username]";

    let argc = args.len();

    if argc > 1 || argc == 0 {
        send_message_txt(&msg.channel_id, ctx, usage).await;
        return Ok(());
    }

    let player_name = match args.single::<String>() {
        Ok(player_name) => player_name,
        Err(_) => { 
            send_message_txt(&msg.channel_id, ctx, usage).await;
            panic!("Parse error in link args");
        }
    };

    let uuid = match crate::utils::get_uuid(&player_name).await {
        Ok(uuid) => uuid,
        Err(err) => {
            match err {
                FishyError::User(_) => send_message_txt(&msg.channel_id, ctx, &format!("{}", err)).await,
                _ => { println!("{:?}", err); }
            };
            return Ok(());
        }
    };
    
    let discord_tag = match get_discord(&uuid).await {
        Ok(discord_tag) => discord_tag,
        Err(err) => {
            match err {
                FishyError::User(_) => send_message_txt(&msg.channel_id, ctx, &format!("{}", err)).await,
                _ => { println!("{:?}", err); }
            };
            return Ok(());
        }
    };

    let author_tag = msg.author.tag();

    if discord_tag != author_tag {
        send_message_txt(&msg.channel_id, ctx, &format!("Account {author_tag} does not match the discord account linked to the user specified.")).await;
        return Ok(());
    }

    match d_get_uuid(&author_tag).await {
        Ok(d_uuid) => {
            let ign = crate::utils::get_ign(&d_uuid).await.unwrap();
            send_message_txt(&msg.channel_id, ctx, &format!("You've already linked to \"{ign}\".")).await; 
            return Ok(());
        },
        Err(_) => {
            set_uuid(&author_tag, &uuid).await.unwrap();
            send_message_txt(&msg.channel_id, ctx, &format!("Successfully linked {author_tag} to \"{player_name}\"!")).await; 
        }
    };
    Ok(())
}