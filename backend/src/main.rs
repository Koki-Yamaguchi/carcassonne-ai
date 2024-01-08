#[macro_use]
extern crate rocket;

mod database;
mod error;
mod event;
mod game;
mod handlers;
mod optimal_move;
mod player;
mod problem;
mod schema;
mod storage;
mod translate;

use event::UpdateEvent;
use handlers::all_options;
use handlers::create_player;
use handlers::create_problem_proposal;
use handlers::events;
use handlers::get_board;
use handlers::get_final_events;
use handlers::get_moves;
use handlers::get_player;
use handlers::get_player_by_uid;
use handlers::get_players;
use handlers::get_problem_proposals;
use handlers::health;
use handlers::send_event;
use handlers::update_player;
use handlers::upload_profile_image;
use handlers::use_problem_proposal;
use handlers::wait_ai_move;
use handlers::{create_discard_move, create_meeple_move, create_tile_move};
use handlers::{create_favorite, get_favorites};
use handlers::{create_game, get_game, get_games};
use handlers::{create_problem, delete_problem, publish_problem, update_problem};
use handlers::{create_vote, get_vote, get_votes};
use handlers::{create_waiting_game, delete_waiting_game, get_waiting_games, update_waiting_game};
use handlers::{get_problem, get_problems};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use dotenvy::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::tokio::sync::broadcast::channel;
use rocket::{Request, Response};
use std::env;
use std::time::Duration;

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
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(5) // FIXME: Didn't think about this number carefully
        .connection_timeout(Duration::from_secs(300))
        .build(manager)
        .expect("Creating a pool failed");

    let region_provider = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let storage_client = Client::new(&config);

    let r = rocket::build()
        .manage(channel::<UpdateEvent>(1024).0)
        .manage(storage_client)
        .manage(pool)
        .attach(CORS)
        .mount(
            "/",
            routes![
                get_player,
                get_player_by_uid,
                get_players,
                update_player,
                get_game,
                get_games,
                create_game,
                get_waiting_games,
                create_waiting_game,
                update_waiting_game,
                delete_waiting_game,
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
                get_problem,
                get_problems,
                create_vote,
                get_vote,
                get_votes,
                create_favorite,
                get_favorites,
                create_problem_proposal,
                use_problem_proposal,
                create_problem,
                update_problem,
                publish_problem,
                delete_problem,
                get_problem_proposals,
            ],
        );
    r
}
