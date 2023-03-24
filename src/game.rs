use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Deserialize, Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Game {
  pub id: i32,
  pub note: String,
}
