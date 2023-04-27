pub mod mov;
pub mod tile;
pub mod calculate;
pub mod calculate_next_move;
pub mod solver;
pub mod mergeable_feature;
pub mod decoder;

use rocket::{serde::{Serialize}};
use diesel::prelude::*;

use crate::database;
use crate::error::Error;

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
  pub next_player_id: Option<i32>,
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
#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MeepleMoveResult {
  pub complete_events: Vec<CompleteEvent>,
  pub next_tile_id: i32,
  pub next_player_id: i32,
}

pub fn create_game(note: String, player0_id: i32, player1_id: i32) -> Result<Game, Error> {
  let mut rng = rand::thread_rng();
  let first_player_id = if rng.gen_range(0..2) < 1 { player0_id } else { player1_id };
  let second_player_id = if first_player_id == player0_id { player1_id } else { player0_id };

  let tiles = tiles();
  let next_tile = tiles[rng.gen_range(0..tiles.len())];

  let g = match database::create_game(
    note,
    player0_id,
    player1_id,
    Some(next_tile.to_id()),
    Some(first_player_id),
  ) {
    Ok(g) => g,
    Err(e) => { return Err(e); }
  };

  let mv0 = TMove( TileMove { ord: 0, game_id: g.id, player_id: second_player_id, tile: StartingTile, rot: 0, pos: (0, 0) } );
  let mv1 = MMove( MeepleMove { ord: 1, game_id: g.id, player_id: second_player_id, meeple_id: -1, tile_pos: (0, 0), meeple_pos: -1 });

  match database::create_move(mv0.clone()) {
    Err(e) => { return Err(e) }
    _ => {}
  }
  match database::create_move(mv1.clone()) {
    Err(e) => { return Err(e) }
    _ => {}
  }

  Ok(g)
}

pub fn create_tile_move(game_id: i32, player_id: i32, tile: tile::Tile, rot: i32, pos: (i32, i32)) -> Result<MeepleablePositions, Error> {
  let mut moves = match database::list_moves(game_id) {
    Ok(mvs) => { mvs }
    Err(e) => { return Err(e); }
  };
  assert!(moves.len() != 0);

  let ord = moves.last().unwrap().ord() + 1;

  let mv = TMove( TileMove { ord, game_id, player_id, tile, rot, pos } );
  moves.push(mv.clone());

  let res = calculate::calculate(&moves, false);
  let meepleable_positions = match res {
    Ok(s) => { s.meepleable_positions }
    Err(e) => { return Err(e); }
  };

  match database::create_move(mv) {
    Err(e) => { return Err(e) }
    _ => {}
  }

  Ok(MeepleablePositions {
    meepleable_positions,
  })
}

pub fn create_meeple_move(game_id: i32, player_id: i32, meeple_id: i32, tile_pos: (i32, i32), meeple_pos: i32) -> Result<MeepleMoveResult, Error> {
  let mut rng = rand::thread_rng();
  let mut moves = match database::list_moves(game_id) {
    Ok(mvs) => { mvs }
    Err(e) => { return Err(e); }
  };
  assert!(moves.len() != 0);

  let ord = moves.last().unwrap().ord() + 1;

  let mv = MMove( MeepleMove { ord, game_id, player_id, meeple_id, tile_pos, meeple_pos } );
  moves.push(mv.clone());

  match database::create_move(mv) {
    Err(e) => { return Err(e) }
    _ => {}
  }

  let mut complete_events = vec![];

  let res = calculate::calculate(&moves, false);
  let (player0_point, player1_point) = match res {
    Ok(s) => {
      for e in &s.complete_events {
        complete_events.push(CompleteEvent {
          meeple_ids: e.meeple_ids.clone(),
          feature: e.feature.clone().to_string(),
          point: e.point,
        })
      }
      (s.player0_point, s.player1_point)
    }
    Err(e) => { return Err(e); }
  };

  let mut out_tiles = vec![];
  for mv in moves {
    match mv {
      mov::Move::TMove(tm) => {
        out_tiles.push(tm.tile.clone());
      }
      _ => {}
    }
  }
  let remaining_tiles = tile::remaining_tiles(out_tiles.clone());
  let next_tile = remaining_tiles[rng.gen_range(0..remaining_tiles.len())];

  let gm = match database::get_game(game_id) {
    Ok(game) => {
      game
    }
    Err(e) => {
      return Err(e);
    }
  };

  let next_player_id = if player_id == gm.player0_id { gm.player1_id } else { gm.player0_id };

  match database::update_game(
    game_id,
    next_tile.to_id(),
    next_player_id,
    player0_point,
    player1_point,
  ) {
    Err(e) => { return Err(e); }
    Ok(_) => {}
  }

  Ok(MeepleMoveResult {
    complete_events,
    next_tile_id: next_tile.to_id(),
    next_player_id,
  })
}

pub fn wait_ai_move(game_id: i32) -> Result<MeepleMoveResult, Error> {
  let game = match database::get_game(game_id) {
    Ok(gm) => { gm }
    Err(e) => { return Err(e); }
  };

  let moves = match database::list_moves(game.id) {
    Ok(mvs) => { mvs }
    Err(e) => { return Err(e); }
  };
  assert!(moves.len() != 0);

  let placing_tile = tile::to_tile(game.next_tile_id.unwrap());
  let (tile_move, meeple_move): (TileMove, MeepleMove) = calculate_next_move::calculate_next_move(
    &moves,
    game.id,
    1,
    placing_tile,
  );

  let r = match create_tile_move(game.id, 1, placing_tile, tile_move.rot, tile_move.pos) {
    Ok(res) => { res }
    Err(e) => { return Err(e); }
  };
  assert!(meeple_move.meeple_id == -1 || r.meepleable_positions.contains(&meeple_move.meeple_pos));

  let meeple_move_result = create_meeple_move(game.id, 1, meeple_move.meeple_id, meeple_move.tile_pos, meeple_move.meeple_pos);

  meeple_move_result
}

pub fn get_game(game_id: i32) -> Result<Game, Error> {
  database::get_game(game_id)
}

pub fn get_games(player_id: Option<i32>) -> Result<Vec<Game>, Error> {
  database::get_games(player_id)
}
