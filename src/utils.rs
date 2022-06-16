use serenity::model::id::ChannelId;
use serenity::client::Context;
use serenity::prelude::*;
use serenity::model;
use std::env::var;
use serde::{Serialize, Deserialize};

macro_rules! unwrap_or_return_r {
    ( $e:expr, $r:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return $r
        }
    }
}

// Errors
// player dosent exist (uuid request returns 204 No Content)
// player hasnt played hypixel (player is null or success false)
// player hasnt linked discord
#[derive(Debug)]
pub enum UserError {
    PlayerDosentExist(String),
    PlayerNoHypixel(String),
    PlayerNoLinkedDiscord(String),
    Nofished(),
    IgnNotLinked(String)
}

#[derive(Debug)]
pub enum FishyError {
    User(UserError),
    NetworkError(String),
    JsonDecodeError(String),
    DatabaseError(diesel::result::Error),
}

impl std::error::Error for FishyError {}

impl std::fmt::Display for FishyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FishyError::User(user_error) => {
                match user_error {
                    UserError::PlayerDosentExist(player_name) => write!(f, "The player {player_name} dosen't exists."),
                    UserError::PlayerNoHypixel(player_name) => write!(f, "{player_name} hasn't logged into hypixel."),
                    UserError::PlayerNoLinkedDiscord(player_name) => write!(f, "I can't find a discord profile on hypixel, linked to {player_name}."),
                    UserError::Nofished() => write!(f, "Hasn't fished before"),
                    UserError::IgnNotLinked(_) => write!(f, "You need to link your account first. f!l [username]."),
                }
            },
            FishyError::DatabaseError(err) => write!(f, "Database Err: {err}"),
            FishyError::NetworkError(url) => write!(f, "NetworkError URL: {url}"),
            FishyError::JsonDecodeError(location) => write!(f, "JsonDecodeError Loc: {location}"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MojangProfile {
    name: String,
    id: String
}

#[derive(Serialize, Deserialize)]
pub struct HypixelProfile {
    success: bool,
    player: serde_json::Value
}

pub async fn send_message_txt(channel_id: &ChannelId, ctx: &Context, message: &str) {
    match channel_id.send_message(&ctx.http, |m| m.content(message)).await {
        Ok(_) => {},
        Err(err) => {
            if let SerenityError::Model(model_err) = err {
                if let model::error::Error::MessageTooLong(num_too_long) = model_err {
                    println!("The static method you tried to send was {num_too_long} chars long, needs to be 2000 or less :(");
                }
            } else if let SerenityError::Http(_) = err {
                println!("The bot doesn't have the required permissions :(");
            }
        }
    };
}

pub async fn get_uuid(player_name: &str) -> Result<String, FishyError> {
    let mojang_url_loc = var("MOJANG_PROFILE_API_ENDPOINT").expect("mojang enpoint");
    let url = format!("{}/{}", mojang_url_loc, player_name);
    let mojang_resp = unwrap_or_return_r!(reqwest::Client::new().get(&url).send().await, Err(FishyError::NetworkError(url)));

    match mojang_resp.status() {
        reqwest::StatusCode::NO_CONTENT | reqwest::StatusCode::BAD_REQUEST => { 
            return Err(FishyError::User(UserError::PlayerDosentExist(player_name.to_string())));
        }, _ => {}
    };

    let mojang_pf = unwrap_or_return_r!(mojang_resp.json::<MojangProfile>().await, Err(FishyError::JsonDecodeError("Mojang".to_string())));

    Ok(mojang_pf.id)
}

async fn get_hypixel(player_name: &str) -> Result<HypixelProfile, FishyError> {
    let hypixel_url_loc = var("HYPIXEL_PROFILE_API_ENDPOINT").expect("hypixel enpoint");
    let hypixel_apik = var("HYPIXEL_API_KEY").expect("hypixel api key");
    
    let uuid: String = get_uuid(player_name).await?;

    let url = format!("{}?key={}&uuid={}", hypixel_url_loc, hypixel_apik, uuid);

    let hypixel_resp = unwrap_or_return_r!(reqwest::Client::new().get(&url).send().await, Err(FishyError::NetworkError(url)));
    let hypixel_pf = unwrap_or_return_r!(hypixel_resp.json::<HypixelProfile>().await, Err(FishyError::JsonDecodeError("Hypixel".to_string())));

    if !(hypixel_pf.success) || hypixel_pf.player == serde_json::Value::Null {
        return Err(FishyError::User(UserError::PlayerNoHypixel(player_name.to_string())));
    }

    Ok(hypixel_pf)
}

pub async fn get_fishing(player_name: &str) -> Result<Vec<u64>, FishyError> {
    let hypixel_pf = get_hypixel(player_name).await?;

    if let Some(achievements) = hypixel_pf.player.get("achievements") {
        if let Some(general_master_lure) = achievements.get("general_master_lure") {
        if let Some(general_luckiest_of_the_sea) = achievements.get("general_luckiest_of_the_sea") {
        if let Some(general_trashiest_diver) = achievements.get("general_trashiest_diver") {
            // return Ok(discord.as_str().expect("discord link value wasn't a string").to_string());
            return Ok(vec![general_master_lure.as_u64().unwrap(),
                           general_luckiest_of_the_sea.as_u64().unwrap(),
                           general_trashiest_diver.as_u64().unwrap()]);
        }}}
    }
    
    Err(FishyError::User(UserError::Nofished()))
}

pub async fn get_discord(player_name: &str) -> Result<String, FishyError> {
    let hypixel_pf = get_hypixel(player_name).await?;

    if let Some(social_media) = hypixel_pf.player.get("socialMedia") {
        if let Some(links) = social_media.get("links") {
            if let Some(discord) = links.get("DISCORD") {
                return Ok(discord.as_str().expect("discord link value wasn't a string").to_string());
            }
        }
    }

    Err(FishyError::User(UserError::PlayerNoLinkedDiscord(player_name.to_string())))
}