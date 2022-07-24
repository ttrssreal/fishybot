use crate::database;
use crate::utils;
use crate::imageutils;

use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use imageutils::fill_template;
use serenity::model::channel::AttachmentType;
use std::path::Path;
use std::env::var;

#[command]
pub async fn s(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {

    let usage = "Incorrect usage. Try just \"f!s\" or \"f!s [username]\"";

    if _args.len() > 1 {
        utils::send_message_txt(&msg.channel_id, ctx, usage).await;
        return Ok(());
    }

    let author_tag = &msg.author.tag();
    let target_ign = _args.single::<String>().unwrap_or("".to_string());
    let uuid = if _args.len() == 0 { match database::d_get_uuid( author_tag).await {
        Ok(uuid) => uuid,
        Err(err) => {
            match err {
                utils::FishyError::User(_) => utils::send_message_txt(&msg.channel_id, ctx, &format!("{}", err)).await,
                _ => { println!("{:?}", err); }};
            return Ok(());
        }
    } } else {
        match utils::get_uuid(&target_ign).await {
            Ok(uuid) => uuid,
            Err(err) => {
                match err {
                    utils::FishyError::User(_) => utils::send_message_txt(&msg.channel_id, ctx, &format!("{}", err)).await,
                    _ => { println!("{:?}", err); }};
                return Ok(());
            }
        }
    };

    let ign = utils::get_ign(&uuid).await.expect("couldn't get ign from uuid");

    let is_special: bool = var("SPECIAL").expect("no special list").split(" ").collect::<Vec<&str>>().iter().any(|x| x.to_string() == uuid); // TODO: fix with uuid
    let fish_stats = match utils::get_fishing(&uuid).await {
        Ok(fish_stats) => fish_stats,
        Err(err) => {
            match err {
                utils::FishyError::User(_) => utils::send_message_txt(&msg.channel_id, ctx, &format!("{}", err)).await,
                _ => { println!("{:?}", err); }
            };
            return Ok(());
        }
    };
    fill_template(&ign, is_special, fish_stats[0] as f64, fish_stats[1] as f64, fish_stats[2] as f64).await;
    msg.channel_id.send_message(&ctx.http, |m| m.add_file(AttachmentType::Path(Path::new("generated.png")))).await?;
    Ok(())
}