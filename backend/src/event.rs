use crate::game::{tile::Tile, CompleteEvent};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(crate = "rocket::serde")]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub player_id: i32,
    pub tile: Tile,
    pub rot: i32,
    pub tile_pos: (i32, i32),
    pub meeple_id: i32,
    pub meeple_pos: i32,
    pub complete_events: Vec<CompleteEvent>,
}
