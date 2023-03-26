#[derive(Debug)]
pub enum Tile {
  StartingTile,
  Monastery,
  Invalid,
}

impl Tile {
  pub fn to_id(self) -> i32 {
    match self {
      Tile::StartingTile => 0,
      Tile::Monastery => 1,
      Tile::Invalid => -1,
    }
  }
}

pub fn to_tile(id: i32) -> Tile {
  match id {
    0 => Tile::StartingTile,
    1 => Tile::Monastery,
    _ => Tile::Invalid,
  }
}
