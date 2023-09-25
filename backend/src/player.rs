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

    database::update_player(player_id, name, meeple_color, player.rating)
}

pub async fn upload_profile_image(
    storage_client: &State<Client>,
    player_id: i32,
    body: ByteStream,
) {
    let key = format!("profile-image/{}", player_id);
    storage::put_object(storage_client, &key, body).await
}

pub async fn get_player_by_uid(
    storage_client: &State<Client>,
    uid: String,
) -> Result<Player, Error> {
    let mut player = database::get_player_by_uid(uid)?;

    let key = format!("profile-image/{}", player.id);
    player.profile_image_url = storage::get_object_url(storage_client, &key).await;

    Ok(player)
}

pub async fn get_players(storage_client: &State<Client>) -> Result<Vec<Player>, Error> {
    let mut players = match database::get_players() {
        Ok(ps) => ps,
        Err(e) => {
            return Err(e);
        }
    };

    for player in &mut players {
        player.email = "".to_string();
        player.user_id = "".to_string();

        let key = format!("profile-image/{}", player.id);
        player.profile_image_url = storage::get_object_url(storage_client, &key).await;
    }

    Ok(players)
}

pub async fn get_player(storage_client: &State<Client>, id: i32) -> Result<Player, Error> {
    let mut player = database::get_player(id)?;

    player.email = "".to_string();
    player.user_id = "".to_string();

    let key = format!("profile-image/{}", player.id);
    player.profile_image_url = storage::get_object_url(storage_client, &key).await;

    Ok(player)
}
