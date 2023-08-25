use std::cmp::Ordering;
use std::cmp::Ordering::*;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

use self::Feature::*;
use self::Side::*;
#[allow(unused_imports)]
use super::decoder;
use super::mergeable_feature::MergeableFeature;
use super::mov::{MeepleMove, Move, TileMove};
use super::tile::Tile;
use crate::error::{moves_invalid_error, Error};

#[derive(Debug)]
pub struct CompleteEvent {
    pub feature: Feature,
    pub meeple_ids: Vec<i32>,
    pub point: i32,
}
impl Ord for CompleteEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.feature != other.feature {
            if self.feature < other.feature {
                return Less;
            } else {
                return Greater;
            }
        } else if self.point != other.point {
            if self.point < other.point {
                return Less;
            } else {
                return Greater;
            }
        } else if self.meeple_ids[0] < other.meeple_ids[0] {
            return Less;
        } else if self.meeple_ids[0] > other.meeple_ids[0] {
            return Greater;
        } else {
            Equal
        }
    }
}
impl PartialOrd for CompleteEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for CompleteEvent {
    fn eq(&self, other: &Self) -> bool {
        self.feature == other.feature
            && self.point == other.point
            && self.meeple_ids[0] == other.meeple_ids[0]
    }
}
impl Eq for CompleteEvent {}

pub struct Status {
    pub meepleable_positions: Vec<i32>,
    pub complete_events: Vec<CompleteEvent>,
    pub player0_point: i32,
    pub player1_point: i32,
    pub board: HashMap<(i32, i32), TileItem>,
    pub player0_remaining_meeples: HashSet<i32>,
    pub player1_remaining_meeples: HashSet<i32>,
    pub tile_id_to_pos: HashMap<i32, (i32, i32)>,
    pub mergeable_features: MergeableFeature,
    pub feature_num: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct TileItem {
    pub id: i32,
    pub tile: Tile,
    pub rot: i32,
    pub feature_starting_id: i32,
    pub meeple_id: Option<i32>,
    pub meeple_pos: Option<i32>,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Side {
    Field,
    Road,
    City,
    NoSide,
}

#[derive(Eq, PartialEq, PartialOrd, Clone, Debug)]
pub enum Feature {
    RoadFeature,
    CityFeature,
    MonasteryFeature,
    FieldFeature,
}

impl Feature {
    pub fn to_string(self) -> String {
        match self {
            FieldFeature => "field".to_string(),
            RoadFeature => "road".to_string(),
            CityFeature => "city".to_string(),
            MonasteryFeature => "monastery".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DistinctFeature {
    pub id: i32,
    pub feature: Feature,
}

#[derive(Debug)]
pub struct TileablePosition {
    pub pos: (i32, i32),
    pub rot: i32,
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
    #[allow(dead_code)]
    pub fn rotate(&mut self) {
        self.rot += 1;
    }
    pub fn right(self) -> Side {
        self.sides()[((self.rot + 0) % 4) as usize]
    }
    pub fn top(self) -> Side {
        self.sides()[((self.rot + 1) % 4) as usize]
    }
    pub fn left(self) -> Side {
        self.sides()[((self.rot + 2) % 4) as usize]
    }
    pub fn bottom(self) -> Side {
        self.sides()[((self.rot + 3) % 4) as usize]
    }
    pub fn right_features(self) -> Vec<DistinctFeature> {
        self.side_features()[((self.rot + 0) % 4) as usize].clone()
    }
    pub fn top_features(self) -> Vec<DistinctFeature> {
        self.side_features()[((self.rot + 1) % 4) as usize].clone()
    }
    pub fn left_features(self) -> Vec<DistinctFeature> {
        self.side_features()[((self.rot + 2) % 4) as usize].clone()
    }
    pub fn bottom_features(self) -> Vec<DistinctFeature> {
        self.side_features()[((self.rot + 3) % 4) as usize].clone()
    }
    // side_features defined so that they are not influenced by the effect of turn
    fn side_features(self) -> Vec<Vec<DistinctFeature>> {
        match self.tile {
            Tile::StartingTile => vec![
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                ], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                ], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: FieldFeature,
                }], // bottom
            ],
            Tile::Monastery => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // bottom
            ],
            Tile::MonasteryWithRoad => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::CityCapWithCrossroad => vec![
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 6,
                        feature: FieldFeature,
                    },
                ], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 4,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                ], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 6,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 5,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 4,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::TriangleWithRoad => vec![
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                ], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::TriangleWithRoadWithCOA => vec![
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                ], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::Straight => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                }], // right
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 0,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: FieldFeature,
                    },
                ], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                }], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 0,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::CityCap => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // bottom
            ],
            Tile::Separator => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: CityFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                }], // bottom
            ],
            Tile::TripleRoad => vec![
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 0,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 5,
                        feature: FieldFeature,
                    },
                ], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                }], // top
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 0,
                        feature: FieldFeature,
                    },
                ], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 5,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 4,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::Curve => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                }], // top
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 0,
                        feature: FieldFeature,
                    },
                ], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 0,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::QuadrupleRoad => vec![
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 4,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 7,
                        feature: FieldFeature,
                    },
                ], // right
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 0,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: FieldFeature,
                    },
                ], // top
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 5,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 0,
                        feature: FieldFeature,
                    },
                ], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 7,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 6,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 5,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::Connector => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: CityFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: CityFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                }], // bottom
            ],
            Tile::ConnectorWithCOA => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: CityFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: CityFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                }], // bottom
            ],
            Tile::Left => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                ], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::Right => vec![
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                ], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::TripleCity => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // bottom
            ],
            Tile::TripleCityWithCOA => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // bottom
            ],
            Tile::VerticalSeparator => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: CityFeature,
                }], // bottom
            ],
            Tile::TripleCityWithRoad => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::TripleCityWithRoadWithCOA => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // left
                vec![
                    DistinctFeature {
                        id: self.feature_starting_id + 3,
                        feature: FieldFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 2,
                        feature: RoadFeature,
                    },
                    DistinctFeature {
                        id: self.feature_starting_id + 1,
                        feature: FieldFeature,
                    },
                ], // bottom
            ],
            Tile::Triangle => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // bottom
            ],
            Tile::TriangleWithCOA => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                }], // bottom
            ],
            Tile::QuadrupleCityWithCOA => vec![
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // right
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // top
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // left
                vec![DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                }], // bottom
            ],
            Tile::Invalid => vec![vec![]],
        }
    }
    pub fn features(self) -> Vec<DistinctFeature> {
        match self.tile {
            Tile::StartingTile => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: FieldFeature,
                },
            ],
            Tile::Monastery => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: MonasteryFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
            ],
            Tile::MonasteryWithRoad => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: MonasteryFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: RoadFeature,
                },
            ],
            Tile::CityCapWithCrossroad => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 4,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 5,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 6,
                    feature: FieldFeature,
                },
            ],
            Tile::TriangleWithRoad => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: FieldFeature,
                },
            ],
            Tile::TriangleWithRoadWithCOA => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: FieldFeature,
                },
            ],
            Tile::Straight => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                },
            ],
            Tile::CityCap => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
            ],
            Tile::Separator => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                },
            ],
            Tile::TripleRoad => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 4,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 5,
                    feature: FieldFeature,
                },
            ],
            Tile::Curve => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                },
            ],
            Tile::QuadrupleRoad => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 4,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 5,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 6,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 7,
                    feature: FieldFeature,
                },
            ],
            Tile::Connector => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                },
            ],
            Tile::ConnectorWithCOA => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: FieldFeature,
                },
            ],
            Tile::Left => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: FieldFeature,
                },
            ],
            Tile::Right => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: FieldFeature,
                },
            ],
            Tile::TripleCity => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
            ],
            Tile::TripleCityWithCOA => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
            ],
            Tile::VerticalSeparator => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: CityFeature,
                },
            ],
            Tile::TripleCityWithRoad => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: FieldFeature,
                },
            ],
            Tile::TripleCityWithRoadWithCOA => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 2,
                    feature: RoadFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 3,
                    feature: FieldFeature,
                },
            ],
            Tile::Triangle => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
            ],
            Tile::TriangleWithCOA => vec![
                DistinctFeature {
                    id: self.feature_starting_id + 0,
                    feature: CityFeature,
                },
                DistinctFeature {
                    id: self.feature_starting_id + 1,
                    feature: FieldFeature,
                },
            ],
            Tile::QuadrupleCityWithCOA => vec![DistinctFeature {
                id: self.feature_starting_id + 0,
                feature: CityFeature,
            }],
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
        }
        Tile::Monastery => {
            mf.new_feature(t.id, 9, false);
            mf.new_feature(t.id, 4, false);
        }
        Tile::MonasteryWithRoad => {
            mf.new_feature(t.id, 9, false);
            mf.new_feature(t.id, 5, false);
            mf.new_feature(t.id, 1, false);
        }
        Tile::CityCapWithCrossroad => {
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 2, false);
        }
        Tile::TriangleWithRoad => {
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 2, false);
        }
        Tile::TriangleWithRoadWithCOA => {
            mf.new_feature(t.id, 2, true);
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 2, false);
        }
        Tile::Straight => {
            mf.new_feature(t.id, 3, false);
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 3, false);
        }
        Tile::CityCap => {
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 3, false);
        }
        Tile::Separator => {
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 2, false);
        }
        Tile::TripleRoad => {
            mf.new_feature(t.id, 3, false);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 2, false);
        }
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
        }
        Tile::Connector => {
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 1, false);
        }
        Tile::ConnectorWithCOA => {
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 2, true);
            mf.new_feature(t.id, 1, false);
        }
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
        }
        Tile::TripleCityWithCOA => {
            mf.new_feature(t.id, 3, true);
            mf.new_feature(t.id, 1, false);
        }
        Tile::VerticalSeparator => {
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 1, false);
        }
        Tile::TripleCityWithRoad => {
            mf.new_feature(t.id, 3, false);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 1, false);
        }
        Tile::TripleCityWithRoadWithCOA => {
            mf.new_feature(t.id, 3, true);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 1, false);
            mf.new_feature(t.id, 1, false);
        }
        Tile::Triangle => {
            mf.new_feature(t.id, 2, false);
            mf.new_feature(t.id, 2, false);
        }
        Tile::TriangleWithCOA => {
            mf.new_feature(t.id, 2, true);
            mf.new_feature(t.id, 2, false);
        }
        Tile::QuadrupleCityWithCOA => {
            mf.new_feature(t.id, 4, true);
        }
        Tile::Invalid => {}
    }
}

fn set_cities_to_fields(mf: &mut MergeableFeature, t: &TileItem) {
    match t.tile {
        Tile::StartingTile => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::Monastery => {}
        Tile::MonasteryWithRoad => {}
        Tile::CityCapWithCrossroad => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::TriangleWithRoad => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::TriangleWithRoadWithCOA => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::Straight => {}
        Tile::CityCap => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::Separator => {
            mf.set_cities(
                (t.feature_starting_id + 2) as usize,
                (t.feature_starting_id + 0) as usize,
            );
            mf.set_cities(
                (t.feature_starting_id + 2) as usize,
                (t.feature_starting_id + 1) as usize,
            );
        }
        Tile::TripleRoad => {}
        Tile::Curve => {}
        Tile::QuadrupleRoad => {}
        Tile::Connector => {
            mf.set_cities(
                (t.feature_starting_id + 0) as usize,
                (t.feature_starting_id + 1) as usize,
            );
            mf.set_cities(
                (t.feature_starting_id + 2) as usize,
                (t.feature_starting_id + 1) as usize,
            );
        }
        Tile::ConnectorWithCOA => {
            mf.set_cities(
                (t.feature_starting_id + 0) as usize,
                (t.feature_starting_id + 1) as usize,
            );
            mf.set_cities(
                (t.feature_starting_id + 2) as usize,
                (t.feature_starting_id + 1) as usize,
            );
        }
        Tile::Left => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::Right => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::TripleCity => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::TripleCityWithCOA => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::VerticalSeparator => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 2) as usize,
            );
        }
        Tile::TripleCityWithRoad => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
            mf.set_cities(
                (t.feature_starting_id + 3) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::TripleCityWithRoadWithCOA => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
            mf.set_cities(
                (t.feature_starting_id + 3) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::Triangle => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::TriangleWithCOA => {
            mf.set_cities(
                (t.feature_starting_id + 1) as usize,
                (t.feature_starting_id + 0) as usize,
            );
        }
        Tile::QuadrupleCityWithCOA => {}
        Tile::Invalid => {}
    }
}

fn merge_features(
    mf: &mut MergeableFeature,
    feat0: Vec<DistinctFeature>,
    feat1: Vec<DistinctFeature>,
) {
    if feat0.len() == 1 && feat1.len() == 1 {
        mf.unite(feat0[0].id as usize, feat1[0].id as usize);
    } else if feat0.len() == 3 && feat1.len() == 3 {
        mf.unite(feat0[0].id as usize, feat1[2].id as usize);
        mf.unite(feat0[1].id as usize, feat1[1].id as usize);
        mf.unite(feat0[2].id as usize, feat1[0].id as usize);
    }
}

pub fn calculate(moves: &Vec<Move>, get_final_status: bool) -> Result<Status, Error> {
    let mut mergeable_features = MergeableFeature::new();
    let mut meepleable_positions = vec![];
    let mut complete_events = vec![];
    let mut player0_point = 0;
    let mut player1_point = 0;
    let mut current_feature_id = 0;
    let mut current_tile_id = 0;
    let mut board = HashMap::<(i32, i32), TileItem>::new();
    let mut player0_remaining_meeples = HashSet::from([0, 1, 2, 3, 4, 5, 6]);
    let mut player1_remaining_meeples = HashSet::from([7, 8, 9, 10, 11, 12, 13]);
    let mut tile_id_to_pos = HashMap::<i32, (i32, i32)>::new();
    let mut meeple_id_to_pos = HashMap::<i32, (i32, i32)>::new();

    for mv in moves {
        match mv {
            Move::TMove(m) => {
                let current_tile = TileItem {
                    id: current_tile_id,
                    tile: m.tile,
                    rot: m.rot,
                    feature_starting_id: current_feature_id,
                    meeple_id: None,
                    meeple_pos: None,
                };

                create_mergeable_features(&mut mergeable_features, &current_tile);
                set_cities_to_fields(&mut mergeable_features, &current_tile);

                let y = m.pos.0;
                let x = m.pos.1;

                tile_id_to_pos.insert(current_tile_id, (y, x));

                current_feature_id += current_tile.features().len() as i32;
                current_tile_id += 1;

                // check if the placing position is empty
                if board.contains_key(&(y, x)) {
                    return Err(moves_invalid_error(format!(
                        "position ({}, {}) is not empty",
                        y, x
                    )));
                }

                // check if there is at least one adjacent tile
                if y != 0 || x != 0 {
                    let top_exists = board.contains_key(&(y - 1, x));
                    let bottom_exists = board.contains_key(&(y + 1, x));
                    let left_exists = board.contains_key(&(y, x - 1));
                    let right_exists = board.contains_key(&(y, x + 1));
                    if !top_exists && !bottom_exists && !left_exists && !right_exists {
                        return Err(moves_invalid_error(
                            "at least one adjacent tile must exist".to_string(),
                        ));
                    }
                }

                // check if the sides are correct
                match board.get(&(y - 1, x)) {
                    Some(t) => {
                        if t.bottom() != current_tile.top() {
                            return Err(moves_invalid_error(format!("top side is invalid: top tile's bottom is {:?}, but the current tile's top is {:?}", t.bottom(), current_tile.top())));
                        }
                    }
                    None => {}
                }
                match board.get(&(y + 1, x)) {
                    Some(t) => {
                        if t.top() != current_tile.bottom() {
                            return Err(moves_invalid_error(format!("bottom side is invalid: bottom tile's top is {:?}, but the current tile's bottom is {:?}", t.top(), current_tile.bottom())));
                        }
                    }
                    None => {}
                }
                match board.get(&(y, x - 1)) {
                    Some(t) => {
                        if t.right() != current_tile.left() {
                            return Err(moves_invalid_error(format!("left side is invalid: left tile's right is {:?}, but the current tile's left is {:?}", t.right(), current_tile.left())));
                        }
                    }
                    None => {}
                }
                match board.get(&(y, x + 1)) {
                    Some(t) => {
                        if t.left() != current_tile.right() {
                            return Err(moves_invalid_error(format!("right side is invalid: right tile's left is {:?}, but the current tile's right is {:?}", t.left(), current_tile.right())));
                        }
                    }
                    None => {}
                }

                // place tile
                board.insert((y, x), current_tile);

                // merge features
                match board.get(&(y - 1, x)) {
                    Some(t) => {
                        assert_eq!(t.bottom_features().len(), current_tile.top_features().len());
                        merge_features(
                            &mut mergeable_features,
                            t.bottom_features(),
                            current_tile.top_features(),
                        );
                    }
                    None => {}
                }
                match board.get(&(y + 1, x)) {
                    Some(t) => {
                        assert_eq!(t.top_features().len(), current_tile.bottom_features().len());
                        merge_features(
                            &mut mergeable_features,
                            t.top_features(),
                            current_tile.bottom_features(),
                        );
                    }
                    None => {}
                }
                match board.get(&(y, x - 1)) {
                    Some(t) => {
                        assert_eq!(t.right_features().len(), current_tile.left_features().len());
                        merge_features(
                            &mut mergeable_features,
                            t.right_features(),
                            current_tile.left_features(),
                        );
                    }
                    None => {}
                }
                match board.get(&(y, x + 1)) {
                    Some(t) => {
                        assert_eq!(t.left_features().len(), current_tile.right_features().len());
                        merge_features(
                            &mut mergeable_features,
                            t.left_features(),
                            current_tile.right_features(),
                        );
                    }
                    None => {}
                }

                // update meepleable positions
                match board.get(&(y, x)) {
                    Some(t) => {
                        meepleable_positions.clear();
                        for f in &t.features() {
                            if mergeable_features.get_meeples(f.id as usize).len() == 0 {
                                assert!(f.id >= t.feature_starting_id);
                                meepleable_positions.push(f.id - t.feature_starting_id);
                            }
                        }
                    }
                    None => {
                        assert!(false);
                    }
                }

                // update open side for monastery that was placed just now
                match m.tile {
                    Tile::Monastery | Tile::MonasteryWithRoad => {
                        let mut filled_count = 0;
                        for dy in -1..2 {
                            for dx in -1..2 {
                                filled_count += match board.get(&(y + dy, x + dx)) {
                                    None => 0,
                                    _ => 1,
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
                        let ny = y + dy;
                        let nx = x + dx;
                        match board.get(&(ny, nx)) {
                            Some(t) => match t.tile {
                                Tile::Monastery | Tile::MonasteryWithRoad => {
                                    mergeable_features
                                        .reduce_open_sides(t.feature_starting_id as usize, 1);
                                }
                                _ => {}
                            },
                            None => {}
                        }
                    }
                }
            }
            Move::MMove(m) => {
                complete_events.clear();
                let y = m.tile_pos.0;
                let x = m.tile_pos.1;
                if m.meeple_id != -1 {
                    if m.meeple_id < 7 {
                        // FIXME
                        if !player0_remaining_meeples.contains(&m.meeple_id) {
                            return Err(moves_invalid_error(format!(
                                "meeple {} is already on the board",
                                m.meeple_id
                            )));
                        }
                    } else {
                        if !player1_remaining_meeples.contains(&m.meeple_id) {
                            return Err(moves_invalid_error(format!(
                                "meeple {} is already on the board",
                                m.meeple_id
                            )));
                        }
                    }
                    match board.get(&(y, x)) {
                        Some(t) => {
                            let feature_id = t.feature_starting_id + m.meeple_pos;
                            if mergeable_features.get_meeples(feature_id as usize).len() != 0 {
                                return Err(moves_invalid_error(
                                    "meepling on this feature is not allowed".to_string(),
                                ));
                            }
                            mergeable_features.place_meeple(feature_id as usize, m.meeple_id);
                            board.insert(
                                (y, x),
                                TileItem {
                                    id: board[&(y, x)].id,
                                    tile: board[&(y, x)].tile,
                                    feature_starting_id: board[&(y, x)].feature_starting_id,
                                    rot: board[&(y, x)].rot,
                                    meeple_id: Some(m.meeple_id),
                                    meeple_pos: Some(m.meeple_pos),
                                },
                            ); // is there a better way to do this?
                            assert!(!meeple_id_to_pos.contains_key(&m.meeple_id));
                            meeple_id_to_pos.insert(m.meeple_id, (y, x));
                            if m.meeple_id < 7 {
                                // FIXME
                                player0_remaining_meeples.remove(&m.meeple_id);
                            } else {
                                player1_remaining_meeples.remove(&m.meeple_id);
                            }
                        }
                        None => {
                            assert!(false);
                        }
                    }
                }
                match board.get(&(y, x)) {
                    Some(t) => {
                        for f in &t.features() {
                            if mergeable_features.is_done(f.id as usize) {
                                continue;
                            }
                            if f.feature == FieldFeature {
                                continue;
                            }
                            if mergeable_features.is_completed(f.id as usize) {
                                let sz = mergeable_features.size(f.id as usize);
                                let meeple_ids = mergeable_features.get_meeples(f.id as usize);
                                if meeple_ids.len() == 0 {
                                    continue;
                                }
                                let pts = match f.feature {
                                    RoadFeature => (sz * 1) as i32,
                                    CityFeature => (sz * 2) as i32,
                                    MonasteryFeature => 9,
                                    FieldFeature => 0,
                                };
                                let mut player0_meeples = 0;
                                let mut player1_meeples = 0;
                                for meeple_id in &meeple_ids {
                                    if *meeple_id < 7 {
                                        player0_meeples += 1;
                                        player0_remaining_meeples.insert(*meeple_id);
                                    } else {
                                        player1_meeples += 1;
                                        player1_remaining_meeples.insert(*meeple_id);
                                    }
                                    assert!(meeple_id_to_pos.contains_key(meeple_id));
                                    let pos = meeple_id_to_pos.get(meeple_id).unwrap();
                                    assert!(board.contains_key(&pos));
                                    board.insert(
                                        *pos,
                                        TileItem {
                                            id: board[pos].id,
                                            tile: board[pos].tile,
                                            feature_starting_id: board[pos].feature_starting_id,
                                            rot: board[pos].rot,
                                            meeple_id: None,
                                            meeple_pos: None,
                                        },
                                    );
                                    meeple_id_to_pos.remove(&meeple_id);
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
                    }
                    None => {
                        assert!(false);
                    }
                }

                // resolve meeples on adjacent monasteries
                for dy in -1..2 {
                    for dx in -1..2 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let ny = y + dy;
                        let nx = x + dx;
                        match board.get(&(ny, nx)) {
                            Some(t) => match t.tile {
                                Tile::Monastery | Tile::MonasteryWithRoad => {
                                    if mergeable_features
                                        .is_completed(t.feature_starting_id as usize)
                                    {
                                        let meeple_ids = mergeable_features
                                            .get_meeples(t.feature_starting_id as usize);
                                        if meeple_ids.len() == 0 {
                                            continue;
                                        }
                                        assert!(meeple_ids.len() == 1);
                                        if meeple_ids[0] < 7 {
                                            player0_point += 9;
                                            player0_remaining_meeples.insert(meeple_ids[0]);
                                        } else {
                                            player1_point += 9;
                                            player1_remaining_meeples.insert(meeple_ids[0]);
                                        }
                                        assert!(meeple_id_to_pos.contains_key(&meeple_ids[0]));
                                        let pos = meeple_id_to_pos.get(&meeple_ids[0]).unwrap();
                                        assert!(board.contains_key(&pos));
                                        board.insert(
                                            *pos,
                                            TileItem {
                                                id: board[pos].id,
                                                tile: board[pos].tile,
                                                feature_starting_id: board[pos].feature_starting_id,
                                                rot: board[pos].rot,
                                                meeple_id: None,
                                                meeple_pos: None,
                                            },
                                        );
                                        meeple_id_to_pos.remove(&meeple_ids[0]);
                                        complete_events.push(CompleteEvent {
                                            feature: MonasteryFeature,
                                            meeple_ids,
                                            point: 9,
                                        });
                                    }
                                }
                                _ => {}
                            },
                            None => {}
                        }
                    }
                }
            }
            Move::DMove(_m) => {
                // do nothing
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
            board,
            player0_remaining_meeples,
            player1_remaining_meeples,
            tile_id_to_pos,
            mergeable_features,
            feature_num: current_feature_id,
        });
    }

    let mut complete_events = vec![];

    for t in board.values() {
        let fs = t.features();
        for f in &fs {
            let meeple_ids = mergeable_features.get_meeples(f.id as usize);
            if meeple_ids.len() == 0 {
                continue;
            }

            if f.feature != FieldFeature && mergeable_features.is_completed(f.id as usize) {
                continue;
            }
            if mergeable_features.is_done(f.id as usize) {
                continue;
            }

            let pts = match f.feature {
                RoadFeature => {
                    let sz = mergeable_features.size(f.id as usize);
                    sz as i32
                }
                CityFeature => {
                    let sz = mergeable_features.size(f.id as usize);
                    sz as i32
                }
                MonasteryFeature => {
                    let open_sides = mergeable_features.get_open_sides(f.id as usize);
                    (9 - open_sides) as i32
                }
                FieldFeature => {
                    let mut p = 0;
                    let cities = mergeable_features.get_facing_cities(f.id as usize);
                    for city in &cities {
                        if mergeable_features.is_completed(*city) {
                            p += 3;
                        }
                    }
                    p
                }
            };
            let mut player0_meeples = 0;
            let mut player1_meeples = 0;
            for meeple_id in &meeple_ids {
                if *meeple_id < 7 {
                    player0_meeples += 1;
                    player0_remaining_meeples.insert(*meeple_id);
                } else {
                    player1_meeples += 1;
                    player1_remaining_meeples.insert(*meeple_id);
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

    for pos in meeple_id_to_pos.values() {
        assert!(board.contains_key(pos));
        board.insert(
            *pos,
            TileItem {
                id: board[pos].id,
                tile: board[pos].tile,
                feature_starting_id: board[pos].feature_starting_id,
                rot: board[pos].rot,
                meeple_id: None,
                meeple_pos: None,
            },
        );
    }

    assert_eq!(player0_remaining_meeples.len(), 7);
    assert_eq!(player1_remaining_meeples.len(), 7);

    Ok(Status {
        meepleable_positions,
        complete_events,
        player0_point,
        player1_point,
        board,
        player0_remaining_meeples,
        player1_remaining_meeples,
        tile_id_to_pos,
        mergeable_features,
        feature_num: current_feature_id,
    })
}

pub fn calculate_tileable_positions(moves: &Vec<Move>, t: Tile) -> Vec<TileablePosition> {
    let board = match calculate(&moves, false) {
        Ok(s) => s.board,
        Err(e) => panic!("{:?}", e.detail.msg),
    };

    let mut tile = TileItem {
        id: t.to_id(),
        tile: t,
        rot: 0,
        feature_starting_id: 0,
        meeple_id: None,
        meeple_pos: None,
    };

    let mut checked: HashMap<(i32, i32), bool> = HashMap::new();
    let mut tileable_positions = vec![];
    for (y, x) in board.keys() {
        match checked.get(&(*y, *x)) {
            Some(_) => {
                continue;
            }
            None => {}
        }
        checked.insert((*y, *x), true);

        let dy = [0, -1, 0, 1];
        let dx = [1, 0, -1, 0];
        for di in 0..4 {
            let ny = y + dy[di];
            let nx = x + dx[di];
            match board.get(&(ny, nx)) {
                Some(_) => {
                    continue;
                }
                None => {}
            }
            match checked.get(&(ny, nx)) {
                Some(_) => {
                    continue;
                }
                None => {}
            }
            checked.insert((ny, nx), true);

            for rot in vec![1, 2, 3, 4] {
                tile.rotate();

                match board.get(&(ny - 1, nx)) {
                    Some(t) => {
                        if t.bottom() != tile.top() {
                            continue;
                        }
                    }
                    None => {}
                }
                match board.get(&(ny + 1, nx)) {
                    Some(t) => {
                        if t.top() != tile.bottom() {
                            continue;
                        }
                    }
                    None => {}
                }
                match board.get(&(ny, nx - 1)) {
                    Some(t) => {
                        if t.right() != tile.left() {
                            continue;
                        }
                    }
                    None => {}
                }
                match board.get(&(ny, nx + 1)) {
                    Some(t) => {
                        if t.left() != tile.right() {
                            continue;
                        }
                    }
                    None => {}
                }

                tileable_positions.push(TileablePosition { pos: (ny, nx), rot })
            }
        }
    }

    tileable_positions
}

#[test]
fn calculate_test_for_road_and_city_completion() {
    let game_id = 0;
    let player0_id = 0;
    let player1_id = 1;
    let mut mvs = vec![
        Move::TMove(TileMove {
            ord: 0,
            game_id,
            player_id: player1_id,
            tile: Tile::StartingTile,
            rot: 0,
            pos: (0, 0),
        }),
        Move::MMove(MeepleMove {
            ord: 1,
            game_id,
            player_id: player1_id,
            meeple_id: -1,
            tile_pos: (0, 0),
            meeple_pos: -1,
        }),
    ];

    mvs.push(Move::TMove(TileMove {
        ord: 2,
        game_id,
        player_id: player0_id,
        tile: Tile::TriangleWithRoad,
        rot: 2,
        pos: (-1, 0),
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.meepleable_positions, vec![0, 1, 2, 3]);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::MMove(MeepleMove {
        ord: 3,
        game_id,
        player_id: player0_id,
        meeple_id: 0,
        tile_pos: (-1, 0),
        meeple_pos: 0,
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events.len(), 0);
            assert_eq!(res.player0_point, 0);
            assert_eq!(res.player1_point, 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::TMove(TileMove {
        ord: 4,
        game_id,
        player_id: player1_id,
        tile: Tile::CityCapWithCrossroad,
        rot: 3,
        pos: (0, -1),
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.meepleable_positions, vec![0, 1, 2, 3, 4, 5, 6]);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::MMove(MeepleMove {
        ord: 5,
        game_id,
        player_id: player1_id,
        meeple_id: 7,
        tile_pos: (0, -1),
        meeple_pos: 0,
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events.len(), 0);
            assert_eq!(res.player0_point, 0);
            assert_eq!(res.player1_point, 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::TMove(TileMove {
        ord: 6,
        game_id,
        player_id: player0_id,
        tile: Tile::CityCapWithCrossroad,
        rot: 0,
        pos: (0, 1),
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.meepleable_positions, vec![0, 1, 2, 3, 4, 5, 6]);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::MMove(MeepleMove {
        ord: 7,
        game_id,
        player_id: player0_id,
        meeple_id: 1,
        tile_pos: (0, 1),
        meeple_pos: 2,
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events.len(), 1);
            assert_eq!(res.complete_events[0].feature, RoadFeature);
            assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
            assert_eq!(res.complete_events[0].point, 3);
            assert_eq!(res.player0_point, 3);
            assert_eq!(res.player1_point, 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::TMove(TileMove {
        ord: 8,
        game_id,
        player_id: player1_id,
        tile: Tile::StartingTile,
        rot: 1,
        pos: (0, -2),
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.meepleable_positions, vec![1, 2, 3]);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::MMove(MeepleMove {
        ord: 9,
        game_id,
        player_id: player1_id,
        meeple_id: 8,
        tile_pos: (0, -2),
        meeple_pos: 2,
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events.len(), 1);
            assert_eq!(res.complete_events[0].feature, CityFeature);
            assert_eq!(res.complete_events[0].meeple_ids, vec![7]);
            assert_eq!(res.complete_events[0].point, 4);
            assert_eq!(res.player0_point, 3);
            assert_eq!(res.player1_point, 4);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::TMove(TileMove {
        ord: 10,
        game_id,
        player_id: player0_id,
        tile: Tile::TriangleWithRoadWithCOA,
        rot: 3,
        pos: (-1, 1),
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.meepleable_positions, vec![1, 2, 3]);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::MMove(MeepleMove {
        ord: 11,
        game_id,
        player_id: player0_id,
        meeple_id: 1,
        tile_pos: (-1, 1),
        meeple_pos: 1,
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events.len(), 1);
            assert_eq!(res.complete_events[0].feature, CityFeature);
            assert_eq!(res.complete_events[0].meeple_ids, vec![0]);
            assert_eq!(res.complete_events[0].point, 10);
            assert_eq!(res.player0_point, 13);
            assert_eq!(res.player1_point, 4);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    let status = calculate(&mvs, true);
    match status {
        Ok(res) => {
            let mut events = vec![];
            for e in res.complete_events {
                events.push(e);
            }
            events.sort();
            assert_eq!(events.len(), 2);
            assert_eq!(events[0].feature, RoadFeature);
            assert_eq!(events[0].meeple_ids, vec![8]);
            assert_eq!(events[0].point, 1);
            assert_eq!(events[1].feature, FieldFeature);
            assert_eq!(events[1].meeple_ids, vec![1]);
            assert_eq!(events[1].point, 3);
            assert_eq!(res.player0_point, 16);
            assert_eq!(res.player1_point, 5);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
}

#[test]
fn calculate_test_for_monastery_completion() {
    let game_id = 0;
    let player0_id = 0;
    let player1_id = 1;
    let mut mvs = vec![
        Move::TMove(TileMove {
            ord: 0,
            game_id,
            player_id: player1_id,
            tile: Tile::StartingTile,
            rot: 0,
            pos: (0, 0),
        }),
        Move::MMove(MeepleMove {
            ord: 1,
            game_id,
            player_id: player1_id,
            meeple_id: -1,
            tile_pos: (0, 0),
            meeple_pos: -1,
        }),
    ];

    mvs.push(Move::TMove(TileMove {
        ord: 2,
        game_id,
        player_id: player0_id,
        tile: Tile::Monastery,
        rot: 0,
        pos: (1, 0),
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.meepleable_positions, vec![0, 1]);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::MMove(MeepleMove {
        ord: 3,
        game_id,
        player_id: player0_id,
        meeple_id: 0,
        tile_pos: (1, 0),
        meeple_pos: 0,
    }));
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events.len(), 0);
            assert_eq!(res.player0_point, 0);
            assert_eq!(res.player1_point, 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    mvs.push(Move::TMove(TileMove {
        ord: 4,
        game_id,
        player_id: player1_id,
        tile: Tile::Monastery,
        rot: 0,
        pos: (2, 0),
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: 5,
        game_id,
        player_id: player1_id,
        meeple_id: -1,
        tile_pos: (2, 0),
        meeple_pos: -1,
    }));

    mvs.push(Move::TMove(TileMove {
        ord: 6,
        game_id,
        player_id: player0_id,
        tile: Tile::Monastery,
        rot: 0,
        pos: (2, -1),
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: 7,
        game_id,
        player_id: player0_id,
        meeple_id: -1,
        tile_pos: (2, -1),
        meeple_pos: -1,
    }));

    mvs.push(Move::TMove(TileMove {
        ord: 8,
        game_id,
        player_id: player1_id,
        tile: Tile::Monastery,
        rot: 0,
        pos: (2, -2),
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: 9,
        game_id,
        player_id: player1_id,
        meeple_id: -1,
        tile_pos: (2, -2),
        meeple_pos: -1,
    }));

    mvs.push(Move::TMove(TileMove {
        ord: 10,
        game_id,
        player_id: player0_id,
        tile: Tile::Monastery,
        rot: 0,
        pos: (1, -2),
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: 11,
        game_id,
        player_id: player0_id,
        meeple_id: -1,
        tile_pos: (1, -2),
        meeple_pos: -1,
    }));

    mvs.push(Move::TMove(TileMove {
        ord: 12,
        game_id,
        player_id: player0_id,
        tile: Tile::Monastery,
        rot: 0,
        pos: (2, 1),
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: 13,
        game_id,
        player_id: player0_id,
        meeple_id: -1,
        tile_pos: (2, 1),
        meeple_pos: -1,
    }));

    mvs.push(Move::TMove(TileMove {
        ord: 14,
        game_id,
        player_id: player1_id,
        tile: Tile::Monastery,
        rot: 0,
        pos: (1, 1),
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: 15,
        game_id,
        player_id: player1_id,
        meeple_id: -1,
        tile_pos: (1, 1),
        meeple_pos: -1,
    }));

    mvs.push(Move::TMove(TileMove {
        ord: 16,
        game_id,
        player_id: player0_id,
        tile: Tile::StartingTile,
        rot: 0,
        pos: (0, 1),
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: 17,
        game_id,
        player_id: player0_id,
        meeple_id: -1,
        tile_pos: (0, 1),
        meeple_pos: -1,
    }));

    mvs.push(Move::TMove(TileMove {
        ord: 18,
        game_id,
        player_id: player1_id,
        tile: Tile::StartingTile,
        rot: 0,
        pos: (0, -1),
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: 19,
        game_id,
        player_id: player1_id,
        meeple_id: -1,
        tile_pos: (0, -1),
        meeple_pos: -1,
    }));

    mvs.push(Move::TMove(TileMove {
        ord: 20,
        game_id,
        player_id: player0_id,
        tile: Tile::StartingTile,
        rot: 0,
        pos: (0, -2),
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: 21,
        game_id,
        player_id: player0_id,
        meeple_id: -1,
        tile_pos: (0, -2),
        meeple_pos: -1,
    }));

    mvs.push(Move::TMove(TileMove {
        ord: 22,
        game_id,
        player_id: player1_id,
        tile: Tile::Monastery,
        rot: 0,
        pos: (1, -1),
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: 23,
        game_id,
        player_id: player1_id,
        meeple_id: 7,
        tile_pos: (1, -1),
        meeple_pos: 0,
    }));

    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            let mut events = vec![];
            for e in res.complete_events {
                events.push(e);
            }
            events.sort();
            assert_eq!(events.len(), 2);
            assert_eq!(events[0].feature, MonasteryFeature);
            assert_eq!(events[0].meeple_ids, vec![0]);
            assert_eq!(events[0].point, 9);
            assert_eq!(events[1].feature, MonasteryFeature);
            assert_eq!(events[1].meeple_ids, vec![7]);
            assert_eq!(events[1].point, 9);
            assert_eq!(res.player0_point, 9);
            assert_eq!(res.player1_point, 9);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
}

#[allow(dead_code)]
fn add_move(
    mvs: &mut Vec<Move>,
    tile: Tile,
    rot: i32,
    pos: (i32, i32),
    meeple_id: i32,
    meeple_pos: i32,
) {
    mvs.push(Move::TMove(TileMove {
        ord: -1,
        game_id: -1,
        player_id: -1,
        tile,
        rot,
        pos,
    }));
    mvs.push(Move::MMove(MeepleMove {
        ord: -1,
        game_id: -1,
        player_id: -1,
        meeple_id: meeple_id,
        tile_pos: pos,
        meeple_pos,
    }));
}

#[test]
fn calculate_test0() {
    /* actual game here: https://boardgamearena.com/table?table=361472535 */
    let mut mvs = vec![];

    add_move(&mut mvs, Tile::StartingTile, 0, (0, 0), -1, -1);
    add_move(&mut mvs, Tile::Straight, 1, (0, 1), 0, 1);
    add_move(&mut mvs, Tile::CityCap, 2, (-1, 0), 7, 0);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::Separator, 2, (1, 0), 1, 0);
    add_move(&mut mvs, Tile::StartingTile, 3, (1, -1), 7, 0);
    add_move(&mut mvs, Tile::TripleRoad, 0, (0, -1), 2, 4);
    add_move(&mut mvs, Tile::CityCap, 3, (1, 1), 8, 0);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::Curve, 1, (0, 2), -1, -1);
    add_move(&mut mvs, Tile::TripleRoad, 0, (2, 1), 8, 1);
    add_move(&mut mvs, Tile::QuadrupleRoad, 0, (0, -2), 3, 4);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::TripleRoad, 2, (3, 1), 9, 4);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::CityCapWithCrossroad, 0, (-1, 2), 3, 0);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::Straight, 1, (-1, 3), 9, 1);
    add_move(&mut mvs, Tile::Curve, 0, (2, 2), 10, 1);
    add_move(&mut mvs, Tile::ConnectorWithCOA, 1, (-2, 2), -1, -1);
    add_move(&mut mvs, Tile::Straight, 0, (2, -1), -1, -1);
    add_move(&mut mvs, Tile::Left, 1, (1, -2), 11, 2);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::CityCap, 2, (-3, 2), -1, -1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::CityCap, 0, (-4, 2), 7, 0);
    add_move(&mut mvs, Tile::TripleCity, 2, (-5, 2), -1, -1);
    add_move(&mut mvs, Tile::TriangleWithRoad, 2, (3, 2), 12, 0);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::TripleCity, 1, (-4, 3), -1, -1);
    add_move(&mut mvs, Tile::Curve, 3, (0, -3), -1, -1);
    add_move(&mut mvs, Tile::CityCapWithCrossroad, 2, (3, -1), 0, 0);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::TripleCity, 2, (-5, 3), -1, -1);
    add_move(&mut mvs, Tile::StartingTile, 0, (4, -1), -1, -1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::VerticalSeparator, 1, (-5, 1), 10, 2);
    add_move(&mut mvs, Tile::Right, 0, (2, 0), 0, 1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::TripleCityWithCOA, 3, (-4, 4), -1, -1);
    add_move(&mut mvs, Tile::VerticalSeparator, 0, (-4, 5), 1, 2);
    add_move(&mut mvs, Tile::Left, 2, (-5, 5), 13, 0);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::TripleCityWithRoad, 3, (-3, 5), -1, -1);
    add_move(&mut mvs, Tile::Monastery, 0, (1, 2), 13, 0);
    add_move(&mut mvs, Tile::MonasteryWithRoad, 3, (-1, 1), 2, 0);
    add_move(&mut mvs, Tile::TripleRoad, 2, (3, 0), -1, -1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::Right, 0, (-6, 5), 3, 0);
    add_move(&mut mvs, Tile::TriangleWithRoad, 1, (4, 2), -1, -1);
    add_move(&mut mvs, Tile::Straight, 0, (0, -4), -1, -1);
    add_move(&mut mvs, Tile::TriangleWithRoad, 3, (1, -3), -1, -1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::Monastery, 0, (-2, 1), 4, 0);
    add_move(&mut mvs, Tile::TriangleWithCOA, 3, (3, 3), 8, 1);
    add_move(
        &mut mvs,
        Tile::TripleCityWithRoadWithCOA,
        1,
        (-3, 4),
        -1,
        -1,
    );
    add_move(&mut mvs, Tile::Curve, 2, (2, 3), -1, -1);
    add_move(&mut mvs, Tile::Straight, 1, (4, -2), 5, 0);
    add_move(&mut mvs, Tile::Straight, 0, (1, 3), -1, -1);
    add_move(&mut mvs, Tile::MonasteryWithRoad, 2, (-2, 0), 6, 0);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::Connector, 0, (-5, 0), -1, -1);
    add_move(&mut mvs, Tile::Straight, 1, (3, -2), -1, -1);
    add_move(&mut mvs, Tile::Curve, 3, (0, 3), -1, -1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::TriangleWithRoadWithCOA, 1, (-2, 4), -1, -1);
    add_move(&mut mvs, Tile::Straight, 0, (1, 4), 11, 2);
    add_move(&mut mvs, Tile::Curve, 1, (-3, 1), -1, -1);
    add_move(&mut mvs, Tile::Right, 2, (2, 4), 13, 2);
    add_move(&mut mvs, Tile::StartingTile, 3, (5, 2), -1, -1);
    add_move(&mut mvs, Tile::Curve, 0, (0, 4), -1, -1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::Monastery, 0, (5, 3), 2, 1);
    add_move(&mut mvs, Tile::TriangleWithRoad, 1, (5, 1), -1, -1);
    add_move(&mut mvs, Tile::Left, 2, (-7, 5), -1, -1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::Monastery, 0, (2, -2), 13, 0);
    add_move(&mut mvs, Tile::TriangleWithCOA, 1, (3, 4), -1, -1);
    add_move(&mut mvs, Tile::CityCap, 1, (-5, -1), -1, -1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::QuadrupleCityWithCOA, 0, (-2, 5), -1, -1);
    add_move(&mut mvs, Tile::ConnectorWithCOA, 0, (3, 5), 10, 1);
    add_move(&mut mvs, Tile::CityCapWithCrossroad, 3, (-3, 0), 3, 2);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::VerticalSeparator, 1, (3, 6), -1, -1);

    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events.len(), 1);
            assert_eq!(res.complete_events[0].feature, CityFeature);
            assert_eq!(res.complete_events[0].meeple_ids, vec![10]);
            assert_eq!(res.complete_events[0].point, 12);
            assert_eq!(res.player0_point, 51);
            assert_eq!(res.player1_point, 62);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::Curve, 0, (-4, 1), 3, 0);
    add_move(&mut mvs, Tile::Triangle, 2, (4, 5), 10, 1);
    add_move(&mut mvs, Tile::Triangle, 2, (5, 4), -1, -1);
    add_move(&mut mvs, Tile::Curve, 1, (-1, 4), -1, -1);
    add_move(&mut mvs, Tile::Separator, 2, (-3, -1), 4, 1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::Triangle, 0, (4, 3), -1, -1);

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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    add_move(&mut mvs, Tile::TripleCityWithRoadWithCOA, 1, (-6, 6), 4, 1);

    let status = calculate(&mvs, true);
    match status {
        Ok(res) => {
            assert_eq!(res.player0_point, 113);
            assert_eq!(res.player1_point, 116);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
}

#[test]
fn calculate_test1() {
    /* actual game here: https://boardgamearena.com/table?table=361578832 */
    let mut mvs = vec![];
    add_move(&mut mvs, Tile::StartingTile, 0, (0, 0), -1, -1);
    add_move(&mut mvs, Tile::Curve, 1, (0, 1), 0, 1);
    add_move(&mut mvs, Tile::TriangleWithRoad, 3, (-1, 0), 7, 0);
    add_move(&mut mvs, Tile::TripleRoad, 1, (-1, 1), 1, 4);
    add_move(&mut mvs, Tile::TripleCityWithRoad, 0, (-2, 1), 8, 2);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![8]);
            assert_eq!(res.complete_events[0].point, 2);
            assert_eq!(res.player0_point, 0);
            assert_eq!(res.player1_point, 2);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Triangle, 0, (-2, 2), 2, 0);
    add_move(&mut mvs, Tile::TripleCityWithCOA, 0, (-1, -1), -1, -1);
    add_move(&mut mvs, Tile::CityCap, 1, (0, 2), 3, 0);
    add_move(&mut mvs, Tile::TripleRoad, 3, (-2, 3), 8, 2);
    add_move(&mut mvs, Tile::ConnectorWithCOA, 0, (0, 3), -1, -1);
    add_move(&mut mvs, Tile::TripleCityWithRoadWithCOA, 0, (-3, 3), 9, 0);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![8]);
            assert_eq!(res.complete_events[0].point, 2);
            assert_eq!(res.player0_point, 0);
            assert_eq!(res.player1_point, 4);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Curve, 3, (0, -1), -1, -1);
    add_move(&mut mvs, Tile::CityCapWithCrossroad, 0, (-2, 4), 8, 2);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![8]);
            assert_eq!(res.complete_events[0].point, 2);
            assert_eq!(res.player0_point, 0);
            assert_eq!(res.player1_point, 6);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Right, 3, (0, 4), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![3]);
            assert_eq!(res.complete_events[0].point, 8);
            assert_eq!(res.player0_point, 8);
            assert_eq!(res.player1_point, 6);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::TripleRoad, 0, (1, 0), 8, 1);
    add_move(&mut mvs, Tile::Connector, 1, (0, -2), 3, 1);
    add_move(&mut mvs, Tile::QuadrupleCityWithCOA, 0, (-2, -1), -1, -1);
    add_move(&mut mvs, Tile::Triangle, 2, (-1, -2), -1, -1);
    add_move(&mut mvs, Tile::TripleCity, 2, (-3, -1), -1, -1);
    add_move(
        &mut mvs,
        Tile::TripleCityWithRoadWithCOA,
        0,
        (-2, 0),
        -1,
        -1,
    );
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
            assert_eq!(res.complete_events[0].point, 3);
            assert_eq!(res.player0_point, 11);
            assert_eq!(res.player1_point, 6);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Straight, 0, (-1, 4), 10, 1);
    add_move(&mut mvs, Tile::VerticalSeparator, 0, (1, -2), 1, 2);
    add_move(&mut mvs, Tile::TriangleWithCOA, 2, (-3, 2), -1, -1);
    add_move(&mut mvs, Tile::TriangleWithCOA, 1, (-4, 2), 4, 0);
    add_move(&mut mvs, Tile::Curve, 2, (1, -1), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![0, 8]);
            assert_eq!(res.complete_events[0].point, 6);
            assert_eq!(res.player0_point, 17);
            assert_eq!(res.player1_point, 12);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::StartingTile, 0, (2, -2), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
            assert_eq!(res.complete_events[0].point, 4);
            assert_eq!(res.player0_point, 21);
            assert_eq!(res.player1_point, 12);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Monastery, 0, (-1, 2), 8, 0);
    add_move(&mut mvs, Tile::MonasteryWithRoad, 2, (-1, 3), 0, 0);
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
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::TripleCity, 3, (-3, 4), -1, -1);
    add_move(&mut mvs, Tile::Straight, 0, (-3, 5), -1, -1);
    add_move(&mut mvs, Tile::Straight, 0, (-4, 1), -1, -1);
    add_move(&mut mvs, Tile::Curve, 0, (-4, 5), -1, -1);
    add_move(&mut mvs, Tile::Left, 2, (-4, 4), -1, -1);
    add_move(&mut mvs, Tile::CityCap, 2, (3, -2), 0, 0);
    add_move(&mut mvs, Tile::Curve, 3, (-5, 4), 8, 1);
    add_move(&mut mvs, Tile::Triangle, 3, (-4, 3), -1, -1);
    add_move(&mut mvs, Tile::TriangleWithRoadWithCOA, 1, (-5, 1), 11, 0);
    add_move(&mut mvs, Tile::MonasteryWithRoad, 2, (-5, 3), 5, 0);
    add_move(&mut mvs, Tile::TripleRoad, 1, (0, 5), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![10]);
            assert_eq!(res.complete_events[0].point, 4);
            assert_eq!(res.player0_point, 30);
            assert_eq!(res.player1_point, 25);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Separator, 1, (4, -2), 1, 0);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![0]);
            assert_eq!(res.complete_events[0].point, 4);
            assert_eq!(res.player0_point, 34);
            assert_eq!(res.player1_point, 25);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Right, 3, (2, -3), 12, 0);
    add_move(&mut mvs, Tile::Left, 3, (4, -1), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
            assert_eq!(res.complete_events[0].point, 4);
            assert_eq!(res.player0_point, 38);
            assert_eq!(res.player1_point, 25);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Straight, 1, (-5, 5), -1, -1);
    add_move(&mut mvs, Tile::Monastery, 0, (3, -3), 0, 0);
    add_move(&mut mvs, Tile::TriangleWithRoadWithCOA, 2, (-2, 5), -1, -1);
    add_move(&mut mvs, Tile::TriangleWithRoad, 0, (-6, 3), 1, 0);
    add_move(&mut mvs, Tile::Straight, 1, (-5, 6), -1, -1);
    add_move(&mut mvs, Tile::Curve, 1, (-6, 4), -1, -1);
    add_move(&mut mvs, Tile::QuadrupleRoad, 1, (-5, 7), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![8]);
            assert_eq!(res.complete_events[0].point, 9);
            assert_eq!(res.player0_point, 38);
            assert_eq!(res.player1_point, 34);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::CityCap, 1, (-6, 2), -1, -1);
    add_move(&mut mvs, Tile::Straight, 0, (-7, 4), 8, 1);
    add_move(&mut mvs, Tile::CityCapWithCrossroad, 2, (1, 5), 6, 0);
    add_move(&mut mvs, Tile::Curve, 2, (-7, 2), -1, -1);
    add_move(&mut mvs, Tile::CityCap, 0, (2, 5), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![6]);
            assert_eq!(res.complete_events[0].point, 4);
            assert_eq!(res.player0_point, 42);
            assert_eq!(res.player1_point, 34);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::TriangleWithRoad, 0, (-8, 4), -1, -1);
    add_move(&mut mvs, Tile::CityCapWithCrossroad, 0, (-8, 2), 6, 0);
    add_move(&mut mvs, Tile::VerticalSeparator, 2, (-4, 0), 10, 0);
    add_move(&mut mvs, Tile::Curve, 0, (-7, 1), -1, -1);
    add_move(&mut mvs, Tile::ConnectorWithCOA, 0, (2, -4), -1, -1);
    add_move(&mut mvs, Tile::StartingTile, 2, (-9, 2), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![6]);
            assert_eq!(res.complete_events[0].point, 4);
            assert_eq!(res.player0_point, 46);
            assert_eq!(res.player1_point, 34);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Monastery, 0, (-6, 5), -1, -1);
    add_move(&mut mvs, Tile::Curve, 0, (4, -3), -1, -1);
    add_move(&mut mvs, Tile::Straight, 1, (-8, 5), -1, -1);
    add_move(&mut mvs, Tile::VerticalSeparator, 1, (-3, -2), 6, 2);
    add_move(&mut mvs, Tile::StartingTile, 1, (2, -5), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![12]);
            assert_eq!(res.complete_events[0].point, 8);
            assert_eq!(res.player0_point, 46);
            assert_eq!(res.player1_point, 42);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Straight, 1, (4, -4), -1, -1);
    add_move(&mut mvs, Tile::TripleCity, 3, (-3, 0), -1, -1);
    add_move(&mut mvs, Tile::CityCap, 1, (-3, -3), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![6]);
            assert_eq!(res.complete_events[0].point, 4);
            assert_eq!(res.player0_point, 50);
            assert_eq!(res.player1_point, 42);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Left, 3, (5, -2), 12, 1);
    add_move(&mut mvs, Tile::Right, 2, (-7, 3), 6, 1);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![1]);
            assert_eq!(res.complete_events[0].point, 6);
            assert_eq!(res.player0_point, 56);
            assert_eq!(res.player1_point, 42);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    add_move(&mut mvs, Tile::Separator, 2, (1, 3), 13, 2);
    add_move(&mut mvs, Tile::Monastery, 0, (3, -4), 1, 0);
    let status = calculate(&mvs, false);
    match status {
        Ok(res) => {
            assert_eq!(res.complete_events[0].meeple_ids, vec![0]);
            assert_eq!(res.complete_events[0].point, 9);
            assert_eq!(res.player0_point, 65);
            assert_eq!(res.player1_point, 42);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    let status = calculate(&mvs, true);
    match status {
        Ok(res) => {
            let mut events = vec![];
            for e in res.complete_events {
                events.push(e);
            }
            events.sort();
            assert_eq!(events.len(), 8);
            assert_eq!(events[0].feature, RoadFeature);
            assert_eq!(events[0].meeple_ids, [8]);
            assert_eq!(events[0].point, 6);
            assert_eq!(events[1].feature, CityFeature);
            assert_eq!(events[1].meeple_ids, [11]);
            assert_eq!(events[1].point, 2);
            assert_eq!(events[2].feature, CityFeature);
            assert_eq!(events[2].meeple_ids, [7, 3, 2, 9, 4, 10]);
            assert_eq!(events[2].point, 27);
            assert_eq!(events[3].feature, MonasteryFeature);
            assert_eq!(events[3].meeple_ids, [1]);
            assert_eq!(events[3].point, 7);
            assert_eq!(events[4].feature, MonasteryFeature);
            assert_eq!(events[4].meeple_ids, [5]);
            assert_eq!(events[4].point, 8);
            assert_eq!(events[5].feature, FieldFeature);
            assert_eq!(events[5].meeple_ids, [6]);
            assert_eq!(events[5].point, 3);
            assert_eq!(events[6].feature, FieldFeature);
            assert_eq!(events[6].meeple_ids, [13]);
            assert_eq!(events[6].point, 3);
            assert_eq!(events[7].feature, FieldFeature);
            assert_eq!(events[7].meeple_ids, [12]);
            assert_eq!(events[7].point, 9);
            assert_eq!(res.player0_point, 110);
            assert_eq!(res.player1_point, 89);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }

    // check if calculate works fairly fast (probably not fast enough yet)
    for _i in 0..1000 {
        match calculate(&mut mvs, false) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

#[test]
fn calculate_test_for_field() {
    // this calculation for the field also fails (red's field on the top is scored as 6, but it must be 3)
    // let mut mvs = decoder::decode("src/data/388947581.json".to_string());

    // actual game here: https://boardgamearena.com/table?table=367130620
    let mut mvs = vec![];
    add_move(&mut mvs, Tile::StartingTile, 0, (0, 0), -1, -1);
    add_move(&mut mvs, Tile::TripleRoad, 1, (0, 1), 0, 4);
    add_move(&mut mvs, Tile::Straight, 0, (-1, 1), 7, 1);
    add_move(&mut mvs, Tile::CityCap, 2, (-1, 0), 1, 0);
    add_move(&mut mvs, Tile::Triangle, 0, (-1, -1), 8, 0);
    add_move(&mut mvs, Tile::Separator, 1, (-1, 2), 1, 0);
    add_move(&mut mvs, Tile::TripleRoad, 3, (-2, 1), -1, -1);
    add_move(&mut mvs, Tile::Separator, 0, (-1, 3), 2, 0);
    add_move(&mut mvs, Tile::Monastery, 0, (0, 2), 7, 0);
    add_move(&mut mvs, Tile::StartingTile, 2, (-2, 2), 1, 0);
    add_move(&mut mvs, Tile::Straight, 0, (1, 1), 9, 2);
    add_move(&mut mvs, Tile::VerticalSeparator, 0, (-1, 4), 1, 0);
    add_move(&mut mvs, Tile::CityCapWithCrossroad, 0, (0, 4), 10, 0);
    add_move(&mut mvs, Tile::TriangleWithRoad, 2, (-2, 3), 3, 2);
    add_move(&mut mvs, Tile::TripleCity, 3, (-2, -1), -1, -1);
    add_move(&mut mvs, Tile::Left, 3, (0, -1), 4, 0);
    add_move(&mut mvs, Tile::Curve, 3, (0, 3), -1, -1);
    add_move(&mut mvs, Tile::Left, 1, (0, -2), -1, -1);
    add_move(&mut mvs, Tile::Curve, 2, (-1, 5), -1, -1);
    add_move(&mut mvs, Tile::CityCap, 2, (1, -2), 4, 0);
    add_move(&mut mvs, Tile::StartingTile, 1, (-2, 5), -1, -1);
    add_move(&mut mvs, Tile::Straight, 1, (-3, 2), -1, -1);
    add_move(&mut mvs, Tile::Left, 0, (-3, 5), 10, 0);
    add_move(&mut mvs, Tile::Curve, 2, (-3, 4), -1, -1);
    add_move(&mut mvs, Tile::TriangleWithRoadWithCOA, 1, (-1, -2), -1, -1);
    add_move(&mut mvs, Tile::Straight, 1, (-1, -3), -1, -1);
    add_move(&mut mvs, Tile::CityCap, 3, (-2, 6), 11, 0);
    add_move(&mut mvs, Tile::Triangle, 3, (-2, 4), -1, -1);

    /* FIXME: field calculation is not correct
    let status = calculate(&mvs, true);
    match status {
      Ok(mut res) => {
        res.complete_events.sort();
        assert_eq!(res.complete_events.len(), 7);
        assert_eq!(res.complete_events[6].feature, FieldFeature);
        assert_eq!(res.complete_events[6].meeple_ids, vec![9]);
        assert_eq!(res.complete_events[6].point, 12);
        assert_eq!(res.player0_point, 31);
        assert_eq!(res.player1_point, 35);
      }
      Err(e) => { panic!("Error: {:?}", e.detail); }
    }
    */

    add_move(&mut mvs, Tile::CityCap, 2, (-4, 5), 11, 1);
    add_move(&mut mvs, Tile::TripleCity, 0, (-2, -3), 1, 0);
    add_move(&mut mvs, Tile::ConnectorWithCOA, 0, (-5, 5), -1, -1);
    add_move(&mut mvs, Tile::TriangleWithRoad, 0, (-3, 1), -1, -1);
    add_move(&mut mvs, Tile::QuadrupleRoad, 0, (2, 1), 10, 1);
    add_move(&mut mvs, Tile::ConnectorWithCOA, 0, (-5, 4), 2, 1);
    add_move(&mut mvs, Tile::TripleRoad, 1, (-3, 3), 10, 4);

    // but field calculation is now correct
    let status = calculate(&mvs, true);
    match status {
        Ok(mut res) => {
            res.complete_events.sort();
            for e in &res.complete_events {
                println!("{:?}", e);
            }
            assert_eq!(res.complete_events.len(), 8);
            assert_eq!(res.complete_events[7].feature, FieldFeature);
            assert_eq!(res.complete_events[7].meeple_ids, vec![9]);
            assert_eq!(res.complete_events[7].point, 12);
            assert_eq!(res.player0_point, 37);
            assert_eq!(res.player1_point, 48);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
}

#[test]
fn falculate_test_closed_field() {
    let mut mvs = vec![];
    add_move(&mut mvs, Tile::StartingTile, 0, (0, 0), -1, -1);
    add_move(&mut mvs, Tile::StartingTile, 2, (-1, 0), -1, -1);
    add_move(&mut mvs, Tile::Curve, 3, (-1, -1), 0, 2);
    add_move(&mut mvs, Tile::Curve, 2, (0, -1), -1, -1);
    add_move(&mut mvs, Tile::Curve, 0, (-1, 1), -1, -1);
    add_move(&mut mvs, Tile::Curve, 1, (0, 1), -1, -1);
    let status = calculate(&mvs, false);
    match status {
        Ok(mut res) => {
            res.complete_events.sort();
            for e in &res.complete_events {
                println!("{:?}", e);
            }
            assert_eq!(res.complete_events.len(), 0);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
    let status = calculate(&mvs, true);
    match status {
        Ok(mut res) => {
            res.complete_events.sort();
            for e in &res.complete_events {
                println!("{:?}", e);
            }
            assert_eq!(res.complete_events.len(), 1);
            assert_eq!(res.complete_events[0].point, 3);
            assert_eq!(res.complete_events[0].feature, FieldFeature);
            assert_eq!(res.complete_events[0].meeple_ids, vec![0]);
        }
        Err(e) => {
            panic!("Error: {:?}", e.detail);
        }
    }
}

#[test]
fn calculate_test_with_bga_json_data() {
    let test_data = vec![
        ("src/data/367163108.json", 89, 85),
        ("src/data/366166200.json", 70, 96),
        ("src/data/370417702.json", 91, 108),
        ("src/data/369577999.json", 103, 96),
        ("src/data/368679629.json", 91, 94),
        ("src/data/377230846.json", 86, 101),
    ];
    for d in test_data {
        let file_path = d.0;
        let exp_player0_point = d.1;
        let exp_player1_point = d.2;

        let mvs = decoder::decode(file_path.to_string());
        let status = calculate(&mvs, true);
        match status {
            Ok(res) => {
                assert_eq!(res.player0_point, exp_player0_point);
                assert_eq!(res.player1_point, exp_player1_point);
            }
            Err(e) => {
                panic!("Error: {:?}", e.detail);
            }
        }
    }
}
