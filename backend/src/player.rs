use crate::storage;
use crate::{database, error::Error};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreatePlayer {
    pub name: String,
    pub email: String,
    pub user_id: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdatePlayer {
    pub name: String,
    pub meeple_color: i32,
}

#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub user_id: String,
    pub meeple_color: i32,
    pub rating: Option<i32>,
    pub profile_image_url: String,
}

pub fn update_player(player_id: i32, name: String, meeple_color: i32) -> Result<Player, Error> {
    let player = database::get_player(player_id)?;

    database::update_player(
        player_id,
        name,
        meeple_color,
        player.rating,
        player.profile_image_url,
    )
}

pub fn get_player_by_uid(uid: String) -> Result<Player, Error> {
    database::get_player_by_uid(uid)
}

pub fn get_players() -> Result<Vec<Player>, Error> {
    let mut players = match database::get_players() {
        Ok(ps) => ps,
        Err(e) => {
            return Err(e);
        }
    };

    for player in &mut players {
        player.email = "".to_string();
        player.user_id = "".to_string();
    }

    Ok(players)
}

pub fn get_player(id: i32) -> Result<Player, Error> {
    let mut player = database::get_player(id)?;

    player.email = "".to_string();
    player.user_id = "".to_string();

    Ok(player)
}

pub async fn upload_profile_image(
    storage_client: &State<Client>,
    player_id: i32,
    body: ByteStream,
) -> Result<(), Error> {
    let key = format!("profile-image/{}", player_id);
    storage::put_object(storage_client, &key, body).await;

    let profile_image_url = storage::get_object_url(storage_client, &key).await;

    let player = database::get_player(player_id)?;

    if profile_image_url != "" {
        let _ = database::update_player(
            player_id,
            player.name,
            player.meeple_color,
            player.rating,
            profile_image_url.clone(),
        );

        let votes = database::get_votes(None, Some(player_id), false)?;
        for vote in &votes {
            database::update_vote(vote.id, profile_image_url.clone())?;
        }
    }

    Ok(())
}

use rocket::tokio;
#[tokio::test]
#[allow(dead_code)]
async fn update_profile_image_test() {
    use aws_config::meta::region::RegionProviderChain;

    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let storage_client = Client::new(&config);

    let players = database::get_all_players().unwrap();

    for player in &players {
        let key = format!("profile-image/{}", player.id);
        let profile_image_url = storage::get_object_url(State::from(&storage_client), &key).await;

        if profile_image_url != "" {
            let _ = database::update_player(
                player.id,
                player.name.clone(),
                player.meeple_color,
                player.rating,
                profile_image_url.clone(),
            );

            let votes = database::get_votes(None, Some(player.id), false).unwrap();
            for vote in &votes {
                database::update_vote(vote.id, profile_image_url.clone()).unwrap();
            }
        }
    }
}
