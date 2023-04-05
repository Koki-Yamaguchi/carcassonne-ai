use super::calculate::{calculate, TileItem, Square::*, Side::*};
use super::tile::Tile;
use super::mov::{Move, TileMove, MeepleMove };

pub fn calculate_next_move(moves: &Vec<Move>, game_id: i32, player_id: i32, next_tile: Tile) -> (TileMove, MeepleMove) {
  let mut tile = TileItem {
    id: next_tile.to_id(),
    tile: next_tile,
    rot: 0,
    feature_starting_id: -1,
  };

  let tile_move_ord = moves.clone().last().unwrap().ord() + 1;
  let meeple_move_ord = tile_move_ord + 1;

  let board = match calculate(&moves, false) {
    Ok(s) => {
      s.board
    }
    Err(_) => { vec![vec![]] }
  };

  assert!(board.len() > 0);
  assert!(board[0].len() > 0);

  for _rot in 0..4 {
    for y in 1..board.len() - 1 {
      for x in 1..board[0].len() - 1 {
        match board[y][x] {
          Tile(_) => {}
          Empty => {
            match (&board[y - 1][x], &board[y + 1][x], &board[y][x - 1], &board[y][x + 1]) {
              (&Empty, &Empty, &Empty, &Empty) => {
                continue;
              }
              (_, _, _, _) => {
                let top_must_be = match board[y - 1][x] { Tile(t) => t.bottom(), Empty => None };
                let bottom_must_be = match board[y + 1][x] { Tile(t) => t.top(), Empty => None };
                let left_must_be = match board[y][x - 1] { Tile(t) => t.right(), Empty => None };
                let right_must_be = match board[y][x + 1] { Tile(t) => t.left(), Empty => None };
                if top_must_be != None && top_must_be != tile.top() {
                  continue;
                }
                if bottom_must_be != None && bottom_must_be != tile.bottom() {
                  continue;
                }
                if left_must_be != None && left_must_be != tile.left() {
                  continue;
                }
                if right_must_be != None && right_must_be != tile.right() {
                  continue;
                }
                return (
                  TileMove { ord: tile_move_ord, game_id, player_id, tile: next_tile, rot: tile.rot, pos: (y as i32, x as i32) },
                  MeepleMove { ord: meeple_move_ord, game_id, player_id, meeple_id: -1, tile_pos: (y as i32, x as i32), meeple_pos: -1 },
                )
              }
            }
          }
        }
      }
    }
    tile.rotate();
  }

  (
    TileMove { ord: 0, game_id, player_id: player_id, tile: next_tile, rot: 0, pos: (-1, -1) },
    MeepleMove { ord: 1, game_id, player_id: player_id, meeple_id: -1, tile_pos: (-1, -1), meeple_pos: -1 },
  )
}