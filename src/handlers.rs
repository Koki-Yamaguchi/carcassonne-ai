use rocket::serde::{Deserialize, json::Json};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use dotenvy::dotenv;

use crate::database;
use crate::game;
use crate::game::tile;
use crate::player;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateTileMove {
  pub game_id: i32,
  pub player_id: i32,
  pub tile_id: i32,
  pub rot: i32,
  pub pos_y: i32,
  pub pos_x: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateMeepleMove {
  pub game_id: i32,
  pub player_id: i32,
  pub meeple_id: i32,
  pub pos: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateGame {
  pub note: String,
  pub player0_id: i32,
  pub player1_id: i32,
}

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
pub fn create_game(params: Json<CreateGame>) -> Json<game::Game> {
  let g = game::create_game(
    params.note.clone(),
    params.player0_id,
    params.player1_id,
  );
  Json(g)
}

#[post("/tile-moves/create", format = "application/json", data = "<params>")]
pub fn create_tile_move(params: Json<CreateTileMove>) -> Json<game::MeepleablePositions> {
  let g = game::create_tile_move(
    params.game_id,
    params.player_id,
    tile::to_tile(params.tile_id),
    params.rot,
    (params.pos_y, params.pos_x),
  );
  Json(g)
}

#[post("/meeple-moves/create", format = "application/json", data = "<params>")]
pub fn create_meeple_move(params: Json<CreateMeepleMove>) -> Json<game::MeepleablePositions> {
  let g = game::create_meeple_move(
    params.game_id,
    params.player_id,
    params.meeple_id,
    params.pos,
  );
  Json(g)
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}