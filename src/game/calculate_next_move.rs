use std::collections::HashMap;

use super::calculate::calculate;
use super::calculate::TileItem;
use super::tile::Tile;
use super::mov::{Move, TileMove, MeepleMove };

pub fn calculate_next_move(moves: &Vec<Move>, game_id: i32, player_id: i32, next_tile: Tile) -> (TileMove, MeepleMove) {
  let mut tile = TileItem {
    id: next_tile.to_id(),
    tile: next_tile,
    rot: 0,
    feature_starting_id: 0,
  };

  let tile_move_ord = moves.last().unwrap().ord() + 1;
  let meeple_move_ord = tile_move_ord + 1;

  let board = match calculate(&moves, false) {
    Ok(s) => {
      s.board
    }
    Err(_) => { HashMap::new() }
  };

  let mut checked = HashMap::new();
  for pos in board.keys() {
    match checked.get(pos) {
      Some(_) => {
        continue;
      }
      None => {},
    }
    checked.insert(pos.clone(), true);

    let y = pos.0;
    let x = pos.1;
    let dy = [0, -1, 0, 1];
    let dx = [1, 0, -1, 0];
    for i in 0..4 {
      let ny = y + dy[i];
      let nx = x + dx[i];
      match board.get(&(ny, nx)) {
        Some(_) => {
          continue;
        }
        None => {},
      }
      for rot in vec![1, 2, 3, 4] {
        tile.rotate();

        match board.get(&(ny - 1, nx)) {
          Some(t) => {
            if t.bottom() != tile.top() {
              continue;
            }
          }
          None => {},
        }
        match board.get(&(ny + 1, nx)) {
          Some(t) => {
            if t.top() != tile.bottom() {
              continue;
            }
          }
          None => {},
        }
        match board.get(&(ny, nx - 1)) {
          Some(t) => {
            if t.right() != tile.left() {
              continue;
            }
          }
          None => {},
        }
        match board.get(&(ny, nx + 1)) {
          Some(t) => {
            if t.left() != tile.right() {
              continue;
            }
          }
          None => {},
        }
        return (
          TileMove { ord: tile_move_ord, game_id, player_id, tile: next_tile, rot: rot, pos: (ny, nx) },
          MeepleMove { ord: meeple_move_ord, game_id, player_id, meeple_id: -1, tile_pos: (ny, nx), meeple_pos: -1 },
        );
      }
    }
  }

  (
    TileMove { ord: 0, game_id, player_id: player_id, tile: next_tile, rot: 0, pos: (-1, -1) },
    MeepleMove { ord: 1, game_id, player_id: player_id, meeple_id: -1, tile_pos: (-1, -1), meeple_pos: -1 },
  )
}
