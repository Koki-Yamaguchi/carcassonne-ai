use rocket::serde::{Deserialize, json::Json, json::to_string};
use rocket::http::{Status, ContentType};

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
  pub tile_pos_y: i32,
  pub tile_pos_x: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct WaitAIMove {
  pub game_id: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateGame {
  pub note: String,
  pub player0_id: i32,
  pub player1_id: i32,
}

#[post("/players/create", format = "application/json", data = "<params>")]
pub fn create_player(params: Json<player::CreatePlayer>) -> (Status, (ContentType, String)) {
  match database::create_player(params.name.clone()) {
    Ok(player) => {
      (Status::Ok, (ContentType::JSON, to_string(&player).unwrap()))
    }
    Err(e) => {
      (e.status, (ContentType::JSON, to_string(&e.detail).unwrap()))
    }
  }
}

#[get("/games?<player>", format = "application/json")]
pub fn get_games(player: Option<i32>) -> (Status, (ContentType, String)) {
  match game::get_games(player) {
    Ok(games) => {
      (Status::Ok, (ContentType::JSON, to_string(&games).unwrap()))
    }
    Err(e) => {
      (e.status, (ContentType::JSON, to_string(&e.detail).unwrap()))
    }
  }
}

#[get("/games/<game_id>")]
pub fn get_game(game_id: i32) -> (Status, (ContentType, String)) {
  match game::get_game(game_id) {
    Ok(game) => {
      (Status::Ok, (ContentType::JSON, to_string(&game).unwrap()))
    }
    Err(e) => {
      (e.status, (ContentType::JSON, to_string(&e.detail).unwrap()))
    }
  }
}

#[post("/games/create", format = "application/json", data = "<params>")]
pub fn create_game(params: Json<CreateGame>) -> (Status, (ContentType, String)) {
  match game::create_game(
    params.note.clone(),
    params.player0_id,
    params.player1_id,
  ) {
    Ok(game) => {
      (Status::Ok, (ContentType::JSON, to_string(&game).unwrap()))
    }
    Err(e) => {
      (e.status, (ContentType::JSON, to_string(&e.detail).unwrap()))
    }
  }
}

#[post("/tile-moves/create", format = "application/json", data = "<params>")]
pub fn create_tile_move(params: Json<CreateTileMove>) -> (Status, (ContentType, String)) {
  match game::create_tile_move(
    params.game_id,
    params.player_id,
    tile::to_tile(params.tile_id),
    params.rot,
    (params.pos_y, params.pos_x),
  ) {
    Ok(res) => {
      (Status::Ok, (ContentType::JSON, to_string(&res).unwrap()))
    }
    Err(e) => {
      (e.status, (ContentType::JSON, to_string(&e.detail).unwrap()))
    }
  }
}

#[post("/meeple-moves/create", format = "application/json", data = "<params>")]
pub fn create_meeple_move(params: Json<CreateMeepleMove>) -> (Status, (ContentType, String)) {
  match game::create_meeple_move(
    params.game_id,
    params.player_id,
    params.meeple_id,
    (params.tile_pos_y, params.tile_pos_x),
    params.pos,
  ) {
    Ok(res) => {
      (Status::Ok, (ContentType::JSON, to_string(&res).unwrap()))
    }
    Err(e) => {
      (e.status, (ContentType::JSON, to_string(&e.detail).unwrap()))
    }
  }
}

#[post("/wait-ai-move", format = "application/json", data = "<params>")]
pub fn wait_ai_move(params: Json<WaitAIMove>) -> (Status, (ContentType, String)) {
  match game::wait_ai_move(params.game_id) {
    Ok(res) => {
      (Status::Ok, (ContentType::JSON, to_string(&res).unwrap()))
    }
    Err(e) => {
      (e.status, (ContentType::JSON, to_string(&e.detail).unwrap()))
    }
  }
}

#[get("/moves?<game>", format = "application/json")]
pub fn get_moves(game: Option<i32>) -> (Status, (ContentType, String)) {
  match game::get_moves(game) {
    Ok(moves) => {
      (Status::Ok, (ContentType::JSON, to_string(&moves).unwrap()))
    }
    Err(e) => {
      (e.status, (ContentType::JSON, to_string(&e.detail).unwrap()))
    }
  }
}

#[options("/<_..>")]
pub fn all_options() {}
