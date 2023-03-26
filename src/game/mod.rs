pub mod mov;
pub mod tile;

use rocket::serde::{Serialize};
use diesel::prelude::*;

use crate::database;
use mov::Move::*;
use tile::Tile::*;
use mov::{ TileMove, MeepleMove, SkipMove };

#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Game {
  pub id: i32,
  pub note: String,
  pub player0_id: i32,
  pub player1_id: i32,
  pub player0_point: i32,
  pub player1_point: i32,
  pub created_at: chrono::NaiveDateTime,
  pub ended_at: Option<chrono::NaiveDateTime>,
  pub next_tile_id: Option<i32>,
}

#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MeepleablePositions {
  pub meepleable_positions: Vec<i32>,
}

pub fn create_game(note: String, player0_id: i32, player1_id: i32) -> Game {
  let next_tile = Monastery;

  let g = database::create_game(
    note,
    player0_id,
    player1_id,
    Some(next_tile.to_id()),
  );

  let mv0 = TMove( TileMove { ord: 0, game_id: g.id, player_id: player1_id, tile: StartingTile, rot: 0, pos: (0, 0) } );
  let mv1 = SMove( SkipMove { ord: 1, game_id: g.id, player_id: player1_id } );

  database::create_move(mv0);
  database::create_move(mv1);

  g
}

pub fn create_tile_move(game_id: i32, player_id: i32, tile: tile::Tile, rot: i32, pos: (i32, i32)) -> MeepleablePositions {
  let moves = database::list_moves(game_id);
  assert!(moves.len() != 0);

  let ord = match moves.last().unwrap() {
    MMove(m) => { m.ord + 1 },
    TMove(m) => { m.ord + 1 },
    SMove(m) => { m.ord + 1 },
    InvalidMove => { 0 }
  };

  let mv = TMove( TileMove { ord, game_id, player_id, tile, rot, pos } );

  database::create_move(mv);

  // update game.next_tile_id

  MeepleablePositions {
    meepleable_positions: vec![1, 2, 3], // calculate meepleable positions on the tile
  }
}

pub fn create_meeple_move(game_id: i32, player_id: i32, meeple_id: i32, pos: i32) -> MeepleablePositions {
  let moves = database::list_moves(game_id);
  assert!(moves.len() != 0);

  let ord = match moves.last().unwrap() {
    MMove(m) => { m.ord + 1 },
    TMove(m) => { m.ord + 1 },
    SMove(m) => { m.ord + 1 },
    InvalidMove => { 0 }
  };

  let mv = MMove( MeepleMove { ord, game_id, player_id, meeple_id, pos } );

  database::create_move(mv);

  // actually return how point changes and meeple goes back
  MeepleablePositions {
    meepleable_positions: vec![1, 2, 3],
  }
}
