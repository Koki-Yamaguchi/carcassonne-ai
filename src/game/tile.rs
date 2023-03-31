#[derive(Debug, Copy, Clone)]
pub enum Tile {
  StartingTile,
  CityCapWithCrossroad,
  Monastery,
  TriagnleWithRoad,
  TriagnleWithRoadWithCOA,
  Invalid,
}

impl Tile {
  pub fn to_id(self) -> i32 {
    match self {
      Tile::StartingTile => 0,
      Tile::Monastery => 1,
      Tile::CityCapWithCrossroad => 2,
      Tile::TriagnleWithRoad => 3,
      Tile::TriagnleWithRoadWithCOA => 4,
      Tile::Invalid => -1,
    }
  }
}

pub fn to_tile(id: i32) -> Tile {
  match id {
    0 => Tile::StartingTile,
    1 => Tile::Monastery,
    2 => Tile::CityCapWithCrossroad,
    3 => Tile::TriagnleWithRoad,
    4 => Tile::TriagnleWithRoadWithCOA,
    _ => Tile::Invalid,
  }
}
