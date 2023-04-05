pub mod mov;
pub mod tile;
pub mod calculate;
pub mod calculate_next_move;
pub mod mergeable_feature;

use rocket::{serde::{Serialize}};
use diesel::prelude::*;

use crate::{database};
use mov::Move::*;
use tile::Tile::*;
use mov::{ TileMove, MeepleMove };
use rand::Rng;
use self::tile::tiles;

#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = schema::game)]
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

#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CompleteEvent {
  pub meeple_ids: Vec<i32>,
  pub feature: String,
  pub point: i32,
}

pub fn create_game(note: String, player0_id: i32, player1_id: i32) -> Game {
  let mut rng = rand::thread_rng();
  let first_player_id = if rng.gen_range(0..2) < 1 { player0_id } else { player1_id };
  let second_player_id = if first_player_id == player0_id { player1_id } else { player0_id };

  let tiles = tiles();
  let next_tile = tiles[rng.gen_range(0..tiles.len())];

  let g = database::create_game(
    note,
    player0_id,
    player1_id,
    Some(next_tile.to_id()),
  );

  let mv0 = TMove( TileMove { ord: 0, game_id: g.id, player_id: second_player_id, tile: StartingTile, rot: 0, pos: (50, 50) } );
  let mv1 = MMove( MeepleMove { ord: 1, game_id: g.id, player_id: second_player_id, meeple_id: -1, tile_pos: (50, 50), meeple_pos: -1 });

  database::create_move(mv0.clone());
  database::create_move(mv1.clone());

  g
}

pub fn create_tile_move(game_id: i32, player_id: i32, tile: tile::Tile, rot: i32, pos: (i32, i32)) -> MeepleablePositions {
  let mut moves = database::list_moves(game_id);
  assert!(moves.len() != 0);

  let ord = match moves.last().unwrap() {
    MMove(m) => { m.ord + 1 },
    TMove(m) => { m.ord + 1 },
    InvalidMove => { 0 }
  };

  let mv = TMove( TileMove { ord, game_id, player_id, tile, rot, pos } );
  moves.push(mv.clone());

  let res = calculate::calculate(&moves, false);
  let meepleable_positions = match res {
    Ok(s) => { s.meepleable_positions }
    Err(_) => { vec![] }
  };

  database::create_move(mv);

  MeepleablePositions {
    meepleable_positions,
  }
}

pub fn create_meeple_move(game_id: i32, player_id: i32, meeple_id: i32, tile_pos: (i32, i32), meeple_pos: i32) -> Vec<CompleteEvent> {
  let mut rng = rand::thread_rng();
  let mut moves = database::list_moves(game_id);
  assert!(moves.len() != 0);

  let ord = match moves.last().unwrap() {
    MMove(m) => { m.ord + 1 },
    TMove(m) => { m.ord + 1 },
    InvalidMove => { 0 }
  };

  let mv = MMove( MeepleMove { ord, game_id, player_id, meeple_id, tile_pos, meeple_pos } );
  moves.push(mv.clone());

  database::create_move(mv);

  let mut complete_events = vec![];

  let res = calculate::calculate(&moves, false);
  match res {
    Ok(s) => {
      for e in &s.complete_events {
        complete_events.push(CompleteEvent {
          meeple_ids: e.meeple_ids.clone(),
          feature: e.feature.clone().to_string(),
          point: e.point,
        })
      }
    }
    Err(_) => {}
  };

  let mut tiles = vec![];
  for mv in moves {
    match mv {
      mov::Move::TMove(tm) => {
        tiles.push(tm.tile.clone());
      }
      _ => {}
    }
  }
  let remaining_tiles = tile::remaining_tiles(tiles.clone());
  let next_tile = remaining_tiles[rng.gen_range(0..tiles.len())];
  let _ = database::update_game(
    game_id,
    next_tile.to_id(),
  );

  complete_events
}

pub fn wait_ai_move(game_id: i32) -> Vec<CompleteEvent> {
  let game = database::get_game(game_id).expect("game not found");

  let moves = database::list_moves(game.id);
  assert!(moves.len() != 0);

  let next_tile = tile::to_tile(game.next_tile_id.unwrap());
  let (tile_move, meeple_move): (TileMove, MeepleMove) = calculate_next_move::calculate_next_move(
    &moves,
    game.id,
    1,
    next_tile,
  );

  let r = create_tile_move(game.id, 1, next_tile, tile_move.rot, tile_move.pos);
  assert!(meeple_move.meeple_id == -1 || r.meepleable_positions.contains(&meeple_move.meeple_pos));
  let complete_event = create_meeple_move(game.id, 1, meeple_move.meeple_id, meeple_move.tile_pos, meeple_move.meeple_pos);

  complete_event
}

pub fn get_game(game_id: i32) -> Game {
  database::get_game(game_id).expect("game not found")
}
