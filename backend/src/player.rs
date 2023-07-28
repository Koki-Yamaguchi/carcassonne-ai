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
}
