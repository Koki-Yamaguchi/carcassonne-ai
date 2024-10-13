use std::path::Path;
use std::thread;

use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
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

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

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
pub struct CreateDiscardMove {
    pub game_id: Option<i32>,
    pub player_id: i32,
    pub tile_id: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateMove {
    pub game_id: Option<i32>,
    pub player_id: i32,
    pub tile_id: i32,
    pub rot: i32,
    pub pos_y: i32,
    pub pos_x: i32,
    pub meeple_id: i32,
    pub meeple_pos: i32,
    pub wait_ai_move: bool,
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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SendEvent {
    pub id: i32,
    pub name: String,
}

#[get("/players/<player_id>", format = "application/json")]
pub fn get_player(player_id: i32, db: &State<DbPool>) -> (Status, (ContentType, String)) {
    match player::get_player(db.inner(), player_id) {
        Ok(player) => (Status::Ok, (ContentType::JSON, to_string(&player).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/players?<user>&<name>", format = "application/json")]
pub fn get_players(
    user: Option<String>,
    name: Option<String>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match player::get_players(user, name, db.inner()) {
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
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match player::update_player(
        db.inner(),
        player_id,
        params.name.clone(),
        params.meeple_color,
        params.tile_edition.clone(),
    ) {
        Ok(player) => (Status::Ok, (ContentType::JSON, to_string(&player).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/players/create", format = "application/json", data = "<params>")]
pub fn create_player(
    params: Json<player::CreatePlayer>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match database::create_player(
        db.inner(),
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
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match game::get_games(db.inner(), player, is_rated, limit) {
        Ok(games) => (Status::Ok, (ContentType::JSON, to_string(&games).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/games/<game_id>")]
pub fn get_game(game_id: i32, db: &State<DbPool>) -> (Status, (ContentType, String)) {
    match game::get_game(db.inner(), game_id) {
        Ok(game) => (Status::Ok, (ContentType::JSON, to_string(&game).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/games/create", format = "application/json", data = "<params>")]
pub fn create_game(
    params: Json<CreateGame>,
    queue: &State<Sender<event::Event>>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    let is_rated = match params.is_rated {
        Some(ir) => ir,
        None => false,
    };
    let cloned_db = db.inner().clone();
    let q = queue.inner().clone();

    match game::create_game(
        db.inner(),
        params.player0_id,
        params.player1_id,
        params.player0_color,
        params.player1_color,
        is_rated,
    ) {
        Ok(game) => {
            if game.current_player_id == Some(1) {
                thread::spawn(move || {
                    thread::sleep(std::time::Duration::from_secs(1));

                    let (mvs, complete_events) = game::wait_ai_move(&cloned_db, game.id).unwrap();

                    // tile move, meeple move
                    if mvs.len() == 2 {
                        match (&mvs[0], &mvs[1]) {
                            (game::mov::Move::TMove(tm), game::mov::Move::MMove(mm)) => {
                                let _ = q.send(event::Event {
                                    id: game.id,
                                    name: "move_created_event".to_string(),
                                    player_id: tm.player_id,
                                    tile: tm.tile,
                                    rot: tm.rot,
                                    tile_pos: tm.pos,
                                    meeple_id: mm.meeple_id,
                                    meeple_pos: mm.meeple_pos,
                                    complete_events,
                                });
                            }
                            _ => {
                                panic!("invalid event");
                            }
                        }
                    } else if mvs.len() == 1 {
                        match &mvs[0] {
                            game::mov::Move::DMove(dm) => {
                                let _ = q.send(event::Event {
                                    id: game.id,
                                    name: "move_created_event".to_string(),
                                    player_id: dm.player_id,
                                    tile: dm.tile,
                                    rot: -1,
                                    tile_pos: (-1, -1),
                                    meeple_id: -1,
                                    meeple_pos: -1,
                                    complete_events,
                                });
                            }
                            _ => {
                                panic!("invalid event");
                            }
                        }
                    }
                });
            }

            (Status::Ok, (ContentType::JSON, to_string(&game).unwrap()))
        }
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post(
    "/waiting-games/delete",
    format = "application/json",
    data = "<params>"
)]
pub fn delete_waiting_game(
    params: Json<DeleteWaitingGame>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match game::delete_waiting_games(db.inner(), params.player_id) {
        Ok(games) => (Status::Ok, (ContentType::JSON, to_string(&games).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}
#[post(
    "/waiting-games/create",
    format = "application/json",
    data = "<params>"
)]
pub fn create_waiting_game(
    params: Json<CreateWaitingGame>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match game::create_waiting_game(db.inner(), params.player_id) {
        Ok(games) => (Status::Ok, (ContentType::JSON, to_string(&games).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}
#[get("/waiting-games", format = "application/json")]
pub fn get_waiting_games(db: &State<DbPool>) -> (Status, (ContentType, String)) {
    match game::get_waiting_games(db.inner()) {
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
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match game::update_waiting_game(db.inner(), id.unwrap(), params.game_id) {
        Ok(games) => (Status::Ok, (ContentType::JSON, to_string(&games).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post(
    "/tile-moves/try-create",
    format = "application/json",
    data = "<params>"
)]
pub fn try_create_tile_move(
    params: Json<CreateTileMove>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    let r = game::try_create_tile_move(
        db.inner(),
        params.game_id,
        params.player_id,
        tile::to_tile(params.tile_id),
        params.rot,
        (params.pos_y, params.pos_x),
    );

    match r {
        Ok(res) => (Status::Ok, (ContentType::JSON, to_string(&res).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post(
    "/discard-moves/create",
    format = "application/json",
    data = "<params>"
)]
pub fn create_discard_move(
    params: Json<CreateDiscardMove>,
    queue: &State<Sender<event::Event>>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match game::create_discard_move(
        db.inner(),
        params.game_id,
        params.player_id,
        tile::to_tile(params.tile_id),
    ) {
        Ok(res) => {
            let _ = queue.send(event::Event {
                id: params.game_id.unwrap(),
                name: "move_created_event".to_string(),
                player_id: params.player_id,
                tile: tile::to_tile(params.tile_id),
                rot: -1,
                tile_pos: (-1, -1),
                meeple_id: -1,
                meeple_pos: -1,
                complete_events: vec![],
            });
            (Status::Ok, (ContentType::JSON, to_string(&res).unwrap()))
        }
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/moves/create", format = "application/json", data = "<params>")]
pub fn create_move(
    params: Json<CreateMove>,
    queue: &State<Sender<event::Event>>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    let q = queue.inner().clone();
    let cloned_db = db.inner().clone();
    let r = game::create_move(
        db.inner(),
        params.game_id,
        params.player_id,
        tile::to_tile(params.tile_id),
        params.rot,
        (params.pos_y, params.pos_x),
        params.meeple_id,
        params.meeple_pos,
    );
    match r {
        Ok(res) => {
            if let Some(gid) = params.game_id {
                match (&res.tile_move, &res.meeple_move) {
                    (game::mov::Move::TMove(tm), game::mov::Move::MMove(mm)) => {
                        let _ = q.send(event::Event {
                            id: gid,
                            name: "move_created_event".to_string(),
                            player_id: tm.player_id,
                            tile: tm.tile,
                            rot: tm.rot,
                            tile_pos: tm.pos,
                            meeple_id: mm.meeple_id,
                            meeple_pos: mm.meeple_pos,
                            complete_events: res.complete_events.clone(),
                        });

                        if params.wait_ai_move {
                            thread::spawn(move || loop {
                                let (mvs, complete_events) =
                                    game::wait_ai_move(&cloned_db, gid).unwrap();
                                if mvs.len() == 2 {
                                    match (&mvs[0], &mvs[1]) {
                                        (
                                            game::mov::Move::TMove(tm),
                                            game::mov::Move::MMove(mm),
                                        ) => {
                                            let _ = q.send(event::Event {
                                                id: gid,
                                                name: "move_created_event".to_string(),
                                                player_id: tm.player_id,
                                                tile: tm.tile,
                                                rot: tm.rot,
                                                tile_pos: tm.pos,
                                                meeple_id: mm.meeple_id,
                                                meeple_pos: mm.meeple_pos,
                                                complete_events,
                                            });
                                        }
                                        _ => {
                                            panic!("invalid response");
                                        }
                                    }
                                    break;
                                } else if mvs.len() == 1 {
                                    match &mvs[0] {
                                        game::mov::Move::DMove(dm) => {
                                            let _ = q.send(event::Event {
                                                id: gid,
                                                name: "move_created_event".to_string(),
                                                player_id: dm.player_id,
                                                tile: dm.tile,
                                                rot: -1,
                                                tile_pos: (-1, -1),
                                                meeple_id: -1,
                                                meeple_pos: -1,
                                                complete_events,
                                            });
                                        }
                                        _ => {
                                            panic!("invalid response");
                                        }
                                    }
                                }
                            });
                        }
                    }
                    _ => {
                        panic!("invalid response");
                    }
                }
            }
            (Status::Ok, (ContentType::JSON, to_string(&res).unwrap()))
        }
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/moves?<game>&<m>", format = "application/json")]
pub fn get_moves(
    game: Option<i32>,
    m: Option<i32>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match game::get_moves(db.inner(), game, m) {
        Ok(moves) => (Status::Ok, (ContentType::JSON, to_string(&moves).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/final-events?<game>", format = "application/json")]
pub fn get_final_events(game: Option<i32>, db: &State<DbPool>) -> (Status, (ContentType, String)) {
    match game::get_final_events(db.inner(), game) {
        Ok(events) => (Status::Ok, (ContentType::JSON, to_string(&events).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/board?<game>&<m>", format = "application/json")]
pub fn get_board(
    game: Option<i32>,
    m: Option<i32>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match game::get_board(db.inner(), game, m) {
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
    queue: &State<Sender<event::Event>>,
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
pub fn send_event(params: Json<SendEvent>, queue: &State<Sender<event::Event>>) {
    let _ = queue.send(event::Event {
        id: params.id,
        name: params.name.clone(),
        player_id: -1,
        tile: tile::Tile::Invalid,
        rot: -1,
        tile_pos: (-1, -1),
        meeple_id: -1,
        meeple_pos: -1,
        complete_events: vec![],
    });
}

#[post("/players/<player_id>/upload-profile-image", data = "<data>")]
pub async fn upload_profile_image(
    content_type: &ContentType,
    storage_client: &State<Client>,
    player_id: i32,
    data: Data<'_>,
    db: &State<DbPool>,
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

        player::upload_profile_image(db.inner(), storage_client, player_id, body)
            .await
            .unwrap();
    }
}

#[get("/problems/<id>?<player>", format = "application/json")]
pub async fn get_problem(
    id: Option<i32>,
    player: Option<i32>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match problem::get_problem(db.inner(), id.unwrap(), player) {
        Ok(p) => (Status::Ok, (ContentType::JSON, to_string(&p).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get(
    "/problems?<page>&<order_by>&<limit>&<creator>&<is_draft>&<is_private>&<player>",
    format = "application/json"
)]
pub fn get_problems(
    page: Option<i32>,
    order_by: Option<String>,
    limit: Option<i32>,
    creator: Option<i32>,
    is_draft: Option<bool>,
    is_private: Option<bool>,
    player: Option<i32>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match problem::get_problems(
        db.inner(),
        page,
        order_by,
        limit,
        creator,
        is_draft,
        is_private,
        player,
    ) {
        Ok(ps) => (Status::Ok, (ContentType::JSON, to_string(&ps).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/votes/create", format = "application/json", data = "<params>")]
pub fn create_vote(
    params: Json<problem::CreateVote>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    let cloned_db = db.inner().clone();
    match problem::create_vote(
        db.inner(),
        params.problem_id,
        params.player_id,
        params.player_name.clone(),
        params.note.clone(),
        params.tile_move_id,
        params.meeple_move_id,
    ) {
        Ok(v) => {
            thread::spawn(move || problem::update_vote_translation(&cloned_db, v.id));

            (Status::Ok, (ContentType::JSON, to_string(&v).unwrap()))
        }
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/votes/<id>", format = "application/json")]
pub fn get_vote(id: Option<i32>, db: &State<DbPool>) -> (Status, (ContentType, String)) {
    match problem::get_vote(db.inner(), id.unwrap()) {
        Ok(v) => (Status::Ok, (ContentType::JSON, to_string(&v).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/votes?<problem>&<player>", format = "application/json")]
pub fn get_votes(
    problem: Option<i32>,
    player: Option<i32>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match problem::get_votes(db.inner(), problem, player) {
        Ok(vs) => (Status::Ok, (ContentType::JSON, to_string(&vs).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/favorites/create", format = "application/json", data = "<params>")]
pub fn create_favorite(
    params: Json<problem::CreateFavorite>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match problem::create_favorite(
        db.inner(),
        params.problem_id,
        params.player_id,
        params.player_name.clone(),
    ) {
        Ok(f) => (Status::Ok, (ContentType::JSON, to_string(&f).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/favorites/delete", format = "application/json", data = "<params>")]
pub fn delete_favorite(
    params: Json<problem::DeleteFavorite>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match problem::delete_favorite(db.inner(), params.problem_id, params.player_id) {
        Ok(v) => (Status::Ok, (ContentType::JSON, to_string(&v).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/favorites?<vote>&<player>", format = "application/json")]
pub fn get_favorites(
    vote: Option<i32>,
    player: Option<i32>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match problem::get_favorites(db.inner(), vote, player) {
        Ok(fs) => (Status::Ok, (ContentType::JSON, to_string(&fs).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/problems/create", format = "application/json", data = "<params>")]
pub fn create_problem(
    params: Json<problem::CreateProblem>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match problem::create_draft_problem(db.inner(), &params) {
        Ok(v) => (Status::Ok, (ContentType::JSON, to_string(&v).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post(
    "/problems/<id>/update",
    format = "application/json",
    data = "<params>"
)]
pub fn update_problem(
    id: i32,
    params: Json<problem::UpdateProblem>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match problem::update_problem(db.inner(), id, &params) {
        Ok(v) => (Status::Ok, (ContentType::JSON, to_string(&v).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post(
    "/problems/<id>/publish",
    format = "application/json",
    data = "<params>"
)]
pub fn publish_problem(
    id: i32,
    params: Json<problem::PublishProblem>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match problem::publish_problem(db.inner(), id, &params) {
        Ok(v) => (Status::Ok, (ContentType::JSON, to_string(&v).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/problems/<id>/delete", format = "application/json")]
pub fn delete_problem(id: i32, db: &State<DbPool>) -> (Status, (ContentType, String)) {
    match problem::delete_problem(db.inner(), id) {
        Ok(v) => (Status::Ok, (ContentType::JSON, to_string(&v).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/problem-proposals?<player>", format = "application/json")]
pub fn get_problem_proposals(
    db: &State<DbPool>,
    player: Option<i32>,
) -> (Status, (ContentType, String)) {
    match problem::get_problem_proposals(db.inner(), player) {
        Ok(vs) => (Status::Ok, (ContentType::JSON, to_string(&vs).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post(
    "/problem-proposals/create",
    format = "application/json",
    data = "<params>"
)]
pub fn create_problem_proposal(
    params: Json<problem::CreateProblemProposal>,
    db: &State<DbPool>,
) -> (Status, (ContentType, String)) {
    match problem::create_problem_proposal(db.inner(), &params) {
        Ok(v) => (Status::Ok, (ContentType::JSON, to_string(&v).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[post("/problem-proposals/<id>/use", format = "application/json")]
pub fn use_problem_proposal(id: i32, db: &State<DbPool>) -> (Status, (ContentType, String)) {
    match problem::use_problem_proposal(db.inner(), id) {
        Ok(v) => (Status::Ok, (ContentType::JSON, to_string(&v).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}

#[get("/creators", format = "application/json")]
pub fn get_creators(db: &State<DbPool>) -> (Status, (ContentType, String)) {
    match problem::get_creators(db.inner()) {
        Ok(vs) => (Status::Ok, (ContentType::JSON, to_string(&vs).unwrap())),
        Err(e) => (e.status, (ContentType::JSON, to_string(&e.detail).unwrap())),
    }
}
