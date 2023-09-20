#[macro_use]
extern crate rocket;

mod database;
mod error;
mod event;
mod game;
mod handlers;
mod optimal_move;
mod player;
mod schema;
mod storage;

use handlers::all_options;
use handlers::create_player;
use handlers::events;
use handlers::get_board;
use handlers::get_final_events;
use handlers::get_moves;
use handlers::get_player;
use handlers::get_players;
use handlers::health;
use handlers::send_event;
use handlers::update_player;
use handlers::upload_profile_image;
use handlers::wait_ai_move;
use handlers::{create_discard_move, create_meeple_move, create_tile_move};
use handlers::{create_game, get_game, get_games};

use event::UpdateEvent;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::tokio::sync::broadcast::channel;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
async fn rocket() -> _ {
    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let storage_client = Client::new(&config);

    let r = rocket::build()
        .manage(channel::<UpdateEvent>(1024).0)
        .manage(storage_client)
        .attach(CORS)
        .mount(
            "/",
            routes![
                get_player,
                get_players,
                update_player,
                get_game,
                get_games,
                create_game,
                create_player,
                create_tile_move,
                create_meeple_move,
                create_discard_move,
                wait_ai_move,
                get_moves,
                get_final_events,
                get_board,
                all_options,
                health,
                events,
                send_event,
                upload_profile_image,
            ],
        );
    r
}
