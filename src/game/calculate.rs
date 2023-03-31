use super::mov::{Move, TileMove, MeepleMove };
use super::tile::{Tile};
use super::mergeable_feature::MergeableFeature;
use self::Side::*;
use self::Square::*;
use self::Feature::*;

#[derive(Debug)]
pub struct CompleteEvent {
  pub feature: Feature,
  pub meeple_ids: Vec<i32>,
  pub point: i32,
}

pub struct Status {
  pub meepleable_positions: Vec<i32>,
  pub complete_events: Vec<CompleteEvent>,
  pub player0_point: i32,
  pub player1_point: i32,
}

#[allow(dead_code)]
pub struct Error {
  msg: String
}

#[derive(Copy, Clone)]
struct TileItem {
  id: i32,
  tile: Tile,
  rot: i32,
  feature_starting_id: i32,
}

#[derive(Clone)]
enum Square {
  Tile(TileItem),
  Empty
}

#[derive(PartialEq, Copy, Clone)]
enum Side {
  Field,
  Road,
  City,
  None,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Feature {
  FieldFeature,
  RoadFeature,
  CityFeature,
  MonasteryFeature,
}

impl Feature {
  pub fn to_string(self) -> String {
    match self {
      FieldFeature => { "field".to_string() }
      RoadFeature => { "road".to_string() }
      CityFeature => { "city".to_string() }
      MonasteryFeature => { "monastery".to_string() }
    }
  }
}

#[derive(Clone, Debug)]
struct DistinctFeature {
  id: i32,
  feature: Feature,
}

impl TileItem {
  fn sides(self) -> [Side; 4] {
    match self.tile {
      Tile::StartingTile => [Road, City, Road, Field],
      Tile::Monastery => [Field, Field, Field, Field],
      Tile::MonasteryWithRoad => [Field, Field, Field, Road],
      Tile::CityCapWithCrossroad => [Road, City, Road, Road],
      Tile::TriangleWithRoad => [Road, City, City, Road],
      Tile::TriangleWithRoadWithCOA => [Road, City, City, Road],
      Tile::Straight => [Field, Road, Field, Road],
      Tile::CityCap => [Field, City, Field, Field],
      Tile::Separator => [Field, City, City, Field],
      Tile::TripleRoad => [Road, Field, Road, Road],
      Tile::Curve => [Field, Field, Road, Road],
      Tile::QuadrupleRoad => [Road, Road, Road, Road],
      Tile::Connector => [City, Field, City, Field],
      Tile::ConnectorWithCOA => [City, Field, City, Field],
      Tile::Left => [Field, City, Road, Road],
      Tile::Right => [Road, City, Field, Road],
      Tile::TripleCity => [City, City, City, Field],
      Tile::TripleCityWithCOA => [City, City, City, Field],
      Tile::VerticalSeparator => [Field, City, Field, City],
      Tile::TripleCityWithRoad => [City, City, City, Road],
      Tile::TripleCityWithRoadWithCOA => [City, City, City, Road],
      Tile::Triangle => [Field, City, City, Field],
      Tile::TriangleWithCOA => [Field, City, City, Field],
      Tile::QuadrupleCityWithCOA => [City, City, City, City],
      Tile::Invalid => [Field, Field, Field, Field],
    }
  }
  fn right(self) -> Side {
    self.sides()[((self.rot + 0) % 4) as usize]
  }
  fn top(self) -> Side {
    self.sides()[((self.rot + 1) % 4) as usize]
  }
  fn left(self) -> Side {
    self.sides()[((self.rot + 2) % 4) as usize]
  }
  fn bottom(self) -> Side {
    self.sides()[((self.rot + 3) % 4) as usize]
  }
  fn right_features(self) -> Vec<DistinctFeature> {
    self.side_features()[((self.rot + 0) % 4) as usize].clone()
  }
  fn top_features(self) -> Vec<DistinctFeature> {
    self.side_features()[((self.rot + 1) % 4) as usize].clone()
  }
  fn left_features(self) -> Vec<DistinctFeature> {
    self.side_features()[((self.rot + 2) % 4) as usize].clone()
  }
  fn bottom_features(self) -> Vec<DistinctFeature> {
    self.side_features()[((self.rot + 3) % 4) as usize].clone()
  }
  // side_features defined so that they are not influenced by the effect of turn
  fn side_features(self) -> Vec<Vec<DistinctFeature>> {
    match self.tile {
      Tile::StartingTile => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
        ], // bottom
      ],
      Tile::Monastery => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::MonasteryWithRoad => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::CityCapWithCrossroad => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 3, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 6, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 4, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 6, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 5, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 4, feature: FieldFeature },
        ], // bottom
      ],
      Tile::TriangleWithRoad => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::TriangleWithRoadWithCOA => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::Straight => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        ], // bottom
      ],
      Tile::CityCap => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::Separator => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
        ], // bottom
      ],
      Tile::TripleRoad => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 5, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 5, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 4, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
        ], // bottom
      ],
      Tile::Curve => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
        ], // bottom
      ],
      Tile::QuadrupleRoad => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 4, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 7, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 5, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 3, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 7, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 6, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 5, feature: FieldFeature },
        ], // bottom
      ],
      Tile::Connector => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: CityFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
        ], // bottom
      ],
      Tile::ConnectorWithCOA => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: CityFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
        ], // bottom
      ],
      Tile::Left => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
        ], // bottom
      ],
      Tile::Right => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::TripleCity => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::TripleCityWithCOA => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::VerticalSeparator => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 2, feature: CityFeature },
        ], // bottom
      ],
      Tile::TripleCityWithRoad => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::TripleCityWithRoadWithCOA => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
          DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::Triangle => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::TriangleWithCOA => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        ], // bottom
      ],
      Tile::QuadrupleCityWithCOA => vec![
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // right
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // top
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // left
        vec![
          DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        ], // bottom
      ],
      Tile::Invalid => vec![vec![]],
    }
  }
  fn features(self) -> Vec<DistinctFeature> {
    match self.tile {
      Tile::StartingTile => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
      ],
      Tile::Monastery => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: MonasteryFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
      ],
      Tile::MonasteryWithRoad => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: MonasteryFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
      ],
      Tile::CityCapWithCrossroad => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 4, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 5, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 6, feature: FieldFeature },
      ],
      Tile::TriangleWithRoad => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
      ],
      Tile::TriangleWithRoadWithCOA => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
      ],
      Tile::Straight => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
      ],
      Tile::CityCap => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
      ],
      Tile::Separator => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
      ],
      Tile::TripleRoad => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 4, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 5, feature: FieldFeature },
      ],
      Tile::Curve => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
      ],
      Tile::QuadrupleRoad => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 4, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 5, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 6, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 7, feature: FieldFeature },
      ],
      Tile::Connector => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
      ],
      Tile::ConnectorWithCOA => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: FieldFeature },
      ],
      Tile::Left => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
      ],
      Tile::Right => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
      ],
      Tile::TripleCity => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
      ],
      Tile::TripleCityWithCOA => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
      ],
      Tile::VerticalSeparator => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: CityFeature },
      ],
      Tile::TripleCityWithRoad => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
      ],
      Tile::TripleCityWithRoadWithCOA => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
      ],
      Tile::Triangle => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
      ],
      Tile::TriangleWithCOA => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
      ],
      Tile::QuadrupleCityWithCOA => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
      ],
      Tile::Invalid => vec![],
    }
  }
}

fn create_mergeable_features(mf: &mut MergeableFeature, t: &TileItem) {
  match t.tile {
    Tile::StartingTile => {
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 3, false);
    },
    Tile::Monastery => {
      mf.new_feature(t.id, 9, false);
      mf.new_feature(t.id, 4, false);
    },
    Tile::MonasteryWithRoad => {
      mf.new_feature(t.id, 9, false);
      mf.new_feature(t.id, 5, false);
      mf.new_feature(t.id, 1, false);
    },
    Tile::CityCapWithCrossroad => {
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
    },
    Tile::TriangleWithRoad => {
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 2, false);
    },
    Tile::TriangleWithRoadWithCOA => {
      mf.new_feature(t.id, 2, true);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 2, false);
    },
    Tile::Straight => {
      mf.new_feature(t.id, 3, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 3, false);
    }
    Tile::CityCap => {
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 3, false);
    },
    Tile::Separator => {
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
    },
    Tile::TripleRoad => {
      mf.new_feature(t.id, 3, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
    },
    Tile::Curve => {
      mf.new_feature(t.id, 4, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 2, false);
    }
    Tile::QuadrupleRoad => {
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
    },
    Tile::Connector => {
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 1, false);
    },
    Tile::ConnectorWithCOA => {
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, true);
      mf.new_feature(t.id, 1, false);
    },
    Tile::Left => {
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 3, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 2, false);
    }
    Tile::Right => {
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 3, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 2, false);
    }
    Tile::TripleCity => {
      mf.new_feature(t.id, 3, false);
      mf.new_feature(t.id, 1, false);
    },
    Tile::TripleCityWithCOA => {
      mf.new_feature(t.id, 3, true);
      mf.new_feature(t.id, 1, false);
    },
    Tile::VerticalSeparator => {
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 1, false);
    },
    Tile::TripleCityWithRoad => {
      mf.new_feature(t.id, 3, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 1, false);
    },
    Tile::TripleCityWithRoadWithCOA => {
      mf.new_feature(t.id, 3, true);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 1, false);
      mf.new_feature(t.id, 1, false);
    },
    Tile::Triangle => {
      mf.new_feature(t.id, 2, false);
      mf.new_feature(t.id, 2, false);
    },
    Tile::TriangleWithCOA => {
      mf.new_feature(t.id, 2, true);
      mf.new_feature(t.id, 2, false);
    },
    Tile::QuadrupleCityWithCOA => {
      mf.new_feature(t.id, 4, true);
    },
    Tile::Invalid => {}
  }
}

fn set_cities_to_fields(mf: &mut MergeableFeature, t: &TileItem) {
  match t.tile {
    Tile::StartingTile => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    }
    Tile::Monastery => { },
    Tile::MonasteryWithRoad => { },
    Tile::CityCapWithCrossroad => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
    Tile::TriangleWithRoad => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
    Tile::TriangleWithRoadWithCOA => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
    Tile::Straight => {}
    Tile::CityCap => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
    Tile::Separator => {
      mf.set_cities((t.feature_starting_id + 2) as usize, (t.feature_starting_id + 0) as usize);
      mf.set_cities((t.feature_starting_id + 2) as usize, (t.feature_starting_id + 1) as usize);
    }
    Tile::TripleRoad => {}
    Tile::Curve => {}
    Tile::QuadrupleRoad => {}
    Tile::Connector => {
      mf.set_cities((t.feature_starting_id + 0) as usize, (t.feature_starting_id + 1) as usize);
      mf.set_cities((t.feature_starting_id + 2) as usize, (t.feature_starting_id + 1) as usize);
    }
    Tile::ConnectorWithCOA => {
      mf.set_cities((t.feature_starting_id + 0) as usize, (t.feature_starting_id + 1) as usize);
      mf.set_cities((t.feature_starting_id + 2) as usize, (t.feature_starting_id + 1) as usize);
    }
    Tile::Left => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
    Tile::Right => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
    Tile::TripleCity => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    }
    Tile::TripleCityWithCOA => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    }
    Tile::VerticalSeparator => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 2) as usize);
    }
    Tile::TripleCityWithRoad => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
      mf.set_cities((t.feature_starting_id + 3) as usize, (t.feature_starting_id + 0) as usize);
    }
    Tile::TripleCityWithRoadWithCOA => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
      mf.set_cities((t.feature_starting_id + 3) as usize, (t.feature_starting_id + 0) as usize);
    }
    Tile::Triangle => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
    Tile::TriangleWithCOA => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
    Tile::QuadrupleCityWithCOA => {}
    Tile::Invalid => {}
  }
}

fn merge_features(mf: &mut MergeableFeature, feat0: Vec<DistinctFeature>, feat1: Vec<DistinctFeature>) {
  if feat0.len() == 1 && feat1.len() == 1 {
    mf.unite(feat0[0].id as usize, feat1[0].id as usize);
  } else if feat0.len() == 3 && feat1.len() == 3 {
    mf.unite(feat0[0].id as usize, feat1[2].id as usize);
    mf.unite(feat0[1].id as usize, feat1[1].id as usize);
    mf.unite(feat0[2].id as usize, feat1[0].id as usize);
  }
}

pub fn calculate(moves: &Vec<Move>, get_final_status: bool) -> Result<Status, Error> {
  let board_size = 100;

  let mut mergeable_features = MergeableFeature::new();

  let mut board = vec![vec![Empty; board_size]; board_size];

  let mut meepleable_positions = vec![];
  let mut complete_events = vec![];
  let mut player0_point = 0;
  let mut player1_point = 0;

  let mut current_feature_id = 0;
  let mut current_tile_id = 0;

  for mv in moves {
    match mv {
      Move::TMove(m) => {
        let current_tile = TileItem {
          id: current_tile_id,
          tile: m.tile,
          rot: m.rot,
          feature_starting_id: current_feature_id,
        };

        create_mergeable_features(&mut mergeable_features, &current_tile);
        set_cities_to_fields(&mut mergeable_features, &current_tile);

        current_feature_id += current_tile.features().len() as i32;
        current_tile_id += 1;

        let y = m.pos.0 as usize;
        let x = m.pos.1 as usize;
        match board[y][x] {
          Empty => {}
          _ => return Err(Error{ msg: "invalid moves".to_string() })
        }
        match (y, x, &board[y - 1][x], &board[y + 1][x], &board[y][x - 1], &board[y][x + 1]) {
          (50, 50, _, _, _, _) => {} /* initial tile */
          (_, _, &Empty, &Empty, &Empty, &Empty) => {
            return Err(Error{ msg: "there must be at least one adjacent tile".to_string() })
          }
          (_, _, _, _, _, _) => {}
        }
        let top_must_be = match board[y - 1][x] { Tile(t) => t.bottom(), Empty => None };
        let bottom_must_be = match board[y + 1][x] { Tile(t) => t.top(), Empty => None };
        let left_must_be = match board[y][x - 1] { Tile(t) => t.right(), Empty => None };
        let right_must_be = match board[y][x + 1] { Tile(t) => t.left(), Empty => None };
        if top_must_be != None && top_must_be != current_tile.top() {
          return Err(Error{ msg: "top side is not correct".to_string() })
        }
        if bottom_must_be != None && bottom_must_be != current_tile.bottom() {
          return Err(Error{ msg: "bottom side is not correct".to_string() })
        }
        if left_must_be != None && left_must_be != current_tile.left() {
          return Err(Error{ msg: "left side is not correct".to_string() })
        }
        if right_must_be != None && right_must_be != current_tile.right() {
          return Err(Error{ msg: "right side is not correct".to_string() })
        }

        // place tile
        board[y][x] = Tile(current_tile);

        match board[y - 1][x] {
          Tile(t) => {
            assert_eq!(t.bottom_features().len(), current_tile.top_features().len());
            merge_features(&mut mergeable_features, t.bottom_features(), current_tile.top_features());
          }
          Empty => {}
        }
        match board[y + 1][x] {
          Tile(t) => {
            assert_eq!(t.top_features().len(), current_tile.bottom_features().len());
            merge_features(&mut mergeable_features, t.top_features(), current_tile.bottom_features());
          }
          Empty => {}
        }
        match board[y][x - 1] {
          Tile(t) => {
            assert_eq!(t.right_features().len(), current_tile.left_features().len());
            merge_features(&mut mergeable_features, t.right_features(), current_tile.left_features());
          }
          Empty => {}
        }
        match board[y][x + 1] {
          Tile(t) => {
            assert_eq!(t.left_features().len(), current_tile.right_features().len());
            merge_features(&mut mergeable_features, t.left_features(), current_tile.right_features());
          }
          Empty => {}
        }

        // update meepleable positions
        match board[y][x] {
          Empty => {
            return Err(Error{ msg: "tile on (y, x) must exist".to_string() })
          }
          Tile(t) => {
            meepleable_positions.clear();
            for f in &t.features() {
              if mergeable_features.get_meeples(f.id as usize).len() == 0 {
                assert!(f.id >= t.feature_starting_id);
                meepleable_positions.push(f.id - t.feature_starting_id);
              }
            }
          }
        }

        // update open side for monastery that was placed just now
        match m.tile {
          Tile::Monastery | Tile::MonasteryWithRoad => {
            let mut filled_count = 0;
            for dy in -1..2 {
              for dx in -1..2 {
                filled_count += match board[(y as i32 + dy) as usize][(x as i32 + dx) as usize] {
                  Empty => { 0 }
                  _ => { 1 }
                }
              }
            }
            assert!(filled_count <= 9);
            mergeable_features.reduce_open_sides(
              current_tile.feature_starting_id as usize,
              filled_count,
            );
          }
          _ => {}
        }

        // update open side for monastery that had been placed around the current tile
        for dy in -1..2 {
          for dx in -1..2 {
            if dy == 0 && dx == 0 {
              continue;
            }
            let ny = (y as i32 + dy) as usize;
            let nx = (x as i32 + dx) as usize;
            match board[ny][nx] {
              Tile(t) => {
                match t.tile {
                  Tile::Monastery | Tile::MonasteryWithRoad => {
                    mergeable_features.reduce_open_sides(t.feature_starting_id as usize, 1);
                  }
                  _ => {}
                }
              }
              Empty => { }
            }
          }
        }
      }
      Move::MMove(m) => {
        complete_events.clear();
        let y = m.tile_pos.0 as usize;
        let x = m.tile_pos.1 as usize;
        if m.meeple_id != -1 {
          match board[y][x] {
            Empty => {
              return Err(Error{ msg: "tile on (y, x) must exist".to_string() })
            }
            Tile(t) => {
              let feature_id = t.feature_starting_id + m.meeple_pos;
              if mergeable_features.get_meeples(feature_id as usize).len() != 0 {
                return Err(Error{ msg: "meepling on this feature is not allowed".to_string() })
              }
              mergeable_features.place_meeple(feature_id as usize, m.meeple_id);
            }
          }
        }
        match board[y][x] {
          Empty => {
            return Err(Error{ msg: "tile on (y, x) must exist".to_string() })
          }
          Tile(t) => {
            for f in &t.features() {
              if mergeable_features.is_completed(f.id as usize) {
                let sz = mergeable_features.size(f.id as usize);
                let meeple_ids = mergeable_features.get_meeples(f.id as usize);
                if meeple_ids.len() == 0 {
                  continue;
                }
                let pts = match f.feature {
                  RoadFeature => {
                    (sz * 1) as i32
                  }
                  CityFeature => {
                    (sz * 2) as i32
                  }
                  MonasteryFeature => {
                    9
                  }
                  FieldFeature => {
                    0
                  }
                };
                let mut player0_meeples = 0;
                let mut player1_meeples = 0;
                for meeple_id in &meeple_ids {
                  if *meeple_id < 7 {
                    player0_meeples += 1;
                  } else {
                    player1_meeples += 1;
                  }
                }
                if player0_meeples >= player1_meeples {
                  player0_point += pts;
                }
                if player1_meeples >= player0_meeples {
                  player1_point += pts;
                }
                complete_events.push(CompleteEvent {
                  feature: f.feature.clone(),
                  meeple_ids,
                  point: pts,
                });
              }
            }
          }
        }

        // resolve meeples on adjacent monasteries
        for dy in -1..2 {
          for dx in -1..2 {
            if dy == 0 && dx == 0 {
              continue;
            }
            let ny = (y as i32 + dy) as usize;
            let nx = (x as i32 + dx) as usize;
            match board[ny][nx] {
              Tile(t) => {
                match t.tile {
                  Tile::Monastery | Tile::MonasteryWithRoad => {
                    if mergeable_features.is_completed(t.feature_starting_id as usize) {
                      let meeple_ids = mergeable_features.get_meeples(t.feature_starting_id as usize);
                      if meeple_ids.len() == 0 {
                        continue;
                      }
                      if meeple_ids[0] < 7 {
                        player0_point += 9;
                      } else {
                        player1_point += 9;
                      }
                      complete_events.push(CompleteEvent {
                        feature: MonasteryFeature,
                        meeple_ids,
                        point: 9,
                      });
                    }
                  }
                  _ => {}
                }
              }
              Empty => { }
            }
          }
        }
      }
      Move::InvalidMove => {}
    }
  }

  if !get_final_status {
    return Ok(Status {
      meepleable_positions,
      complete_events,
      player0_point,
      player1_point,
    });
  }

  let mut complete_events = vec![];

  for y in 0..board_size {
    for x in 0..board_size {
      match board[y][x] {
        Tile(t) => {
          let fs = t.features();
          for f in &fs {
            let meeple_ids = mergeable_features.get_meeples(f.id as usize);
            if meeple_ids.len() == 0 {
              continue;
            }

            if mergeable_features.is_completed(f.id as usize) {
              continue;
            }
            if mergeable_features.is_done(f.id as usize) {
              continue;
            }

            let pts = match f.feature {
              RoadFeature => {
                let sz = mergeable_features.size(f.id as usize);
                sz as i32
              },
              CityFeature => {
                let sz = mergeable_features.size(f.id as usize);
                sz as i32
              },
              MonasteryFeature => {
                let open_sides = mergeable_features.get_open_sides(f.id as usize);
                (9 - open_sides) as i32
              },
              FieldFeature => {
                let mut p = 0;
                let cities = mergeable_features.get_facing_cities(f.id as usize);
                for city in &cities {
                  if mergeable_features.is_completed(*city) {
                    p += 3;
                  }
                }
                p
              },
            };
            let mut player0_meeples = 0;
            let mut player1_meeples = 0;
            for meeple_id in &meeple_ids {
              if *meeple_id < 7 {
                player0_meeples += 1;
              } else {
                player1_meeples += 1;
              }
            }
            if player0_meeples >= player1_meeples {
              player0_point += pts;
            }
            if player1_meeples >= player0_meeples {
              player1_point += pts;
            }
            complete_events.push(CompleteEvent {
              feature: f.feature.clone(),
              meeple_ids,
              point: pts,
            });

            mergeable_features.set_as_done(f.id as usize);
          }
        }
        Empty => {}
      }
    }
  }

  Ok(Status {
    meepleable_positions,
    complete_events,
    player0_point,
    player1_point,
  })
}

#[test]
fn calculate_test_for_road_and_city_completion() {
  let game_id = 0;
  let player0_id = 0;
  let player1_id = 1;
  let mut mvs = vec![
    Move::TMove( TileMove { ord: 0, game_id, player_id: player1_id, tile: Tile::StartingTile, rot: 0, pos: (50, 50) } ),
    Move::MMove( MeepleMove { ord: 1, game_id, player_id: player1_id, meeple_id: -1, tile_pos: (50, 50), meeple_pos: -1 } ),
  ];

  mvs.push(Move::TMove( TileMove { ord: 2, game_id, player_id: player0_id, tile: Tile::TriangleWithRoad, rot: 2, pos: (49, 50) } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => { assert_eq!(res.meepleable_positions, vec![0, 1, 2, 3]); },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::MMove( MeepleMove { ord: 3, game_id, player_id: player0_id, meeple_id: 0, tile_pos: (49, 50), meeple_pos: 0 } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 0);
      assert_eq!(res.player0_point, 0);
      assert_eq!(res.player1_point, 0);
    },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::TMove( TileMove { ord: 4, game_id, player_id: player1_id, tile: Tile::CityCapWithCrossroad, rot: 3, pos: (50, 49) } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => { assert_eq!(res.meepleable_positions, vec![0, 1, 2, 3, 4, 5, 6]); },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::MMove( MeepleMove { ord: 5, game_id, player_id: player1_id, meeple_id: 7, tile_pos: (50, 49), meeple_pos: 0 } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 0);
      assert_eq!(res.player0_point, 0);
      assert_eq!(res.player1_point, 0);
    },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::TMove( TileMove { ord: 6, game_id, player_id: player0_id, tile: Tile::CityCapWithCrossroad, rot: 0, pos: (50, 51) } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => { assert_eq!(res.meepleable_positions, vec![0, 1, 2, 3, 4, 5, 6]); },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::MMove( MeepleMove { ord: 7, game_id, player_id: player0_id, meeple_id: 1, tile_pos: (50, 51), meeple_pos: 2 } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, RoadFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
      assert_eq!(res.complete_events[0].point, 3);
      assert_eq!(res.player0_point, 3);
      assert_eq!(res.player1_point, 0);
    },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::TMove( TileMove { ord: 8, game_id, player_id: player1_id, tile: Tile::StartingTile, rot: 1, pos: (50, 48) } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => { assert_eq!(res.meepleable_positions, vec![1, 2, 3]); },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::MMove( MeepleMove { ord: 9, game_id, player_id: player1_id, meeple_id: 8, tile_pos: (50, 48), meeple_pos: 2 } ));
  let status = calculate(&mvs,false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![7]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 3);
      assert_eq!(res.player1_point, 4);
    },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::TMove( TileMove { ord: 10, game_id, player_id: player0_id, tile: Tile::TriangleWithRoadWithCOA, rot: 3, pos: (49, 51) } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => { assert_eq!(res.meepleable_positions, vec![1, 2, 3]); },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::MMove( MeepleMove { ord: 11, game_id, player_id: player0_id, meeple_id: 1, tile_pos: (49, 51), meeple_pos: 1 } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![0]);
      assert_eq!(res.complete_events[0].point, 10);
      assert_eq!(res.player0_point, 13);
      assert_eq!(res.player1_point, 4);
    },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  let status = calculate(&mvs, true);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 2);
      assert_eq!(res.complete_events[0].feature, FieldFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
      assert_eq!(res.complete_events[0].point, 3);
      assert_eq!(res.complete_events[1].feature, RoadFeature);
      assert_eq!(res.complete_events[1].meeple_ids, vec![8]);
      assert_eq!(res.complete_events[1].point, 1);
      assert_eq!(res.player0_point, 16);
      assert_eq!(res.player1_point, 5);
    },
    Err(e) => { panic!("Error: {}", e.msg); }
  }
}

#[test]
fn calculate_test_for_monastery_completion() {
  let game_id = 0;
  let player0_id = 0;
  let player1_id = 1;
  let mut mvs = vec![
    Move::TMove( TileMove { ord: 0, game_id, player_id: player1_id, tile: Tile::StartingTile, rot: 0, pos: (50, 50) } ),
    Move::MMove( MeepleMove { ord: 1, game_id, player_id: player1_id, meeple_id: -1, tile_pos: (50, 50), meeple_pos: -1 } ),
  ];

  mvs.push(Move::TMove( TileMove { ord: 2, game_id, player_id: player0_id, tile: Tile::Monastery, rot: 0, pos: (51, 50) } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => { assert_eq!(res.meepleable_positions, vec![0, 1]); },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::MMove( MeepleMove { ord: 3, game_id, player_id: player0_id, meeple_id: 0, tile_pos: (51, 50), meeple_pos: 0 } ));
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 0);
      assert_eq!(res.player0_point, 0);
      assert_eq!(res.player1_point, 0);
    },
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  mvs.push(Move::TMove( TileMove { ord: 4, game_id, player_id: player1_id, tile: Tile::Monastery, rot: 0, pos: (52, 50) } ));
  mvs.push(Move::MMove( MeepleMove { ord: 5, game_id, player_id: player1_id, meeple_id: -1, tile_pos: (52, 50), meeple_pos: -1 } ));

  mvs.push(Move::TMove( TileMove { ord: 6, game_id, player_id: player0_id, tile: Tile::Monastery, rot: 0, pos: (52, 49) } ));
  mvs.push(Move::MMove( MeepleMove { ord: 7, game_id, player_id: player0_id, meeple_id: -1, tile_pos: (52, 49), meeple_pos: -1 } ));

  mvs.push(Move::TMove( TileMove { ord: 8, game_id, player_id: player1_id, tile: Tile::Monastery, rot: 0, pos: (52, 48) } ));
  mvs.push(Move::MMove( MeepleMove { ord: 9, game_id, player_id: player1_id, meeple_id: -1, tile_pos: (52, 48), meeple_pos: -1 } ));

  mvs.push(Move::TMove( TileMove { ord: 10, game_id, player_id: player0_id, tile: Tile::Monastery, rot: 0, pos: (51, 48) } ));
  mvs.push(Move::MMove( MeepleMove { ord: 11, game_id, player_id: player0_id, meeple_id: -1, tile_pos: (51, 48), meeple_pos: -1 } ));

  mvs.push(Move::TMove( TileMove { ord: 12, game_id, player_id: player0_id, tile: Tile::Monastery, rot: 0, pos: (52, 51) } ));
  mvs.push(Move::MMove( MeepleMove { ord: 13, game_id, player_id: player0_id, meeple_id: -1, tile_pos: (52, 51), meeple_pos: -1 } ));

  mvs.push(Move::TMove( TileMove { ord: 14, game_id, player_id: player1_id, tile: Tile::Monastery, rot: 0, pos: (51, 51) } ));
  mvs.push(Move::MMove( MeepleMove { ord: 15, game_id, player_id: player1_id, meeple_id: -1, tile_pos: (51, 51), meeple_pos: -1 } ));

  mvs.push(Move::TMove( TileMove { ord: 16, game_id, player_id: player0_id, tile: Tile::StartingTile, rot: 0, pos: (50, 51) } ));
  mvs.push(Move::MMove( MeepleMove { ord: 17, game_id, player_id: player0_id, meeple_id: -1, tile_pos: (50, 51), meeple_pos: -1 } ));

  mvs.push(Move::TMove( TileMove { ord: 18, game_id, player_id: player1_id, tile: Tile::StartingTile, rot: 0, pos: (50, 49) } ));
  mvs.push(Move::MMove( MeepleMove { ord: 19, game_id, player_id: player1_id, meeple_id: -1, tile_pos: (50, 49), meeple_pos: -1 } ));

  mvs.push(Move::TMove( TileMove { ord: 20, game_id, player_id: player0_id, tile: Tile::StartingTile, rot: 0, pos: (50, 48) } ));
  mvs.push(Move::MMove( MeepleMove { ord: 21, game_id, player_id: player0_id, meeple_id: -1, tile_pos: (50, 48), meeple_pos: -1 } ));

  mvs.push(Move::TMove( TileMove { ord: 22, game_id, player_id: player1_id, tile: Tile::Monastery, rot: 0, pos: (51, 49) } ));
  mvs.push(Move::MMove( MeepleMove { ord: 23, game_id, player_id: player1_id, meeple_id: 7, tile_pos: (51, 49), meeple_pos: 0 } ));

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 2);
      assert_eq!(res.complete_events[0].feature, MonasteryFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![7]);
      assert_eq!(res.complete_events[0].point, 9);
      assert_eq!(res.complete_events[1].feature, MonasteryFeature);
      assert_eq!(res.complete_events[1].meeple_ids, vec![0]);
      assert_eq!(res.complete_events[1].point, 9);
      assert_eq!(res.player0_point, 9);
      assert_eq!(res.player1_point, 9);
    },
    Err(e) => { panic!("Error: {}", e.msg); }
  }
}

#[allow(dead_code)]
fn add_move(mvs: &mut Vec<Move>, tile: Tile, rot: i32, pos: (i32, i32), meeple_id: i32, meeple_pos: i32) {
  mvs.push(Move::TMove( TileMove { ord: -1, game_id: -1, player_id: -1, tile, rot, pos, } ));
  mvs.push(Move::MMove( MeepleMove { ord: -1, game_id: -1, player_id: -1, meeple_id: meeple_id, tile_pos: pos, meeple_pos } ));
}

#[test]
fn calculate_test0() {
  /* actual game here: https://boardgamearena.com/table?table=361472535 */
  let mut mvs = vec![];

  add_move(&mut mvs, Tile::StartingTile, 0, (50, 50), -1, -1);
  add_move(&mut mvs, Tile::Straight, 1, (50, 51), 0, 1);
  add_move(&mut mvs, Tile::CityCap, 2, (49, 50), 7, 0);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![7]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 0);
      assert_eq!(res.player1_point, 4);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::Separator, 2, (51, 50), 1, 0);
  add_move(&mut mvs, Tile::StartingTile, 3, (51, 49), 7, 0);
  add_move(&mut mvs, Tile::TripleRoad, 0, (50, 49), 2, 4);
  add_move(&mut mvs, Tile::CityCap, 3, (51, 51), 8, 0);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![8]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 0);
      assert_eq!(res.player1_point, 8);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::Curve, 1, (50, 52), -1, -1);
  add_move(&mut mvs, Tile::TripleRoad, 0, (52, 51), 8, 1);
  add_move(&mut mvs, Tile::QuadrupleRoad, 0, (50, 48), 3, 4);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, RoadFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![3]);
      assert_eq!(res.complete_events[0].point, 2);
      assert_eq!(res.player0_point, 2);
      assert_eq!(res.player1_point, 8);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::TripleRoad, 2, (53, 51), 9, 4);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, RoadFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![9]);
      assert_eq!(res.complete_events[0].point, 2);
      assert_eq!(res.player0_point, 2);
      assert_eq!(res.player1_point, 10);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::CityCapWithCrossroad, 0, (49, 52), 3, 0);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, RoadFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![0]);
      assert_eq!(res.complete_events[0].point, 5);
      assert_eq!(res.player0_point, 7);
      assert_eq!(res.player1_point, 10);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::Straight, 1, (49, 53), 9, 1);
  add_move(&mut mvs, Tile::Curve, 0, (52, 52), 10, 1);
  add_move(&mut mvs, Tile::ConnectorWithCOA, 1, (48, 52), -1, -1);
  add_move(&mut mvs, Tile::Straight, 0, (52, 49), -1, -1);
  add_move(&mut mvs, Tile::Left, 1, (51, 48), 11, 2);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![7]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 7);
      assert_eq!(res.player1_point, 14);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::CityCap, 2, (47, 52), -1, -1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![3]);
      assert_eq!(res.complete_events[0].point, 8);
      assert_eq!(res.player0_point, 15);
      assert_eq!(res.player1_point, 14);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::CityCap, 0, (46, 52), 7, 0);
  add_move(&mut mvs, Tile::TripleCity, 2, (45, 52), -1, -1);
  add_move(&mut mvs, Tile::TriangleWithRoad, 2, (53, 52), 12, 0);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, RoadFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![10]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 15);
      assert_eq!(res.player1_point, 18);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::TripleCity, 1, (46, 53), -1, -1);
  add_move(&mut mvs, Tile::Curve, 3, (50, 47), -1, -1);
  add_move(&mut mvs, Tile::CityCapWithCrossroad, 2, (53, 49), 0, 0);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, RoadFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![2]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 19);
      assert_eq!(res.player1_point, 18);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::TripleCity, 2, (45, 53), -1, -1);
  add_move(&mut mvs, Tile::StartingTile, 0, (54, 49), -1, -1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![0]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 23);
      assert_eq!(res.player1_point, 18);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::VerticalSeparator, 1, (45, 51), 10, 2);
  add_move(&mut mvs, Tile::Right, 0, (52, 50), 0, 1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 27);
      assert_eq!(res.player1_point, 18);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::TripleCityWithCOA, 3, (46, 54), -1, -1);
  add_move(&mut mvs, Tile::VerticalSeparator, 0, (46, 55), 1, 2);
  add_move(&mut mvs, Tile::Left, 2, (45, 55), 13, 0);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![13]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 27);
      assert_eq!(res.player1_point, 22);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::TripleCityWithRoad, 3, (47, 55), -1, -1);
  add_move(&mut mvs, Tile::Monastery, 0, (51, 52), 13, 0);
  add_move(&mut mvs, Tile::MonasteryWithRoad, 3, (49, 51), 2, 0);
  add_move(&mut mvs, Tile::TripleRoad, 2, (53, 50), -1, -1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, RoadFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![8]);
      assert_eq!(res.complete_events[0].point, 3);
      assert_eq!(res.player0_point, 27);
      assert_eq!(res.player1_point, 25);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::Right, 0, (44, 55), 3, 0);
  add_move(&mut mvs, Tile::TriangleWithRoad, 1, (54, 52), -1, -1);
  add_move(&mut mvs, Tile::Straight, 0, (50, 46), -1, -1);
  add_move(&mut mvs, Tile::TriangleWithRoad, 3, (51, 47), -1, -1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, RoadFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![11]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 27);
      assert_eq!(res.player1_point, 29);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::Monastery, 0, (48, 51), 4, 0);
  add_move(&mut mvs, Tile::TriangleWithCOA, 3, (53, 53), 8, 1);
  add_move(&mut mvs, Tile::TripleCityWithRoadWithCOA, 1, (47, 54), -1, -1);
  add_move(&mut mvs, Tile::Curve, 2, (52, 53), -1, -1);
  add_move(&mut mvs, Tile::Straight, 1, (54, 48), 5, 0);
  add_move(&mut mvs, Tile::Straight, 0, (51, 53), -1, -1);
  add_move(&mut mvs, Tile::MonasteryWithRoad, 2, (48, 50), 6, 0);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, MonasteryFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![2]);
      assert_eq!(res.complete_events[0].point, 9);
      assert_eq!(res.player0_point, 36);
      assert_eq!(res.player1_point, 29);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::Connector, 0, (45, 50), -1, -1);
  add_move(&mut mvs, Tile::Straight, 1, (53, 48), -1, -1);
  add_move(&mut mvs, Tile::Curve, 3, (50, 53), -1, -1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, MonasteryFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![13]);
      assert_eq!(res.complete_events[0].point, 9);
      assert_eq!(res.player0_point, 36);
      assert_eq!(res.player1_point, 38);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::TriangleWithRoadWithCOA, 1, (48, 54), -1, -1);
  add_move(&mut mvs, Tile::Straight, 0, (51, 54), 11, 2);
  add_move(&mut mvs, Tile::Curve, 1, (47, 51), -1, -1);
  add_move(&mut mvs, Tile::Right, 2, (52, 54), 13, 2);
  add_move(&mut mvs, Tile::StartingTile, 3, (55, 52), -1, -1);
  add_move(&mut mvs, Tile::Curve, 0, (50, 54), -1, -1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, RoadFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![13]);
      assert_eq!(res.complete_events[0].point, 6);
      assert_eq!(res.player0_point, 36);
      assert_eq!(res.player1_point, 44);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::Monastery, 0, (55, 53), 2, 1);
  add_move(&mut mvs, Tile::TriangleWithRoad, 1, (55, 51), -1, -1);
  add_move(&mut mvs, Tile::Left, 2, (43, 55), -1, -1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![3]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 40);
      assert_eq!(res.player1_point, 44);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::Monastery, 0, (52, 48), 13, 0);
  add_move(&mut mvs, Tile::TriangleWithCOA, 1, (53, 54), -1, -1);
  add_move(&mut mvs, Tile::CityCap, 1, (45, 49), -1, -1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![10]);
      assert_eq!(res.complete_events[0].point, 6);
      assert_eq!(res.player0_point, 40);
      assert_eq!(res.player1_point, 50);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::QuadrupleCityWithCOA, 0, (48, 55), -1, -1);
  add_move(&mut mvs, Tile::ConnectorWithCOA, 0, (53, 55), 13, 1);
  add_move(&mut mvs, Tile::CityCapWithCrossroad, 3, (47, 50), 3, 2);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 2);
      assert_eq!(res.complete_events[0].feature, RoadFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![3]);
      assert_eq!(res.complete_events[0].point, 2);
      assert_eq!(res.complete_events[1].feature, MonasteryFeature);
      assert_eq!(res.complete_events[1].meeple_ids, vec![4]);
      assert_eq!(res.complete_events[1].point, 9);
      assert_eq!(res.player0_point, 51);
      assert_eq!(res.player1_point, 50);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::VerticalSeparator, 1, (53, 56), -1, -1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![13]);
      assert_eq!(res.complete_events[0].point, 12);
      assert_eq!(res.player0_point, 51);
      assert_eq!(res.player1_point, 62);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::Curve, 0, (46, 51), 3, 0);
  add_move(&mut mvs, Tile::Triangle, 2, (54, 55), 13, 1);
  add_move(&mut mvs, Tile::Triangle, 2, (55, 54), -1, -1);
  add_move(&mut mvs, Tile::Curve, 1, (49, 54), -1, -1);
  add_move(&mut mvs, Tile::Separator, 2, (47, 49), 4, 1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![4]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 55);
      assert_eq!(res.player1_point, 62);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::Triangle, 0, (54, 53), -1, -1);

  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events.len(), 1);
      assert_eq!(res.complete_events[0].feature, CityFeature);
      assert_eq!(res.complete_events[0].meeple_ids, vec![12]);
      assert_eq!(res.complete_events[0].point, 10);
      assert_eq!(res.player0_point, 55);
      assert_eq!(res.player1_point, 72);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }

  add_move(&mut mvs, Tile::TripleCityWithRoadWithCOA, 1, (44, 56), 4, 1);

  let status = calculate(&mvs, true);
  match status {
    Ok(res) => {
      assert_eq!(res.player0_point, 113);
      assert_eq!(res.player1_point, 116);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
}

#[test]
fn calculate_test1() {
  /* actual game here: https://boardgamearena.com/table?table=361578832 */
  let mut mvs = vec![];
  add_move(&mut mvs, Tile::StartingTile, 0, (50, 50), -1, -1);
  add_move(&mut mvs, Tile::Curve, 1, (50, 51), 0, 1);
  add_move(&mut mvs, Tile::TriangleWithRoad, 3, (49, 50), 7, 0);
  add_move(&mut mvs, Tile::TripleRoad, 1, (49, 51), 1, 4);
  add_move(&mut mvs, Tile::TripleCityWithRoad, 0, (48, 51), 8, 2);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![8]);
      assert_eq!(res.complete_events[0].point, 2);
      assert_eq!(res.player0_point, 0);
      assert_eq!(res.player1_point, 2);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Triangle, 0, (48, 52), 2, 0);
  add_move(&mut mvs, Tile::TripleCityWithCOA, 0, (49, 49), -1, -1);
  add_move(&mut mvs, Tile::CityCap, 1, (50, 52), 3, 0);
  add_move(&mut mvs, Tile::TripleRoad, 3, (48, 53), 8, 2);
  add_move(&mut mvs, Tile::ConnectorWithCOA, 0, (50, 53), -1, -1);
  add_move(&mut mvs, Tile::TripleCityWithRoadWithCOA, 0, (47, 53), 9, 0);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![8]);
      assert_eq!(res.complete_events[0].point, 2);
      assert_eq!(res.player0_point, 0);
      assert_eq!(res.player1_point, 4);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Curve, 3, (50, 49), -1, -1);
  add_move(&mut mvs, Tile::CityCapWithCrossroad, 0, (48, 54), 8, 2);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![8]);
      assert_eq!(res.complete_events[0].point, 2);
      assert_eq!(res.player0_point, 0);
      assert_eq!(res.player1_point, 6);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Right, 3, (50, 54), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![3]);
      assert_eq!(res.complete_events[0].point, 8);
      assert_eq!(res.player0_point, 8);
      assert_eq!(res.player1_point, 6);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::TripleRoad, 0, (51, 50), 8, 1);
  add_move(&mut mvs, Tile::Connector, 1, (50, 48), 3, 1);
  add_move(&mut mvs, Tile::QuadrupleCityWithCOA, 0, (48, 49), -1, -1);
  add_move(&mut mvs, Tile::Triangle, 2, (49, 48), -1, -1);
  add_move(&mut mvs, Tile::TripleCity, 2, (47, 49), -1, -1);
  add_move(&mut mvs, Tile::TripleCityWithRoadWithCOA, 0, (48, 50), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
      assert_eq!(res.complete_events[0].point, 3);
      assert_eq!(res.player0_point, 11);
      assert_eq!(res.player1_point, 6);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Straight, 0, (49, 54), 10, 1);
  add_move(&mut mvs, Tile::VerticalSeparator, 0, (51, 48), 1, 2);
  add_move(&mut mvs, Tile::TriangleWithCOA, 2, (47, 52), -1, -1);
  add_move(&mut mvs, Tile::TriangleWithCOA, 1, (46, 52), 4, 0);
  add_move(&mut mvs, Tile::Curve, 2, (51, 49), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![0, 8]);
      assert_eq!(res.complete_events[0].point, 6);
      assert_eq!(res.player0_point, 17);
      assert_eq!(res.player1_point, 12);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::StartingTile, 0, (52, 48), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 21);
      assert_eq!(res.player1_point, 12);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Monastery, 0, (49, 52), 8, 0);
  add_move(&mut mvs, Tile::MonasteryWithRoad, 2, (49, 53), 0, 0);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![0]);
      assert_eq!(res.complete_events[0].point, 9);
      assert_eq!(res.complete_events[1].meeple_ids, vec![8]);
      assert_eq!(res.complete_events[1].point, 9);
      assert_eq!(res.player0_point, 30);
      assert_eq!(res.player1_point, 21);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::TripleCity, 3, (47, 54), -1, -1);
  add_move(&mut mvs, Tile::Straight, 0, (47, 55), -1, -1);
  add_move(&mut mvs, Tile::Straight, 0, (46, 51), -1, -1);
  add_move(&mut mvs, Tile::Curve, 0, (46, 55), -1, -1);
  add_move(&mut mvs, Tile::Left, 2, (46, 54), -1, -1);
  add_move(&mut mvs, Tile::CityCap, 2, (53, 48), 0, 0);
  add_move(&mut mvs, Tile::Curve, 3, (45, 54), 8, 1);
  add_move(&mut mvs, Tile::Triangle, 3, (46, 53), -1, -1);
  add_move(&mut mvs, Tile::TriangleWithRoadWithCOA, 1, (45, 51), 11, 0);
  add_move(&mut mvs, Tile::MonasteryWithRoad, 2, (45, 53), 5, 0);
  add_move(&mut mvs, Tile::TripleRoad, 1, (50, 55), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![10]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 30);
      assert_eq!(res.player1_point, 25);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Separator, 1, (54, 48), 1, 0);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![0]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 34);
      assert_eq!(res.player1_point, 25);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Right, 3, (52, 47), 12, 0);
  add_move(&mut mvs, Tile::Left, 3, (54, 49), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 38);
      assert_eq!(res.player1_point, 25);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Straight, 1, (45, 55), -1, -1);
  add_move(&mut mvs, Tile::Monastery, 0, (53, 47), 0, 0);
  add_move(&mut mvs, Tile::TriangleWithRoadWithCOA, 2, (48, 55), -1, -1);
  add_move(&mut mvs, Tile::TriangleWithRoad, 0, (44, 53), 1, 0);
  add_move(&mut mvs, Tile::Straight, 1, (45, 56), -1, -1);
  add_move(&mut mvs, Tile::Curve, 1, (44, 54), -1, -1);
  add_move(&mut mvs, Tile::QuadrupleRoad, 1, (45, 57), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![8]);
      assert_eq!(res.complete_events[0].point, 9);
      assert_eq!(res.player0_point, 38);
      assert_eq!(res.player1_point, 34);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::CityCap, 1, (44, 52), -1, -1);
  add_move(&mut mvs, Tile::Straight, 0, (43, 54), 8, 1);
  add_move(&mut mvs, Tile::CityCapWithCrossroad, 2, (51, 55), 6, 0);
  add_move(&mut mvs, Tile::Curve, 2, (43, 52), -1, -1);
  add_move(&mut mvs, Tile::CityCap, 0, (52, 55), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![6]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 42);
      assert_eq!(res.player1_point, 34);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::TriangleWithRoad, 0, (42, 54), -1, -1);
  add_move(&mut mvs, Tile::CityCapWithCrossroad, 0, (42, 52), 6, 0);
  add_move(&mut mvs, Tile::VerticalSeparator, 2, (46, 50), 10, 0);
  add_move(&mut mvs, Tile::Curve, 0, (43, 51), -1, -1);
  add_move(&mut mvs, Tile::ConnectorWithCOA, 0, (52, 46), -1, -1);
  add_move(&mut mvs, Tile::StartingTile, 2, (41, 52), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![6]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 46);
      assert_eq!(res.player1_point, 34);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Monastery, 0, (44, 55), -1, -1);
  add_move(&mut mvs, Tile::Curve, 0, (54, 47), -1, -1);
  add_move(&mut mvs, Tile::Straight, 1, (42, 55), -1, -1);
  add_move(&mut mvs, Tile::VerticalSeparator, 1, (47, 48), 6, 2);
  add_move(&mut mvs, Tile::StartingTile, 1, (52, 45), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![12]);
      assert_eq!(res.complete_events[0].point, 8);
      assert_eq!(res.player0_point, 46);
      assert_eq!(res.player1_point, 42);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Straight, 1, (54, 46), -1, -1);
  add_move(&mut mvs, Tile::TripleCity, 3, (47, 50), -1, -1);
  add_move(&mut mvs, Tile::CityCap, 1, (47, 47), -1, -1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![6]);
      assert_eq!(res.complete_events[0].point, 4);
      assert_eq!(res.player0_point, 50);
      assert_eq!(res.player1_point, 42);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Left, 3, (55, 48), 12, 1);
  add_move(&mut mvs, Tile::Right, 2, (43, 53), 6, 1);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
      assert_eq!(res.complete_events[0].point, 6);
      assert_eq!(res.player0_point, 56);
      assert_eq!(res.player1_point, 42);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  add_move(&mut mvs, Tile::Separator, 2, (51, 53), 13, 2);
  add_move(&mut mvs, Tile::Monastery, 0, (53, 46), 1, 0);
  let status = calculate(&mvs, false);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].meeple_ids, vec![0]);
      assert_eq!(res.complete_events[0].point, 9);
      assert_eq!(res.player0_point, 65);
      assert_eq!(res.player1_point, 42);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
  let status = calculate(&mvs, true);
  match status {
    Ok(res) => {
      assert_eq!(res.complete_events[0].feature, FieldFeature);
      assert_eq!(res.complete_events[0].meeple_ids, [6]);
      assert_eq!(res.complete_events[0].point, 3);
      assert_eq!(res.complete_events[1].feature, RoadFeature);
      assert_eq!(res.complete_events[1].meeple_ids, [8]);
      assert_eq!(res.complete_events[1].point, 6);
      assert_eq!(res.complete_events[2].feature, CityFeature);
      assert_eq!(res.complete_events[2].meeple_ids, [11]);
      assert_eq!(res.complete_events[2].point, 2);
      assert_eq!(res.complete_events[3].feature, MonasteryFeature);
      assert_eq!(res.complete_events[3].meeple_ids, [5]);
      assert_eq!(res.complete_events[3].point, 8);
      assert_eq!(res.complete_events[4].feature, CityFeature);
      assert_eq!(res.complete_events[4].meeple_ids, [7, 3, 2, 9, 4, 10]);
      assert_eq!(res.complete_events[4].point, 27);
      assert_eq!(res.complete_events[5].feature, FieldFeature);
      assert_eq!(res.complete_events[5].meeple_ids, [13]);
      assert_eq!(res.complete_events[5].point, 3);
      assert_eq!(res.complete_events[6].feature, FieldFeature);
      assert_eq!(res.complete_events[6].meeple_ids, [12]);
      assert_eq!(res.complete_events[6].point, 9);
      assert_eq!(res.complete_events[7].feature, MonasteryFeature);
      assert_eq!(res.complete_events[7].meeple_ids, [1]);
      assert_eq!(res.complete_events[7].point, 7);
      assert_eq!(res.player0_point, 110);
      assert_eq!(res.player1_point, 89);
    }
    Err(e) => { panic!("Error: {}", e.msg); }
  }
}
