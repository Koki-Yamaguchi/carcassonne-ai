use super::tile::Tile;
use self::Move::{TMove, MMove, InvalidMove};

#[derive(Debug, Clone)]
pub struct TileMove {
  pub ord: i32,
  pub game_id: i32,
  pub player_id: i32,
  pub tile: Tile,
  pub rot: i32,
  pub pos: (i32, i32),
}

#[derive(Debug, Clone)]
pub struct MeepleMove {
  pub ord: i32,
  pub game_id: i32,
  pub player_id: i32,
  pub meeple_id: i32,
  pub tile_pos: (i32, i32),
  pub meeple_pos: i32,
}

#[derive(Debug, Clone)]
pub enum Move {
  TMove(TileMove),
  MMove(MeepleMove),
  InvalidMove,
}

impl Move {
  pub fn ord(&self) -> i32 {
    match self {
      MMove(m) => { m.ord },
      TMove(m) => { m.ord },
      InvalidMove => { 0 }
    }
  }
}
