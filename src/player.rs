use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreatePlayer {
  pub name: String,
}

#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Player {
  pub id: i32,
  pub name: String,
}