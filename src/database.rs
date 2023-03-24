use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema;
use crate::game;

#[derive(Insertable)]
#[diesel(table_name = schema::game)]
struct NewGame {
  note: String,
}

pub fn create_game(conn: &mut PgConnection, note: String) -> game::Game {
  let new_game = NewGame{
    note: note,
  };
  let r = diesel::insert_into(schema::game::table)
    .values(&new_game)
    .get_result(conn)
    .expect("Error saving new game");
  return r;
}
