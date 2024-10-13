use crate::storage;
use crate::{database, error::Error};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

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
    pub tile_edition: String,
}

#[derive(Serialize, Queryable, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub user_id: String,
    pub meeple_color: i32,
    pub rating: Option<i32>,
    pub tile_edition: String,
    pub profile_image_url: String,
}

pub fn update_player(
    db: &DbPool,
    player_id: i32,
    name: String,
    meeple_color: i32,
    tile_edition: String,
) -> Result<Player, Error> {
    let player = database::get_player(db, player_id)?;

    database::update_player(
        db,
        player_id,
        name,
        meeple_color,
        player.rating,
        tile_edition,
        player.profile_image_url,
    )
}

pub fn get_players(
    uid: Option<String>,
    name: Option<String>,
    db: &DbPool,
) -> Result<Vec<Player>, Error> {
    let mut players = match database::get_players(db, &uid, name) {
        Ok(ps) => ps,
        Err(e) => {
            return Err(e);
        }
    };

    if uid.is_none() {
        for player in &mut players {
            player.email = "".to_string();
            player.user_id = "".to_string();
        }
    }

    Ok(players)
}

pub fn get_player(db: &DbPool, id: i32) -> Result<Player, Error> {
    let mut player = database::get_player(db, id)?;

    player.email = "".to_string();
    player.user_id = "".to_string();

    Ok(player)
}

pub async fn upload_profile_image(
    db: &DbPool,
    storage_client: &State<Client>,
    player_id: i32,
    body: ByteStream,
) -> Result<(), Error> {
    let key = format!("profile-image/{}", player_id);
    storage::put_object(storage_client, &key, body).await;

    let profile_image_url = storage::get_object_url(storage_client, &key).await;

    let player = database::get_player(db, player_id)?;

    if profile_image_url != "" {
        let _ = database::update_player(
            db,
            player_id,
            player.name,
            player.meeple_color,
            player.rating,
            player.tile_edition,
            profile_image_url.clone(),
        );

        let mut votes = vec![];
        let mut page = 0;
        loop {
            let mut vs = database::get_votes(db, None, Some(player_id), false, page, 100)?;
            if vs.len() == 0 {
                break;
            }
            votes.append(&mut vs);
            page += 1;
        }

        for vote in &votes {
            database::update_vote(
                db,
                vote.id,
                profile_image_url.clone(),
                vote.lang.clone(),
                vote.translation.clone(),
            )?;
        }
    }

    Ok(())
}
