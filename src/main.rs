#[macro_use] extern crate rocket;

mod schema;
mod handlers;
mod database;
mod game;

use handlers::{ get_games, get_game, create };

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/games", routes![
      get_game,
      get_games,
      create,
    ])
}
