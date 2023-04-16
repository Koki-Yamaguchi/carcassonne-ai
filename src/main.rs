#[macro_use] extern crate rocket;

mod schema;
mod handlers;
mod database;
mod game;
mod player;
mod error;

use handlers::{ get_games, get_game, create_game };
use handlers::{ create_player };
use handlers::{ create_tile_move, create_meeple_move };
use handlers::{ wait_ai_move };

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![
      get_game,
      get_games,
      create_game,
      create_player,
      create_tile_move,
      create_meeple_move,
      wait_ai_move,
    ])
}
