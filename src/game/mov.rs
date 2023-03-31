use super::tile::Tile;

#[derive(Debug)]
pub struct TileMove {
  pub ord: i32,
  pub game_id: i32,
  pub player_id: i32,
  pub tile: Tile,
  pub rot: i32,
  pub pos: (i32, i32),
}

#[derive(Debug)]
pub struct MeepleMove {
  pub ord: i32,
  pub game_id: i32,
  pub player_id: i32,
  pub meeple_id: i32,
  pub tile_pos: (i32, i32),
  pub meeple_pos: i32,
}

#[derive(Debug)]
pub enum Move {
  TMove(TileMove),
  MMove(MeepleMove),
  InvalidMove,
}
