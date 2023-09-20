use crate::{database, error::Error};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

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
}

pub fn update_player(player_id: i32, name: String, meeple_color: i32) -> Result<Player, Error> {
    let player = match database::get_player(player_id) {
        Ok(p) => p,
        Err(e) => {
            return Err(e);
        }
    };

    database::update_player(player_id, name, meeple_color, player.rating)
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
