use crate::{models::{Link, MakeLink}};
use std::env::var;

use diesel::prelude::*;

use crate::utils::{FishyError};
use diesel::{prelude::SqliteConnection};

async fn get_conn() -> SqliteConnection {
    let location = var("DATABASE_DIR").expect("no database location");
    SqliteConnection::establish(&format!("{}/database", &location)).expect("cant connect")
}

pub async fn get_ign(_discord_tag: &str) -> Result<String, FishyError> {
    let conn = get_conn().await;
    use crate::schema::links::dsl::*;
    match links.filter(discord_tag.eq(_discord_tag)).first::<Link>(&conn) {
        Ok(result) => { return Ok(result.ign.to_string()); },
        Err(_) => { return Err(FishyError::IgnNotLinked(_discord_tag.to_string())); }
    };
}

pub async fn set_ign(_discord_tag: &str, _ign: &str) -> Result<(), FishyError> {
    let conn = get_conn().await;
    use crate::schema::links::dsl::*;
    let new_link = MakeLink { ign: _ign, discord_tag: _discord_tag };
    if let Err(e) = diesel::insert_into(links).values(&new_link).execute(&conn) {
        return Err(FishyError::DatabaseError(e));
    };
    Ok(())
}