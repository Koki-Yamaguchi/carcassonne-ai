pub mod mov;

use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::database;
use mov::Move::*;
use mov::Tile::*;
use mov::{ TileMove, SkipMove };

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateGame {
  pub note: String,
  pub player0_id: i32,
  pub player1_id: i32,
}

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
}

pub fn create_game(note: String, player0_id: i32, player1_id: i32) -> Game {
  let g = database::create_game(
    note,
    player0_id,
    player1_id,
  );

  let mv0 = TMove( TileMove { ord: 1, game_id: g.id, player_id: player1_id, tile: StartingTile, rot: 0, pos: (0, 0) } );
  let mv1 = SMove( SkipMove { ord: 1, game_id: g.id, player_id: player1_id } );

  database::create_move(mv0);
  database::create_move(mv1);

  g
}
