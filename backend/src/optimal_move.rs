use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct OptimalMove {
    pub id: i32,
    pub game_id: i32,
    pub last_n: i32,
    pub tile_move_id: i32,
    pub meeple_move_id: i32,
}
