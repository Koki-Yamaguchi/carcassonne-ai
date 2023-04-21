use std::collections::HashMap;

use super::tile;
use super::calculate::{calculate, calculate_tileable_positions};
use super::tile::Tile;
use super::mov::{Move, Move::*, TileMove, MeepleMove };

#[derive(Debug)]
pub struct Win {
  pos: (i32, i32),
  rot: i32,
  meeple_pos: i32,
  win_player_id: i32,
}

pub fn next_permutation<T: std::cmp::Ord>(arr: &mut [T]) -> bool {
    use std::cmp::Ordering;
    let last_ascending = match arr.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            arr.reverse();
            return false;
        }
    };
    let swap_with = arr[last_ascending + 1..]
        .binary_search_by(|n| match arr[last_ascending].cmp(n) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        })
        .unwrap_err();
    arr.swap(last_ascending, last_ascending + swap_with);
    arr[last_ascending + 1..].reverse();
    true
}

pub fn search(mvs: &Vec<Move>, ordered_tiles: Vec<Tile>, player_id: i32, other_player_id: i32, player0_id: i32, player1_id: i32) -> Vec<Win> {
  let mut moves = mvs.clone();

  let next_tile = ordered_tiles.first().unwrap();

  // O(n log n)
  let tileable_positions = calculate_tileable_positions(&moves, *next_tile);

  let next_ordered_tiles = ordered_tiles[1..].to_vec();

  let mut wins = vec![];

  // FIXME: meepleable_positions must be calculated more efficiently
  for tileable_position in &tileable_positions {
    moves.push(TMove(TileMove { ord: -1, game_id: -1, player_id, tile: *next_tile, rot: tileable_position.rot, pos: tileable_position.pos }));

    let (mut meepleable_positions, meeples0, meeples1) = match calculate(&moves, false) {
      Ok(s) => {
        (s.meepleable_positions, s.player0_remaining_meeples, s.player1_remaining_meeples)
      }
      Err(e) => { panic!("{}", e.detail.msg); }
    };
    meepleable_positions.push(-1); // not meeple the tile

    for meepleable_position in &meepleable_positions {
      let mut meeple_id = -1;
      if *meepleable_position != -1 {
        if player_id == player0_id {
          if meeples0.len() == 0 {
            continue;
          }
          meeple_id = meeples0.iter().next().unwrap().clone();
        } else {
          if meeples1.len() == 0 {
            continue;
          }
          meeple_id = meeples1.iter().next().unwrap().clone();
        }
      }

      moves.push(MMove(MeepleMove { ord: -1, game_id: -1, player_id, meeple_id, tile_pos: tileable_position.pos, meeple_pos: *meepleable_position }));

      if next_ordered_tiles.len() == 0 {
        match calculate(&moves, true) {
          Ok(s) => {
            wins.push(Win {
              pos: tileable_position.pos,
              rot: tileable_position.rot,
              meeple_pos: *meepleable_position,
              win_player_id: if s.player0_point > s.player1_point { player0_id } else { player1_id },
            });
          }
          Err(e) => {
            panic!("{}", e.detail.msg);
          }
        }
      } else {
        let mut lose = false;
        let results = search(&moves, next_ordered_tiles.clone(), other_player_id, player_id, player0_id, player1_id);
        for res in results {
          lose = lose || res.win_player_id == other_player_id
        }

        wins.push(Win {
          pos: tileable_position.pos,
          rot: tileable_position.rot,
          meeple_pos: *meepleable_position,
          win_player_id: if lose { other_player_id } else { player_id },
        });
      }
      moves.pop(); // pop meeple move
    }

    moves.pop(); // pop tile move

  }

  wins
}

pub fn solve(moves: &Vec<Move>, game_id: i32, player_id: i32, other_player_id: i32, player0_id: i32, player1_id: i32, next_tile: Tile) -> (TileMove, MeepleMove) {
  // check what remaining tiles are
  let mut out_tiles = vec![next_tile];
  for mv in moves {
    match mv {
      Move::TMove(tm) => {
        out_tiles.push(tm.tile.clone());
      }
      _ => {}
    }
  }

  let remaining_tiles = tile::remaining_tiles(out_tiles.clone());

  let remaining_tiles_num = remaining_tiles.len();
  let mut remaining_tiles_idx: Vec<usize> = (0..remaining_tiles_num).collect();

  let mut order_count = 0;
  let mut win_count = HashMap::<(i32, i32, i32, i32), i32>::new();
  let mut total_wins = vec![];

  loop {
    let mut ordered_remaining_tiles = remaining_tiles_idx.clone().into_iter().map(|idx| remaining_tiles[idx]).collect();
    let mut ordered_tiles = vec![next_tile];
    ordered_tiles.append(&mut ordered_remaining_tiles);

    let wins = search(&moves, ordered_tiles, player_id, other_player_id, player0_id, player1_id);

    for win in wins {
      if win.win_player_id == 1 { continue; }
      total_wins.push(win);
    }

    order_count += 1;

    if !next_permutation(&mut remaining_tiles_idx) {
      break;
    }
  }

  for win in &total_wins {
    win_count.insert((win.pos.0, win.pos.1, win.rot, win.meeple_pos), 0);
  }
  for win in &total_wins {
    win_count.entry((win.pos.0, win.pos.1, win.rot, win.meeple_pos)).and_modify(|v| *v += 1);
  }

  println!("====== Winnable Moves ======");
  for (key, value) in win_count {
    println!("position ({}, {}), rotated {} times, meeple on feature {} (win probablity {}%)", key.1, -key.0, key.2, key.3, value as f64 * 100.0 / order_count as f64);
  }
  println!();

  (
    TileMove { ord: 0, game_id, player_id: player_id, tile: next_tile, rot: 0, pos: (-1, -1) },
    MeepleMove { ord: 1, game_id, player_id: player_id, meeple_id: -1, tile_pos: (-1, -1), meeple_pos: -1 },
  )
}

#[allow(dead_code)]
fn add_move(mvs: &mut Vec<Move>, player_id: i32, tile: Tile, rot: i32, pos: (i32, i32), meeple_id: i32, meeple_pos: i32) {
  mvs.push(Move::TMove( TileMove { ord: -1, game_id: -1, player_id, tile, rot, pos, } ));
  mvs.push(Move::MMove( MeepleMove { ord: -1, game_id: -1, player_id, meeple_id: meeple_id, tile_pos: pos, meeple_pos } ));
}

#[test]
fn solve_test0() {
  // solve the last 3 tiles in https://boardgamearena.com/table?table=367130620
  // first player's id is 0
  let mut mvs = vec![];
  add_move(&mut mvs, 1, Tile::StartingTile, 0, (0, 0), -1, -1);
  add_move(&mut mvs, 0, Tile::TripleRoad, 1, (0, 1), 0, 4);
  add_move(&mut mvs, 1, Tile::Straight, 0, (-1, 1), 7, 1);
  add_move(&mut mvs, 0, Tile::CityCap, 2, (-1, 0), 1, 0);
  add_move(&mut mvs, 1, Tile::Triangle, 0, (-1, -1), 8, 0);
  add_move(&mut mvs, 0, Tile::Separator, 1, (-1, 2), 1, 0);
  add_move(&mut mvs, 1, Tile::TripleRoad, 3, (-2, 1), -1, -1);
  add_move(&mut mvs, 0, Tile::Separator, 0, (-1, 3), 2, 0);
  add_move(&mut mvs, 1, Tile::Monastery, 0, (0, 2), 7, 0);
  add_move(&mut mvs, 0, Tile::StartingTile, 2, (-2, 2), 1, 0);
  add_move(&mut mvs, 1, Tile::Straight, 0, (1, 1), 9, 2);
  add_move(&mut mvs, 0, Tile::VerticalSeparator, 0, (-1, 4), 1, 0);
  add_move(&mut mvs, 1, Tile::CityCapWithCrossroad, 0, (0, 4), 10, 0);
  add_move(&mut mvs, 0, Tile::TriangleWithRoad, 2, (-2, 3), 3, 2);
  add_move(&mut mvs, 1, Tile::TripleCity, 3, (-2, -1), -1, -1);
  add_move(&mut mvs, 0, Tile::Left, 3, (0, -1), 4, 0);
  add_move(&mut mvs, 1, Tile::Curve, 3, (0, 3), -1, -1);
  add_move(&mut mvs, 0, Tile::Left, 1, (0, -2), -1, -1);
  add_move(&mut mvs, 1, Tile::Curve, 2, (-1, 5), -1, -1);
  add_move(&mut mvs, 0, Tile::CityCap, 2, (1, -2), 4, 0);
  add_move(&mut mvs, 1, Tile::StartingTile, 1, (-2, 5), -1, -1);
  add_move(&mut mvs, 0, Tile::Straight, 1, (-3, 2), -1, -1);
  add_move(&mut mvs, 1, Tile::Left, 0, (-3, 5), 10, 0);
  add_move(&mut mvs, 0, Tile::Curve, 2, (-3, 4), -1, -1);
  add_move(&mut mvs, 1, Tile::TriangleWithRoadWithCOA, 1, (-1, -2), -1, -1);
  add_move(&mut mvs, 0, Tile::Straight, 1, (-1, -3), -1, -1);
  add_move(&mut mvs, 1, Tile::CityCap, 3, (-2, 6), 11, 0);
  add_move(&mut mvs, 0, Tile::Triangle, 3, (-2, 4), -1, -1);
  add_move(&mut mvs, 1, Tile::CityCap, 2, (-4, 5), 11, 1);
  add_move(&mut mvs, 0, Tile::TripleCity, 0, (-2, -3), 1, 0);
  add_move(&mut mvs, 1, Tile::ConnectorWithCOA, 0, (-5, 5), -1, -1);

  // FIXME: field calculation is wrong
  /*
  let status = calculate(&mvs, true);
  match status {
    Ok(res) => {
      let mut events = vec![];
      for e in res.complete_events {
        println!("{:?}", e);
        events.push(e);
      }
      println!("player0 {:?}", res.player0_point);
      println!("player1 {:?}", res.player1_point);
    }
    Err(e) => { panic!("Error: {:?}", e.detail); }
  }
  */

  add_move(&mut mvs, 0, Tile::TriangleWithRoad, 0, (-3, 1), -1, -1);
  add_move(&mut mvs, 1, Tile::QuadrupleRoad, 0, (2, 1), 10, 1);
  add_move(&mut mvs, 0, Tile::ConnectorWithCOA, 0, (-5, 4), 2, 1);
  add_move(&mut mvs, 1, Tile::TripleRoad, 1, (-3, 3), 10, 4);
  add_move(&mut mvs, 0, Tile::TripleCityWithRoadWithCOA, 2, (1, 4), 3, 2);
  add_move(&mut mvs, 1, Tile::TripleRoad, 0, (-4, 3), 10, 2);
  add_move(&mut mvs, 0, Tile::Straight, 1, (-1, 6), 3, 1);
  add_move(&mut mvs, 1, Tile::StartingTile, 1, (1, 3), 12, 0);
  add_move(&mut mvs, 0, Tile::TriangleWithCOA, 0, (-5, 6), -1, -1);
  add_move(&mut mvs, 1, Tile::TripleCity, 1, (2, 4), -1, -1);
  add_move(&mut mvs, 0, Tile::Monastery, 0, (-2, 7), -1, -1);
  add_move(&mut mvs, 1, Tile::Curve, 0, (-4, 4), -1, -1);
  add_move(&mut mvs, 0, Tile::CityCapWithCrossroad, 2, (-6, 6), -1, -1);
  add_move(&mut mvs, 1, Tile::Right, 3, (1, 5), -1, -1);
  add_move(&mut mvs, 0, Tile::MonasteryWithRoad, 2, (1, -1), -1, -1);
  add_move(&mut mvs, 1, Tile::CityCap, 2, (1, 2), -1, -1);
  add_move(&mut mvs, 0, Tile::CityCapWithCrossroad, 0, (2, -2), 0, 1);
  add_move(&mut mvs, 1, Tile::Monastery, 0, (1, 0), 7, 0);

  // field calculation is now correct
  /*
  let status = calculate(&mvs, true);
  match status {
    Ok(res) => {
      let mut events = vec![];
      for e in res.complete_events {
        println!("{:?}", e);
        events.push(e);
      }
      println!("player0 {:?}", res.player0_point);
      println!("player1 {:?}", res.player1_point);
    }
    Err(e) => { panic!("Error: {:?}", e.detail); }
  }
  */

  add_move(&mut mvs, 0, Tile::Straight, 1, (2, -1), 4, 1);
  add_move(&mut mvs, 1, Tile::Connector, 1, (3, 4), -1, -1);
  add_move(&mut mvs, 0, Tile::TriangleWithCOA, 2, (3, 5), -1, -1);
  add_move(&mut mvs, 1, Tile::Straight, 1, (2, 0), -1, -1);
  add_move(&mut mvs, 0, Tile::TripleCityWithRoad, 0, (4, 5), 4, 0);
  add_move(&mut mvs, 1, Tile::Curve, 3, (0, 6), -1, -1);
  add_move(&mut mvs, 0, Tile::MonasteryWithRoad, 2, (2, 3), 5, 2);
  add_move(&mut mvs, 1, Tile::VerticalSeparator, 1, (2, 5), 7, 0);
  add_move(&mut mvs, 0, Tile::TriangleWithRoadWithCOA, 3, (3, 6), -1, -1);
  add_move(&mut mvs, 1, Tile::TripleCityWithCOA, 1, (-3, -1), -1, -1);
  add_move(&mut mvs, 0, Tile::Straight, 1, (0, 7), -1, -1);
  add_move(&mut mvs, 1, Tile::Right, 2, (-3, -2), -1, -1);
  add_move(&mut mvs, 0, Tile::Curve, 1, (1, 6), 5, 1);
  add_move(&mut mvs, 1, Tile::Monastery, 0, (3, 3), -1, -1);
  add_move(&mut mvs, 0, Tile::Curve, 0, (0, 5), -1, -1);
  add_move(&mut mvs, 1, Tile::QuadrupleCityWithCOA, 0, (-3, 0), -1, -1);
  add_move(&mut mvs, 0, Tile::Triangle, 1, (-5, 3), -1, -1);
  add_move(&mut mvs, 1, Tile::Right, 3, (-6, 4), 13, 0);
  add_move(&mut mvs, 0, Tile::TripleCityWithRoadWithCOA, 1, (4, 4), -1, -1);
  add_move(&mut mvs, 1, Tile::Curve, 3, (4, 3), -1, -1);

  let status = calculate(&mvs, true);
  match status {
    Ok(res) => {
      let mut events = vec![];
      for e in res.complete_events {
        events.push(e);
      }
    }
    Err(e) => { panic!("Error: {:?}", e.detail); }
  }

  solve(&mvs, -1, 0, 1, 0, 1, Tile::VerticalSeparator);
}
