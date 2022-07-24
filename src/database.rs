use crate::{models::{Link, MakeLink}};
use std::env::var;

use diesel::prelude::*;

use crate::utils::{FishyError, UserError};
use diesel::{prelude::SqliteConnection};

async fn get_conn() -> SqliteConnection {
    let location = var("DATABASE_DIR").expect("no database location");
    SqliteConnection::establish(&format!("{}/database", &location)).expect("cant connect")
}

pub async fn d_get_uuid(_discord_tag: &str) -> Result<String, FishyError> {
    let conn = get_conn().await;
    use crate::schema::links::dsl::*;
    match links.filter(discord_tag.eq(_discord_tag)).first::<Link>(&conn) {
        Ok(result) => { return Ok(result.uuid.to_string()); },
        Err(_) => { return Err(FishyError::User(UserError::UUIDNotLinked(_discord_tag.to_string()))); }
    };
}

pub async fn set_uuid(_discord_tag: &str, _uuid: &str) -> Result<(), FishyError> {
    let conn = get_conn().await;
    use crate::schema::links::dsl::*;
    let new_link = MakeLink { uuid: _uuid, discord_tag: _discord_tag };
    if let Err(e) = diesel::insert_into(links).values(&new_link).execute(&conn) {
        return Err(FishyError::DatabaseError(e));
    };
    Ok(())
}