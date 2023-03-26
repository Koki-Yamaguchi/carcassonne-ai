use rocket::serde::{json::Json};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use dotenvy::dotenv;

use crate::database;
use crate::game;
use crate::player;

#[post("/players/create", format = "application/json", data = "<params>")]
pub fn create_player(params: Json<player::CreatePlayer>) -> Json<player::Player> {
  let conn = &mut establish_connection();

  Json(database::create_player(conn, params.name.clone()))
}

#[get("/games")]
pub fn get_games() -> &'static str {
  "Games List Here"
}

#[get("/games/<game_id>")]
pub fn get_game(game_id: &str) -> String {
  format!("Game (id: {})", game_id)
}

#[post("/games/create", format = "application/json", data = "<params>")]
pub fn create_game(params: Json<game::CreateGame>) -> Json<game::Game> {
  let g = game::create_game(
    params.note.clone(),
    params.player0_id,
    params.player1_id,
  );
  Json(g)
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url = {}", &database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}