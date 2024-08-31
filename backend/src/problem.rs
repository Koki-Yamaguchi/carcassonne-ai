use crate::{
    error::Error,
    game::{
        calculate,
        mov::{DiscardMove, MeepleMove, Move, TileMove},
        tile::Tile,
    },
    translate,
};
use chrono::Duration;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::game::decoder;
use super::game::mov::Move::*;
use crate::database;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize, Queryable, Clone, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = schema::problem)]
pub struct Problem {
    pub id: i32,
    pub game_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub name: String,
    pub start_at: Option<chrono::NaiveDateTime>,
    pub creator_id: Option<i32>,
    pub creator_name: Option<String>,
    pub vote_count: i32,
    pub is_solved: bool,
    pub optimal_move_count: Option<i32>,
    pub tester_id: Option<i32>,
    pub tester_name: Option<String>,
    pub is_draft: bool,
    pub point_diff: Option<i32>,
    pub note: String,
    pub is_deleted: bool,
    pub num: Option<i32>,
    pub favorite_count: i32,
    pub voted: Option<bool>,
    pub favorited: Option<bool>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProblemsResponse {
    pub problems: Vec<Problem>,
    pub total_count: i32,
}

#[derive(Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Vote {
    pub id: i32,
    pub problem_id: i32,
    pub player_id: i32,
    pub player_name: String,
    pub player_profile_image_url: String,
    pub note: String,
    pub tile_move_id: i32,
    pub tile_move: Option<TileMove>,
    pub meeple_move_id: i32,
    pub meeple_move: Option<MeepleMove>,
    pub created_at: chrono::NaiveDateTime,
    pub problem_name: Option<String>,
    pub lang: Option<String>,
    pub translation: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateVote {
    pub problem_id: i32,
    pub player_id: i32,
    pub player_name: String,
    pub note: String,
    pub tile_move_id: i32,
    pub meeple_move_id: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateProblem {
    pub creator_id: i32,
    pub remaining_tile_count: i32,
    pub moves: String,
    pub note: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PublishProblem {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateProblem {
    pub name: String,
    pub start_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Queryable, Clone, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = schema::favorite)]
pub struct Favorite {
    pub id: i32,
    pub player_id: i32,
    pub player_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub problem_id: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateFavorite {
    pub problem_id: i32,
    pub player_id: i32,
    pub player_name: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteFavorite {
    pub problem_id: i32,
    pub player_id: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateProblemProposal {
    pub table_id: String,
    pub remaining_tile_count: i32,
    pub creator_id: i32,
    pub note: String,
}

#[derive(Serialize, Queryable, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ProblemProposal {
    pub id: i32,
    pub table_id: String,
    pub remaining_tile_count: i32,
    pub creator_id: Option<i32>,
    pub used_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub note: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Creator {
    pub id: i32,
    pub name: String,
}

pub fn create_draft_problem(db: &DbPool, params: &CreateProblem) -> Result<Problem, Error> {
    let all_mvs = decoder::decode(params.moves.clone());
    let remaining_tile_count = params.remaining_tile_count;
    let problem_name = "".to_string();
    let creator_id = Some(params.creator_id);
    let mut creator_name = None;
    if let Some(pid) = creator_id {
        let player = database::get_player(&db, pid).unwrap();
        creator_name = Some(player.name);
    }

    let mut tile_count = 0;
    let mut mv_idx = 0;
    for mv in &all_mvs {
        mv_idx += 1;
        match mv {
            MMove(_) => {
                let rem_tile = 72 - tile_count - 2;
                if remaining_tile_count == rem_tile {
                    break;
                }

                continue;
            }
            _ => {
                tile_count += 1;
            }
        }
    }

    let mut mvs = all_mvs[0..mv_idx].to_vec();

    let you = -2;
    let opponent = -3;
    let (cur_tile_id, player_map) = match &all_mvs[mv_idx] {
        TMove(tm) => {
            if tm.player_id == 0 {
                (tm.tile.to_id(), vec![-2, -3])
            } else {
                (tm.tile.to_id(), vec![-3, -2])
            }
        }
        _ => {
            panic!("something wrong");
        }
    };

    let you_name = "You".to_string();
    let opponent_name = "Opponent".to_string();

    let first_player_id = player_map[mvs[2].player_id() as usize];

    let g = database::create_game(
        &db,
        you,
        opponent,
        None,
        Some(opponent),
        Some(cur_tile_id),
        Some(you),
        you_name,
        opponent_name,
        1,
        0,
        false,
        Some(first_player_id),
    )
    .unwrap();

    update_moves_fields(g.id, &mut mvs, you, player_map);

    let s = calculate::calculate(&mvs, true)?;
    let point_diff = s.player0_point - s.player1_point;

    for mv in mvs {
        database::create_move(&db, mv).unwrap();
    }

    database::create_problem(
        db,
        &database::NewProblem {
            game_id: g.id,
            name: problem_name,
            start_at: None,
            creator_id,
            creator_name,
            vote_count: 0,
            is_solved: false,
            optimal_move_count: None,
            tester_id: None,
            tester_name: None,
            is_draft: true,
            point_diff: Some(point_diff),
            note: params.note.clone(),
            is_deleted: false,
            num: None,
            favorite_count: 0,
        },
    )
}

pub fn get_problem(db: &DbPool, id: i32, player: Option<i32>) -> Result<Problem, Error> {
    let mut problem = database::get_problem(db, id)?;

    if let Some(plid) = player {
        let votes = database::get_votes(db, Some(id), Some(plid), false, 0, 1)?;

        let favorites = database::get_favorites(db, Some(id), Some(plid), 0, 1)?;

        problem.voted = Some(votes.len() > 0);
        problem.favorited = Some(favorites.len() > 0);
    }

    Ok(problem)
}

pub fn get_problems(
    db: &DbPool,
    page: Option<i32>,
    order_by: Option<String>,
    limit: Option<i32>,
    creator: Option<i32>,
    is_draft: Option<bool>,
    is_private: Option<bool>,
    player: Option<i32>,
) -> Result<ProblemsResponse, Error> {
    let mut p = 0;
    if let Some(pg) = page {
        if p >= 0 {
            p = pg;
        }
    }
    let mut o = "-start_at".to_string();
    if let Some(ob) = order_by {
        if ob == "start_at"
            || ob == "vote_count"
            || ob == "-start_at"
            || ob == "-vote_count"
            || ob == "favorite_count"
            || ob == "-favorite_count"
        {
            o = ob;
        }
    }
    let mut l = 100;
    if let Some(lm) = limit {
        if lm >= 1 {
            l = lm;
        }
    }

    let mut is_drft = false;
    if let Some(isd) = is_draft {
        is_drft = isd;
    }

    let mut is_prvt = false;
    if let Some(isp) = is_private {
        is_prvt = isp;
    }

    let mut problem_res = database::get_problems(db, p, o, l, creator, is_drft, is_prvt)?;

    if let Some(plid) = player {
        let mut votes = vec![];
        let mut page = 0;
        loop {
            let mut vs = database::get_votes(db, None, Some(plid), false, page, 100)?;
            if vs.len() == 0 {
                break;
            }
            votes.append(&mut vs);
            page += 1;
        }

        let mut favorites = vec![];
        let mut page = 0;
        loop {
            let mut fs = database::get_favorites(db, None, Some(plid), page, 100)?;
            if fs.len() == 0 {
                break;
            }
            favorites.append(&mut fs);
            page += 1;
        }

        for problem in &mut problem_res.problems {
            problem.voted = Some(votes.iter().any(|v| v.problem_id == problem.id));
            problem.favorited = Some(favorites.iter().any(|f| f.problem_id == problem.id));
        }
    }

    Ok(problem_res)
}

#[allow(dead_code)]
fn add_move(
    mvs: &mut Vec<Move>,
    tile: Tile,
    ord0: i32,
    ord1: i32,
    rot: i32,
    pos: (i32, i32),
    meeple_id: i32,
    meeple_pos: i32,
    player_id: i32,
) {
    mvs.push(Move::TMove(TileMove {
        id: -1,
        ord: ord0,
        game_id: None,
        player_id,
        tile,
        rot,
        pos,
    }));
    mvs.push(Move::MMove(MeepleMove {
        id: -1,
        ord: ord1,
        game_id: None,
        player_id,
        meeple_id: meeple_id,
        tile_pos: pos,
        meeple_pos,
    }));
}

#[allow(dead_code)]
fn create_moves_from_game_against_ai(db: &DbPool, game_id: i32) -> Vec<Move> {
    let src_mvs = database::list_moves(db, game_id, None).unwrap();
    let mut mvs = vec![];
    for mv in src_mvs {
        match mv {
            Move::TMove(tm) => mvs.push(Move::TMove(TileMove {
                id: tm.id,
                ord: tm.ord,
                game_id: tm.game_id,
                player_id: if tm.player_id == 1 { 1 } else { 0 },
                tile: tm.tile,
                rot: tm.rot,
                pos: tm.pos,
            })),
            Move::MMove(mm) => mvs.push(Move::MMove(MeepleMove {
                id: mm.id,
                ord: mm.ord,
                game_id: mm.game_id,
                player_id: if mm.player_id == 1 { 1 } else { 0 },
                meeple_id: mm.meeple_id,
                tile_pos: mm.tile_pos,
                meeple_pos: mm.meeple_pos,
            })),
            Move::DMove(dm) => mvs.push(Move::DMove(DiscardMove {
                id: dm.id,
                ord: dm.ord,
                game_id: dm.game_id,
                player_id: if dm.player_id == 1 { 1 } else { 0 },
                tile: dm.tile,
            })),
            _ => {}
        }
    }
    mvs
}

#[allow(dead_code)]
fn create_moves_manually() -> Vec<Move> {
    let mut mvs = vec![];

    add_move(&mut mvs, Tile::StartingTile, 0, 1, 0, (0, 0), -1, -1, 0);
    add_move(
        &mut mvs,
        Tile::QuadrupleCityWithCOA,
        2,
        3,
        0,
        (-1, 0),
        0,
        0,
        1,
    );
    add_move(&mut mvs, Tile::QuadrupleRoad, 4, 5, 0, (0, -1), 7, 4, 0);
    add_move(
        &mut mvs,
        Tile::CityCapWithCrossroad,
        6,
        7,
        1,
        (-1, -1),
        1,
        3,
        1,
    );
    add_move(&mut mvs, Tile::Straight, 8, 9, 1, (0, 1), -1, -1, 0);
    add_move(&mut mvs, Tile::Triangle, 10, 11, 1, (-2, 0), -1, -1, 1);

    mvs
}

fn update_moves_fields(game_id: i32, mvs: &mut Vec<Move>, you: i32, player_map: Vec<i32>) {
    for mv in mvs {
        match mv {
            TMove(tm) => {
                *mv = TMove(TileMove {
                    id: -1, // ignored
                    ord: tm.ord,
                    game_id: Some(game_id),
                    player_id: player_map[tm.player_id as usize],
                    tile: tm.tile,
                    rot: tm.rot,
                    pos: tm.pos,
                });
            }
            MMove(mm) => {
                let meeple_id = if mm.meeple_id == -1 {
                    -1
                } else {
                    if you /* player0 */ == player_map[mm.player_id as usize] {
                        if mm.meeple_id >= 7 {
                            mm.meeple_id - 7
                        } else {
                            mm.meeple_id
                        }
                    } else {
                        if mm.meeple_id < 7 {
                            mm.meeple_id + 7
                        } else {
                            mm.meeple_id
                        }
                    }
                };
                *mv = MMove(MeepleMove {
                    id: -1, // ignored
                    ord: mm.ord,
                    game_id: Some(game_id),
                    player_id: player_map[mm.player_id as usize],
                    meeple_id,
                    tile_pos: mm.tile_pos,
                    meeple_pos: mm.meeple_pos,
                });
            }
            DMove(dm) => {
                *mv = DMove(DiscardMove {
                    id: -1, // ignored
                    ord: dm.ord,
                    game_id: Some(game_id),
                    player_id: player_map[dm.player_id as usize],
                    tile: dm.tile,
                });
            }
            _ => {
                panic!("move not supported");
            }
        }
    }
}

#[test]
fn create_problem_test() {
    // use super::game::decoder;
    use super::game::mov::Move::*;
    use dotenvy::dotenv;
    use std::env;
    use std::time::Duration;

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db = Pool::builder()
        .max_size(1) // FIXME: Didn't think about this number carefully
        .connection_timeout(Duration::from_secs(300))
        .build(manager)
        .expect("Creating a pool failed");

    // should-be-modified lines start
    let all_mvs = decoder::decode_from_file_path("src/data/484158010.json".to_string());
    // let all_mvs = create_moves_manually();
    // let all_mvs = create_moves_from_game_against_ai(&db, 7944);

    let remaining_tile_count = 67;
    let problem_name = "test".to_string();
    let start_at = Some(
        chrono::DateTime::parse_from_rfc3339("2023-12-09T18:00:00+09:00")
            .unwrap()
            .naive_utc(),
    );
    // let start_at = None;
    let creator_id = None;
    let mut creator_name = None;
    if let Some(pid) = creator_id {
        let player = database::get_player(&db, pid).unwrap();
        creator_name = Some(player.name);
    }
    let is_draft = false;

    // for solved problems
    let is_solved = false;
    let optimal_move_count = None;
    let tester_id = None;
    let mut tester_name = None;
    if let Some(pid) = tester_id {
        let player = database::get_player(&db, pid).unwrap();
        tester_name = Some(player.name);
    }
    // should-be-modified lines end

    let mut tile_count = 0;
    let mut mv_idx = 0;
    for mv in &all_mvs {
        mv_idx += 1;
        match mv {
            MMove(_) => {
                let rem_tile = 72 - tile_count - 2;
                if remaining_tile_count == rem_tile {
                    break;
                }

                continue;
            }
            _ => {
                tile_count += 1;
            }
        }
    }

    let mut mvs = all_mvs[0..mv_idx].to_vec();

    let you = -2;
    let opponent = -3;
    let (cur_tile_id, player_map) = match &all_mvs[mv_idx] {
        TMove(tm) => {
            if tm.player_id == 0 {
                (tm.tile.to_id(), vec![-2, -3])
            } else {
                (tm.tile.to_id(), vec![-3, -2])
            }
        }
        _ => {
            assert!(false);
            return;
        }
    };

    let you_name = "You".to_string();
    let opponent_name = "Opponent".to_string();

    let first_player_id = player_map[mvs[2].player_id() as usize];

    let g = database::create_game(
        &db,
        you,
        opponent,
        None,
        Some(opponent),
        Some(cur_tile_id),
        Some(you),
        you_name,
        opponent_name,
        1,
        0,
        false,
        Some(first_player_id),
    )
    .unwrap();

    update_moves_fields(g.id, &mut mvs, you, player_map);

    let s = calculate::calculate(&mvs, true).unwrap();
    let point_diff = s.player0_point - s.player1_point;

    for mv in mvs {
        database::create_move(&db, mv).unwrap();
    }

    database::create_problem(
        &db,
        &database::NewProblem {
            game_id: g.id,
            name: problem_name,
            start_at,
            creator_id,
            creator_name,
            vote_count: 0,
            is_solved,
            optimal_move_count,
            tester_id,
            tester_name,
            is_draft,
            point_diff: Some(point_diff),
            note: "".to_string(),
            is_deleted: false,
            num: None,
            favorite_count: 0,
        },
    )
    .unwrap();
}

pub fn create_vote(
    db: &DbPool,
    problem_id: i32,
    player_id: i32,
    player_name: String,
    note: String,
    tile_move_id: i32,
    meeple_move_id: i32,
) -> Result<Vote, Error> {
    let player = match super::player::get_player(db, player_id) {
        Ok(p) => p,
        Err(e) => {
            return Err(e);
        }
    };

    let problem = database::get_problem(db, problem_id)?;

    let vote = database::create_vote(
        db,
        &database::NewVote {
            problem_id,
            player_id,
            player_name,
            note,
            tile_move_id,
            meeple_move_id,
            player_profile_image_url: player.profile_image_url,
            problem_name: Some(problem.name.clone()),
            lang: None,
            translation: "".to_string(),
        },
    )?;

    database::update_problem(
        db,
        problem_id,
        problem.name,
        problem.start_at,
        problem.is_draft,
        problem.vote_count + 1,
        problem.point_diff,
        problem.is_deleted,
        problem.num,
    )?;

    Ok(vote)
}

pub fn get_vote(db: &DbPool, id: i32) -> Result<Vote, Error> {
    database::get_vote(db, id)
}

pub fn get_votes(
    db: &DbPool,
    problem_id: Option<i32>,
    player_id: Option<i32>,
) -> Result<Vec<Vote>, Error> {
    let mut fill_moves = false;
    if let Some(_) = problem_id {
        fill_moves = true;
    }

    database::get_votes(db, problem_id, player_id, fill_moves, 0, 300) // TODO: pagination (in frontend)
}

pub fn update_vote_translation(db: &DbPool, vote_id: i32) {
    let v = database::get_vote(db, vote_id).unwrap();
    let t = translate::Translator::new();
    let lang = t.detect_language(v.note.clone());
    let translation = t.translate(v.note.replace("\n", ""), lang.clone());

    let _ = database::update_vote(
        db,
        v.id,
        v.player_profile_image_url,
        if lang == translate::Lang::Japanese {
            Some("ja".to_string())
        } else {
            Some("en".to_string())
        },
        translation,
    )
    .unwrap();
}

pub fn create_favorite(
    db: &DbPool,
    problem_id: i32,
    player_id: i32,
    player_name: String,
) -> Result<Favorite, Error> {
    database::create_favorite(
        db,
        &database::NewFavorite {
            player_id,
            player_name,
            problem_id,
        },
    )
}

pub fn delete_favorite(db: &DbPool, problem_id: i32, player_id: i32) -> Result<(), Error> {
    database::delete_favorite(db, problem_id, player_id)
}

pub fn get_favorites(
    db: &DbPool,
    vote_id: Option<i32>,
    player_id: Option<i32>,
) -> Result<Vec<Favorite>, Error> {
    database::get_favorites(db, vote_id, player_id, 0, 100)
}

/*
#[test]
fn list_daily_number_of_vote() {
    use dotenvy::dotenv;
    use std::env;
    use std::time::Duration;

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db = Pool::builder()
        .max_size(1) // FIXME: Didn't think about this number carefully
        .connection_timeout(Duration::from_secs(300))
        .build(manager)
        .expect("Creating a pool failed");

    let mut votes = vec![];
    for problem_id in 1..48 {
        let mut vs = database::get_votes(&db, Some(problem_id), None, false).unwrap();
        votes.append(&mut vs);
    }

    votes.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    let mut cur = 0;
    let mut cur_t = chrono::DateTime::parse_from_rfc3339("2023-10-16T00:00:00+00:00")
        .unwrap()
        .naive_utc();
    let end_t = chrono::DateTime::parse_from_rfc3339("2023-12-04T00:00:00+00:00")
        .unwrap()
        .naive_utc();
    loop {
        loop {
            if cur >= votes.len() || votes[cur].created_at > cur_t {
                break;
            }
            cur += 1;
        }
        println!("{:?},{:?}", cur_t, cur);
        cur_t += chrono::Duration::days(1);
        if cur_t > end_t {
            break;
        }
    }
    assert!(false);
}
*/

pub fn create_problem_proposal(
    db: &DbPool,
    params: &CreateProblemProposal,
) -> Result<ProblemProposal, Error> {
    database::create_problem_proposal(
        db,
        &database::NewProblemProposal {
            table_id: params.table_id.clone(),
            remaining_tile_count: params.remaining_tile_count,
            creator_id: Some(params.creator_id),
            note: params.note.clone(),
        },
    )
}

pub fn publish_problem(db: &DbPool, id: i32, params: &PublishProblem) -> Result<Problem, Error> {
    let prb = database::get_problem(db, id)?;

    let private_prbs =
        database::get_problems(db, 0, "-start_at".to_string(), 1, None, false, true)?.problems;

    let (last_start_at, last_num) = if private_prbs.len() == 1 {
        (
            private_prbs[0].start_at.unwrap(),
            private_prbs[0].num.unwrap(),
        )
    } else {
        let prbs =
            database::get_problems(db, 0, "-start_at".to_string(), 1, None, false, false)?.problems;
        assert!(prbs.len() >= 1);

        (prbs[0].start_at.unwrap(), prbs[0].num.unwrap())
    };

    database::update_problem(
        db,
        id,
        params.name.clone(),
        Some(last_start_at + Duration::days(1)),
        false,
        prb.vote_count,
        prb.point_diff,
        prb.is_deleted,
        Some(last_num + 1),
    )
}

pub fn delete_problem(db: &DbPool, id: i32) -> Result<Problem, Error> {
    let prb = database::get_problem(db, id)?;

    database::update_problem(
        db,
        id,
        prb.name,
        prb.start_at,
        prb.is_draft,
        prb.vote_count,
        prb.point_diff,
        true,
        prb.num,
    )
}

pub fn update_problem(db: &DbPool, id: i32, params: &UpdateProblem) -> Result<Problem, Error> {
    let prb = database::get_problem(db, id)?;

    database::update_problem(
        db,
        id,
        params.name.clone(),
        Some(params.start_at),
        prb.is_draft,
        prb.vote_count,
        prb.point_diff,
        prb.is_deleted,
        prb.num,
    )
}

pub fn get_problem_proposals(
    db: &DbPool,
    player: Option<i32>,
) -> Result<Vec<ProblemProposal>, Error> {
    database::get_problem_proposals(db, player)
}

pub fn use_problem_proposal(db: &DbPool, id: i32) -> Result<ProblemProposal, Error> {
    database::use_problem_proposal(db, id)
}

#[test]
fn update_all_point_diff() {
    use dotenvy::dotenv;
    use std::env;
    use std::time::Duration;

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db = Pool::builder()
        .max_size(1) // FIXME: Didn't think about this number carefully
        .connection_timeout(Duration::from_secs(300))
        .build(manager)
        .expect("Creating a pool failed");

    for problem_id in 1..100 {
        println!("problem id = {:?}", problem_id);
        match database::get_problem(&db, problem_id) {
            Ok(problem) => {
                let mvs = database::list_moves(&db, problem.game_id, None).unwrap();

                let s = calculate::calculate(&mvs, true).unwrap();
                let point_diff = s.player0_point - s.player1_point;

                println!("before problem = {:?}", problem);
                let problem = database::update_problem(
                    &db,
                    problem.id,
                    problem.name,
                    problem.start_at,
                    problem.is_draft,
                    problem.vote_count,
                    Some(point_diff),
                    problem.is_deleted,
                    problem.num,
                )
                .unwrap();
                println!("after problem = {:?}", problem);
            }
            Err(_) => {
                println!("not found");
            }
        }
    }
}

#[test]
fn update_all_num() {
    use dotenvy::dotenv;
    use std::env;
    use std::time::Duration;

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db = Pool::builder()
        .max_size(1) // FIXME: Didn't think about this number carefully
        .connection_timeout(Duration::from_secs(300))
        .build(manager)
        .expect("Creating a pool failed");

    let prbs = database::get_problems(&db, 0, "start_at".to_string(), 100, None, false, false)
        .unwrap()
        .problems;
    // assert!(prbs.len() == 100);

    let mut cur = 1;
    for prb in &prbs {
        println!("prb = {:?}", prb);
        let new_prb = database::update_problem(
            &db,
            prb.id,
            prb.name.clone(),
            prb.start_at,
            prb.is_draft,
            prb.vote_count,
            prb.point_diff,
            prb.is_deleted,
            Some(cur),
        )
        .unwrap();

        println!("new_prb = {:?}", new_prb);
        println!();

        cur += 1;
    }
}

pub fn get_creators(db: &DbPool) -> Result<Vec<Creator>, Error> {
    let mut page = 0;
    let mut all_problems = vec![];
    loop {
        let mut problem_res =
            database::get_problems(db, page, "-started_at".to_string(), 300, None, false, false)?;
        if problem_res.problems.len() == 0 {
            break;
        }

        all_problems.append(&mut problem_res.problems);
        page += 1;
    }

    let mut count = HashMap::new();
    for problem in &all_problems {
        if problem.creator_id.is_some() {
            *count.entry(problem.creator_id.unwrap()).or_insert(0) += 1;
        }
    }

    let mut creators: Vec<Creator> = all_problems
        .into_iter()
        .filter(|p| p.creator_id.is_some() && p.creator_name.is_some())
        .map(|p| (p.creator_id, p.creator_name))
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|(id, name)| Creator {
            id: id.unwrap(),
            name: name.unwrap(),
        })
        .collect::<Vec<Creator>>();

    creators.sort_by(|a, b| count.get(&b.id).cmp(&count.get(&a.id)));

    Ok(creators)
}
