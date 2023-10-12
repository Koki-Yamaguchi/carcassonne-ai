use std::path::Path;
use std::thread;

use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use rocket::http::{ContentType, Status};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::{json::to_string, json::Json, Deserialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::Data;
use rocket::{Shutdown, State};
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use crate::database;
use crate::event;
use crate::game;
use crate::game::tile;
use crate::player;
use crate::problem;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateTileMove {
    pub game_id: Option<i32>,
    pub player_id: i32,
    pub tile_id: i32,
    pub rot: i32,
    pub pos_y: i32,
    pub pos_x: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateMeepleMove {
    pub game_id: Option<i32>,
    pub player_id: i32,
    pub meeple_id: i32,
    pub pos: i32,
    pub tile_pos_y: i32,
    pub tile_pos_x: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateDiscardMove {
    pub game_id: Option<i32>,
    pub player_id: i32,
    pub tile_id: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct WaitAIMove {
    pub game_id: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateGame {
    pub player0_id: i32,
    pub player0_color: i32,
    pub player1_id: i32,
    pub player1_color: i32,
    pub is_rated: Option<bool>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateWaitingGame {
    pub player_id: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteWaitingGame {
    pub player_id: i32,
}

#[get("/players/<player_id>", format = "application/json")]
pub async fn get_player(
    storage_client: &State<Client>,
    player_id: i32,
) -> (Status, (ContentType, String)) {
    match player::get_player(storage_client, player_id).await {
        Ok(player) => (Status::Ok, (ContentType::JSON, to_string(&player).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/players?<user>", format = "application/json")]
pub async fn get_player_by_uid(
    storage_client: &State<Client>,
    user: String,
) -> (Status, (ContentType, String)) {
    match player::get_player_by_uid(storage_client, user).await {
        Ok(player) => (Status::Ok, (ContentType::JSON, to_string(&player).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/players", format = "application/json")]
pub async fn get_players(storage_client: &State<Client>) -> (Status, (ContentType, String)) {
    match player::get_players(storage_client).await {
        Ok(players) => (
            Status::Ok,
            (ContentType::JSON, to_string(&players).unwrap()),
        ),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post(
    "/players/<player_id>/update",
    format = "application/json",
    data = "<params>"
)]
pub fn update_player(
    player_id: i32,
    params: Json<player::UpdatePlayer>,
) -> (Status, (ContentType, String)) {
    match player::update_player(player_id, params.name.clone(), params.meeple_color) {
        Ok(player) => (Status::Ok, (ContentType::JSON, to_string(&player).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/players/create", format = "application/json", data = "<params>")]
pub fn create_player(params: Json<player::CreatePlayer>) -> (Status, (ContentType, String)) {
    match database::create_player(
        params.name.clone(),
        params.email.clone(),
        params.user_id.clone(),
        0, // red
    ) {
        Ok(player) => (Status::Ok, (ContentType::JSON, to_string(&player).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/games?<player>&<is_rated>&<limit>", format = "application/json")]
pub fn get_games(
    player: Option<i32>,
    is_rated: Option<bool>,
    limit: Option<i32>,
) -> (Status, (ContentType, String)) {
    match game::get_games(player, is_rated, limit) {
        Ok(games) => (Status::Ok, (ContentType::JSON, to_string(&games).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/games/<game_id>")]
pub fn get_game(game_id: i32) -> (Status, (ContentType, String)) {
    match game::get_game(game_id) {
        Ok(game) => (Status::Ok, (ContentType::JSON, to_string(&game).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/games/create", format = "application/json", data = "<params>")]
pub fn create_game(params: Json<CreateGame>) -> (Status, (ContentType, String)) {
    let is_rated = match params.is_rated {
        Some(ir) => ir,
        None => false,
    };
    match game::create_game(
        params.player0_id,
        params.player1_id,
        params.player0_color,
        params.player1_color,
        is_rated,
    ) {
        Ok(game) => (Status::Ok, (ContentType::JSON, to_string(&game).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post(
    "/waiting-games/delete",
    format = "application/json",
    data = "<params>"
)]
pub fn delete_waiting_game(params: Json<DeleteWaitingGame>) -> (Status, (ContentType, String)) {
    match game::delete_waiting_games(params.player_id) {
        Ok(games) => (Status::Ok, (ContentType::JSON, to_string(&games).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}
#[post(
    "/waiting-games/create",
    format = "application/json",
    data = "<params>"
)]
pub fn create_waiting_game(params: Json<CreateWaitingGame>) -> (Status, (ContentType, String)) {
    match game::create_waiting_game(params.player_id) {
        Ok(games) => (Status::Ok, (ContentType::JSON, to_string(&games).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}
#[get("/waiting-games", format = "application/json")]
pub fn get_waiting_games() -> (Status, (ContentType, String)) {
    match game::get_waiting_games() {
        Ok(games) => (Status::Ok, (ContentType::JSON, to_string(&games).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}
#[post(
    "/waiting-games/<id>/update",
    format = "application/json",
    data = "<params>"
)]
pub fn update_waiting_game(
    id: Option<i32>,
    params: Json<game::UpdateWaitingGame>,
) -> (Status, (ContentType, String)) {
    match game::update_waiting_game(id.unwrap(), params.game_id) {
        Ok(games) => (Status::Ok, (ContentType::JSON, to_string(&games).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/tile-moves/create", format = "application/json", data = "<params>")]
pub fn create_tile_move(
    params: Json<CreateTileMove>,
    queue: &State<Sender<event::UpdateEvent>>,
) -> (Status, (ContentType, String)) {
    let r = game::create_tile_move(
        params.game_id,
        params.player_id,
        tile::to_tile(params.tile_id),
        params.rot,
        (params.pos_y, params.pos_x),
    );

    match r {
        Ok(res) => {
            if let Some(gid) = params.game_id {
                let _ = queue.send(event::UpdateEvent {
                    name: "update_game".to_string(),
                    id: gid,
                });
            }
            (Status::Ok, (ContentType::JSON, to_string(&res).unwrap()))
        }
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/meeple-moves/create", format = "application/json", data = "<params>")]
pub fn create_meeple_move(
    params: Json<CreateMeepleMove>,
    queue: &State<Sender<event::UpdateEvent>>,
) -> (Status, (ContentType, String)) {
    let r = game::create_meeple_move(
        params.game_id,
        params.player_id,
        params.meeple_id,
        (params.tile_pos_y, params.tile_pos_x),
        params.pos,
    );
    match r {
        Ok(res) => {
            if let Some(gid) = params.game_id {
                let _ = queue.send(event::UpdateEvent {
                    name: "update_game".to_string(),
                    id: gid,
                });
            }
            (Status::Ok, (ContentType::JSON, to_string(&res).unwrap()))
        }
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post(
    "/discard-moves/create",
    format = "application/json",
    data = "<params>"
)]
pub fn create_discard_move(params: Json<CreateDiscardMove>) -> (Status, (ContentType, String)) {
    match game::create_discard_move(
        params.game_id,
        params.player_id,
        tile::to_tile(params.tile_id),
    ) {
        Ok(res) => (Status::Ok, (ContentType::JSON, to_string(&res).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/wait-ai-move", format = "application/json", data = "<params>")]
pub fn wait_ai_move(params: Json<WaitAIMove>, queue: &State<Sender<event::UpdateEvent>>) {
    let q = queue.inner().clone();
    thread::spawn(move || match game::wait_ai_move(params.game_id) {
        Ok(_) => {
            let _ = q.send(event::UpdateEvent {
                name: "update_game".to_string(),
                id: params.game_id,
            });
        }
        Err(_) => {}
    });
}

#[get("/moves?<game>&<m>", format = "application/json")]
pub fn get_moves(game: Option<i32>, m: Option<i32>) -> (Status, (ContentType, String)) {
    match game::get_moves(game, m) {
        Ok(moves) => (Status::Ok, (ContentType::JSON, to_string(&moves).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/final-events?<game>", format = "application/json")]
pub fn get_final_events(game: Option<i32>) -> (Status, (ContentType, String)) {
    match game::get_final_events(game) {
        Ok(events) => (Status::Ok, (ContentType::JSON, to_string(&events).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/board?<game>&<m>", format = "application/json")]
pub fn get_board(game: Option<i32>, m: Option<i32>) -> (Status, (ContentType, String)) {
    match game::get_board(game, m) {
        Ok(board) => (Status::Ok, (ContentType::JSON, to_string(&board).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[options("/<_..>")]
pub fn all_options() {}

#[get("/health", format = "application/json")]
pub fn health() -> (Status, (ContentType, String)) {
    (Status::Ok, (ContentType::JSON, "".to_string()))
}

#[get("/events?<name>&<id>")]
pub async fn events(
    name: Option<String>,
    id: Option<i32>,
    queue: &State<Sender<event::UpdateEvent>>,
    mut end: Shutdown,
) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            if msg.name == name.clone().unwrap() && msg.id == id.unwrap() {
                yield Event::json(&msg);
            }
        }
    }
}

#[post("/send-event", format = "application/json", data = "<params>")]
pub fn send_event(params: Json<event::UpdateEvent>, queue: &State<Sender<event::UpdateEvent>>) {
    let _ = queue.send(event::UpdateEvent {
        name: params.name.clone(),
        id: params.id,
    });
}

#[post("/players/<player_id>/upload-profile-image", data = "<data>")]
pub async fn upload_profile_image(
    content_type: &ContentType,
    storage_client: &State<Client>,
    player_id: i32,
    data: Data<'_>,
) {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("profile_image")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    ]);

    let multipart_form_data = MultipartFormData::parse(content_type, data, options)
        .await
        .unwrap();

    let profile_image = multipart_form_data.files.get("profile_image");
    if let Some(file_fields) = profile_image {
        let file_field = &file_fields[0];
        let path = &file_field.path;
        let body = ByteStream::from_path(Path::new(path)).await.unwrap();

        player::upload_profile_image(storage_client, player_id, body).await
    }
}

#[get("/problems/<id>", format = "application/json")]
pub async fn get_problem(id: Option<i32>) -> (Status, (ContentType, String)) {
    match problem::get_problem(id.unwrap()) {
        Ok(p) => (Status::Ok, (ContentType::JSON, to_string(&p).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/problems", format = "application/json")]
pub async fn get_problems() -> (Status, (ContentType, String)) {
    match problem::get_problems() {
        Ok(ps) => (Status::Ok, (ContentType::JSON, to_string(&ps).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/votes/create", format = "application/json", data = "<params>")]
pub fn create_vote(params: Json<problem::CreateVote>) -> (Status, (ContentType, String)) {
    match problem::create_vote(
        params.problem_id,
        params.player_id,
        params.player_name.clone(),
        params.note.clone(),
        params.tile_move_id,
        params.meeple_move_id,
    ) {
        Ok(v) => (Status::Ok, (ContentType::JSON, to_string(&v).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/votes/<id>", format = "application/json")]
pub fn get_vote(id: Option<i32>) -> (Status, (ContentType, String)) {
    match problem::get_vote(id.unwrap()) {
        Ok(v) => (Status::Ok, (ContentType::JSON, to_string(&v).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/votes?<problem>", format = "application/json")]
pub fn get_votes(problem: Option<i32>) -> (Status, (ContentType, String)) {
    match problem::get_votes(problem) {
        Ok(vs) => (Status::Ok, (ContentType::JSON, to_string(&vs).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/favorites/create", format = "application/json", data = "<params>")]
pub fn create_favorite(params: Json<problem::CreateFavorite>) -> (Status, (ContentType, String)) {
    match problem::create_favorite(params.vote_id, params.player_id, params.player_name.clone()) {
        Ok(f) => (Status::Ok, (ContentType::JSON, to_string(&f).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/favorites?<vote>&<player>", format = "application/json")]
pub fn get_favorites(vote: Option<i32>, player: Option<i32>) -> (Status, (ContentType, String)) {
    match problem::get_favorites(vote, player) {
        Ok(fs) => (Status::Ok, (ContentType::JSON, to_string(&fs).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}
