use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema;
use crate::game;
use crate::player;

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
  conn: &mut PgConnection,
  note: String,
  player0_id: i32,
  player1_id: i32,
) -> game::Game {
  let new_game = NewGame{
    note: note,
    player0_id: player0_id,
    player1_id: player1_id,
    player0_point: 0,
    player1_point: 0,
  };
  let r = diesel::insert_into(schema::game::table)
    .values(&new_game)
    .get_result(conn)
    .expect("Error saving new game");
  return r;
}
