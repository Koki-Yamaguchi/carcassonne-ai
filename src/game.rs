use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateGame {
  pub note: String,
  pub player0_id: i32,
  pub player1_id: i32,
}

#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Game {
  pub id: i32,
  pub note: String,
  pub player0_id: i32,
  pub player1_id: i32,
  pub player0_point: i32,
  pub player1_point: i32,
  pub created_at: chrono::NaiveDateTime,
  pub ended_at: Option<chrono::NaiveDateTime>,
}
