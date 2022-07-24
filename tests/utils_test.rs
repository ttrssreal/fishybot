// async util function tests

#[path ="../src/utils.rs"]
mod utils;

use std::sync::Once;
use dotenv::dotenv;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        dotenv().ok();
    });
}

#[cfg(test)]
mod tests {
    use crate::utils;
    
    #[tokio::test]
    async fn get_uuid() {
        crate::initialize();
        assert!(utils::get_uuid("ttrss").await.unwrap()
        == "cb9f5dbcfa994e82afc2a34a579c0062".to_string());
    }

    #[tokio::test]
    async fn get_ign() {
        crate::initialize();
        assert!(utils::get_ign("cb9f5dbcfa994e82afc2a34a579c0062").await.unwrap()
        == "ttrss".to_string());
    }

    #[tokio::test]
    async fn get_hypixel() {
        crate::initialize();
        assert!(utils::get_hypixel("cb9f5dbcfa994e82afc2a34a579c0062").await.unwrap().success
        == true);
    }

    #[tokio::test]
    async fn get_fishing() {
        crate::initialize();
        assert!(!utils::get_fishing("cb9f5dbcfa994e82afc2a34a579c0062").await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_discord() {
        crate::initialize();
        assert!(utils::get_discord("cb9f5dbcfa994e82afc2a34a579c0062").await.unwrap()
        == "ttrss#0730");
    }
}