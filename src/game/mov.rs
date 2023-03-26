pub enum Tile {
  StartingTile,
}

pub struct TileMove {
  pub ord: i32,
  pub game_id: i32,
  pub player_id: i32,
  pub tile: Tile,
  pub rot: i32,
  pub pos: (i32, i32),
}

pub struct MeepleMove {
  pub ord: i32,
  pub game_id: i32,
  pub player_id: i32,
  pub meeple_id: i32,
  pub pos: i32,
}

pub struct SkipMove {
  pub ord: i32,
  pub game_id: i32,
  pub player_id: i32,
}

pub enum Move {
  TMove(TileMove),
  MMove(MeepleMove),
  SMove(SkipMove),
}
