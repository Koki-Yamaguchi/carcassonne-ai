use super::tile::Tile;
use super::mov::{Move, TileMove, MeepleMove };

pub fn calculate_next_move(_moves: &Vec<Move>, game_id: i32, player_id: i32, next_tile: Tile) -> (TileMove, MeepleMove) {
  // calulate the following
  let y = 51;
  let x = 50;
  let rot = 0;
  let meeple_id = 7;
  let meeple_pos = 2;

  (
    TileMove { ord: 0, game_id, player_id: player_id, tile: next_tile, rot, pos: (y, x) },
    MeepleMove { ord: 1, game_id, player_id: player_id, meeple_id, tile_pos: (y, x), meeple_pos },
  )
}