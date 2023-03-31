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

pub struct Error {
  msg: String
}

#[derive(Copy, Clone)]
struct TileItem {
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
      Tile::CityCapWithCrossroad => [Road, City, Road, Road],
      Tile::TriagnleWithRoad => [Road, City, City, Road],
      Tile::TriagnleWithRoadWithCOA => [Road, City, City, Road],
      Tile::Invalid => [Field, Field, Field, Field]
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
  fn feature_size(self) -> i32 {
    match self.tile {
      Tile::StartingTile => 4,
      Tile::Monastery => 2,
      Tile::CityCapWithCrossroad => 7,
      Tile::TriagnleWithRoad => 4,
      Tile::TriagnleWithRoadWithCOA => 4,
      Tile::Invalid => 0,
    }
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
      Tile::TriagnleWithRoad => vec![
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
      Tile::TriagnleWithRoadWithCOA => vec![
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
      Tile::CityCapWithCrossroad => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 4, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 5, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 6, feature: FieldFeature },
      ],
      Tile::TriagnleWithRoad => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
      ],
      Tile::TriagnleWithRoadWithCOA => vec![
        DistinctFeature { id: self.feature_starting_id + 0, feature: CityFeature },
        DistinctFeature { id: self.feature_starting_id + 1, feature: FieldFeature },
        DistinctFeature { id: self.feature_starting_id + 2, feature: RoadFeature },
        DistinctFeature { id: self.feature_starting_id + 3, feature: FieldFeature },
      ],
      Tile::Invalid => vec![],
    }
  }
}

fn create_mergeable_features(mf: &mut MergeableFeature, t: Tile) {
  match t {
    Tile::StartingTile => {
      mf.new_feature(1, false);
      mf.new_feature(2, false);
      mf.new_feature(2, false);
      mf.new_feature(3, false);
    },
    Tile::Monastery => {
      mf.new_feature(9, false);
      mf.new_feature(4, false);
    },
    Tile::CityCapWithCrossroad => {
      mf.new_feature(1, false);
      mf.new_feature(2, false);
      mf.new_feature(1, false);
      mf.new_feature(1, false);
      mf.new_feature(2, false);
      mf.new_feature(1, false);
      mf.new_feature(2, false);
    },
    Tile::TriagnleWithRoad => {
      mf.new_feature(2, false);
      mf.new_feature(2, false);
      mf.new_feature(2, false);
      mf.new_feature(2, false);
    },
    Tile::TriagnleWithRoadWithCOA => {
      mf.new_feature(2, true);
      mf.new_feature(2, false);
      mf.new_feature(2, false);
      mf.new_feature(2, false);
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
    Tile::CityCapWithCrossroad => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
    Tile::TriagnleWithRoad => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
    Tile::TriagnleWithRoadWithCOA => {
      mf.set_cities((t.feature_starting_id + 1) as usize, (t.feature_starting_id + 0) as usize);
    },
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
  for mv in moves {
    match mv {
      Move::TMove(m) => {
        let current_tile = TileItem {
          tile: m.tile,
          rot: m.rot,
          feature_starting_id: current_feature_id,
        };

        create_mergeable_features(&mut mergeable_features, m.tile);
        set_cities_to_fields(&mut mergeable_features, &current_tile);

        current_feature_id += current_tile.feature_size();

        let y = m.pos.0 as usize;
        let x = m.pos.1 as usize;
        match board[y][x] {
          Empty => {}
          _ => return Err(Error{ msg: "invalid moves".to_string() })
        }
        match (y, x, &board[y - 1][x], &board[y + 1][x], &board[y][x - 1], &board[y][x + 1]) {
          (50, 50, _, _, _, _) => {} /* initial tile */
          (_, _, &Empty, &Empty, &Empty, &Empty) => {
            return Err(Error{ msg: "invalid moves".to_string() })
          }
          (_, _, _, _, _, _) => {}
        }
        let top_must_be = match board[y - 1][x] { Tile(t) => t.bottom(), Empty => None };
        let bottom_must_be = match board[y + 1][x] { Tile(t) => t.top(), Empty => None };
        let left_must_be = match board[y][x - 1] { Tile(t) => t.right(), Empty => None };
        let right_must_be = match board[y][x + 1] { Tile(t) => t.left(), Empty => None };
        if top_must_be != None && top_must_be != current_tile.top() {
          return Err(Error{ msg: "invalid moves".to_string() })
        }
        if bottom_must_be != None && bottom_must_be != current_tile.bottom() {
          return Err(Error{ msg: "invalid moves".to_string() })
        }
        if left_must_be != None && left_must_be != current_tile.left() {
          return Err(Error{ msg: "invalid moves".to_string() })
        }
        if right_must_be != None && right_must_be != current_tile.right() {
          return Err(Error{ msg: "invalid moves".to_string() })
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
            return Err(Error{ msg: "invalid moves".to_string() })
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
          Tile::Monastery => {
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
                  Tile::Monastery => {
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
              return Err(Error{ msg: "invalid moves".to_string() })
            }
            Tile(t) => {
              let feature_id = t.feature_starting_id + m.meeple_pos;
              if mergeable_features.get_meeples(feature_id as usize).len() != 0 {
                return Err(Error{ msg: "invalid moves".to_string() })
              }
              mergeable_features.place_meeple(feature_id as usize, m.meeple_id);
            }
          }
        }
        match board[y][x] {
          Empty => {
            return Err(Error{ msg: "invalid moves".to_string() })
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
                  Tile::Monastery => {
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

  mvs.push(Move::TMove( TileMove { ord: 2, game_id, player_id: player0_id, tile: Tile::TriagnleWithRoad, rot: 2, pos: (49, 50) } ));
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

  mvs.push(Move::TMove( TileMove { ord: 10, game_id, player_id: player0_id, tile: Tile::TriagnleWithRoadWithCOA, rot: 3, pos: (49, 51) } ));
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