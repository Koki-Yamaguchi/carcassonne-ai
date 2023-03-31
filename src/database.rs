use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use dotenvy::dotenv;

use crate::schema;
use crate::game;
use crate::player;
use crate::game::mov;
use crate::game::tile;

#[derive(Insertable)]
#[diesel(table_name = schema::player)]
struct NewPlayer {
  name: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::game)]
struct NewGame {
  note: String,
  player0_id: i32,
  player1_id: i32,
  player0_point: i32,
  player1_point: i32,
  next_tile_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::move_)]
pub struct InsertMove {
  pub ord: i32,
  pub game_id: i32,
  pub player_id: i32,
  pub tile_id: i32,
  pub meeple_id: i32,
  pub rot: i32,
  pub tile_pos_y: i32,
  pub tile_pos_x: i32,
  pub meeple_pos: i32,
}
#[derive(Queryable)]
#[diesel(table_name = schema::move_)]
pub struct QueryMove {
  pub id: i32,
  pub ord: i32,
  pub game_id: i32,
  pub player_id: i32,
  pub tile_id: i32,
  pub meeple_id: i32,
  pub rot: i32,
  pub tile_pos_y: i32,
  pub tile_pos_x: i32,
  pub meeple_pos: i32,
}

pub fn create_player(conn: &mut PgConnection, name: String) -> player::Player {
  let new_player = NewPlayer{
    name: name,
  };
  let r = diesel::insert_into(schema::player::table)
    .values(&new_player)
    .get_result(conn)
    .expect("Error saving new player");
  return r;
}

pub fn create_game(
  note: String,
  player0_id: i32,
  player1_id: i32,
  next_tile_id: Option<i32>,
) -> game::Game {
  let new_game = NewGame{
    note: note,
    player0_id: player0_id,
    player1_id: player1_id,
    player0_point: 0,
    player1_point: 0,
    next_tile_id: next_tile_id,
  };
  let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
  let r = diesel::insert_into(schema::game::table)
    .values(&new_game)
    .get_result(conn)
    .expect("Error saving new game");
  return r;
}

pub fn list_moves(gmid: i32) -> Vec<mov::Move> {
  let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
  use self::schema::move_::dsl::*;
  let results = move_
    .filter(game_id.eq(gmid))
    .order(ord.asc())
    .load::<QueryMove>(conn)
    .expect("Error loading posts");
  results.into_iter().map(|v| to_move(v)).collect()
}

pub fn create_move(mv: mov::Move) -> mov::Move {
  let conn = &mut establish_connection(); // FIXME: establish connection once, not every time

  let new_move = match mv {
    mov::Move::TMove(m) => InsertMove {
      ord: m.ord,
      game_id: m.game_id,
      player_id: m.player_id,
      tile_id: m.tile.to_id(),
      meeple_id: -1,
      rot: m.rot,
      tile_pos_y: m.pos.0,
      tile_pos_x: m.pos.1,
      meeple_pos: -1,
    },
    mov::Move::MMove(m) => InsertMove {
      ord: m.ord,
      game_id: m.game_id,
      player_id: m.player_id,
      tile_id: -1,
      meeple_id: m.meeple_id,
      rot: -1,
      tile_pos_y: m.tile_pos.0,
      tile_pos_x: m.tile_pos.1,
      meeple_pos: m.meeple_pos,
    },
    mov::Move::InvalidMove => {
      return mov::Move::InvalidMove
    }
  };

  let query_move = diesel::insert_into(schema::move_::table)
    .values(&new_move)
    .get_result::<QueryMove>(conn)
    .expect("Error saving new move");

  to_move(query_move)
}

fn to_move(qm: QueryMove) -> mov::Move {
  match (qm.tile_id, qm.meeple_id) {
    (-1, _) => mov::Move::MMove (mov::MeepleMove {
      ord: qm.ord,
      game_id: qm.game_id,
      player_id: qm.player_id,
      meeple_id: qm.meeple_id,
      meeple_pos: qm.meeple_pos,
      tile_pos: (qm.tile_pos_y, qm.tile_pos_x),
    }),
    (_, -1) => mov::Move::TMove (mov::TileMove {
      ord: qm.ord,
      game_id: qm.game_id,
      player_id: qm.player_id,
      tile: tile::to_tile(qm.tile_id),
      rot: qm.rot,
      pos: (qm.tile_pos_y, qm.tile_pos_x),
    }),
    (_, _) => mov::Move::InvalidMove
  }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url = {}", &database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}