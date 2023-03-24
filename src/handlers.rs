use rocket::serde::{json::Json};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;

use crate::database;
use crate::game;

#[get("/")]
pub fn get_games() -> &'static str {
  "Games List Here"
}

#[get("/<game_id>")]
pub fn get_game(game_id: &str) -> String {
  format!("Game (id: {})", game_id)
}

#[post("/create", format = "application/json", data = "<gm>")]
pub fn create(gm: Json<game::Game>) -> Json<game::Game> {
  let conn = &mut establish_connection();

  Json(database::create_game(conn, gm.note.clone()))
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url = {}", &database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}