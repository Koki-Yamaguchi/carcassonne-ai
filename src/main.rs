#[macro_use] extern crate rocket;

mod schema;

use rocket::serde::{Deserialize, Serialize, json::Json};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

#[derive(Deserialize, Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
struct Game {
  id: i32,
  note: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::game)]
struct NewGame {
  note: String,
}

#[get("/")]
fn get_games() -> &'static str {
  "Games List Here"
}

#[get("/<game_id>")]
fn get_game(game_id: &str) -> String {
  format!("Game (id: {})", game_id)
}

#[post("/create", format = "application/json", data = "<gm>")]
fn create(gm: Json<Game>) -> Json<Game> {
  let conn = &mut establish_connection();

  let new_game = NewGame{
    note: gm.note.clone(),
  };

  let _: _ = diesel::insert_into(schema::game::table)
      .values(&new_game)
      .get_result::<Game>(conn)
      .expect("Error saving new game");

  Json(Game{ id: gm.id, note: gm.note.clone()})
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/games", routes![
      get_game,
      get_games,
      create,
    ])
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url = {}", &database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
