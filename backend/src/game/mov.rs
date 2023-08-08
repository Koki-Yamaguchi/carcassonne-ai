use self::Move::{DMove, InvalidMove, MMove, TMove};
use super::tile::Tile;
use rocket::serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TileMove {
    pub ord: i32,
    pub game_id: i32,
    pub player_id: i32,
    pub tile: Tile,
    pub rot: i32,
    pub pos: (i32, i32),
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MeepleMove {
    pub ord: i32,
    pub game_id: i32,
    pub player_id: i32,
    pub meeple_id: i32,
    pub tile_pos: (i32, i32),
    pub meeple_pos: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DiscardMove {
    pub ord: i32,
    pub game_id: i32,
    pub player_id: i32,
    pub tile: Tile,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Move {
    TMove(TileMove),
    MMove(MeepleMove),
    DMove(DiscardMove),
    InvalidMove,
}

impl Move {
    pub fn ord(&self) -> i32 {
        match self {
            MMove(m) => m.ord,
            TMove(m) => m.ord,
            DMove(m) => m.ord,
            InvalidMove => 0,
        }
    }
}
