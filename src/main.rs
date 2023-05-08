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
use handlers::{ get_moves };
use handlers::{ all_options };

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .attach(CORS)
    .mount("/", routes![
      get_game,
      get_games,
      create_game,
      create_player,
      create_tile_move,
      create_meeple_move,
      wait_ai_move,
      get_moves,
      all_options,
    ])
}
