#[macro_use] extern crate rocket;

mod schema;
mod handlers;
mod database;
mod game;
mod player;

use handlers::{ get_games, get_game, create_game };
use handlers::{ create_player, create_tile_move };

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![
      get_game,
      get_games,
      create_game,
      create_player,
      create_tile_move,
    ])
}
