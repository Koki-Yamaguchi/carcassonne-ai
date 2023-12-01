use crate::{
    error::Error,
    game::{
        mov::{DiscardMove, MeepleMove, Move, TileMove},
        tile::Tile,
    },
    translate,
};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

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
    pub favorite_count: i32,
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

#[derive(Serialize, Queryable, Clone, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = schema::favorite)]
pub struct Favorite {
    pub id: i32,
    pub vote_id: i32,
    pub player_id: i32,
    pub player_name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateFavorite {
    pub vote_id: i32,
    pub player_id: i32,
    pub player_name: String,
}

#[allow(dead_code)]
pub fn create_problem(
    db: &DbPool,
    game_id: i32,
    name: String,
    start_at: Option<chrono::NaiveDateTime>,
    creator_id: Option<i32>,
    creator_name: Option<String>,
) -> Result<Problem, Error> {
    database::create_problem(
        db,
        &database::NewProblem {
            game_id,
            name,
            start_at,
            creator_id,
            creator_name,
            vote_count: 0,
        },
    )
}

pub fn get_problem(db: &DbPool, id: i32) -> Result<Problem, Error> {
    database::get_problem(db, id)
}

pub fn get_problems(
    db: &DbPool,
    page: Option<i32>,
    order_by: Option<String>,
    limit: Option<i32>,
    creator: Option<i32>,
) -> Result<ProblemsResponse, Error> {
    let mut p = 0;
    if let Some(pg) = page {
        if p >= 0 {
            p = pg;
        }
    }
    let mut o = "-id".to_string();
    if let Some(ob) = order_by {
        if ob == "id" || ob == "vote_count" || ob == "-id" || ob == "-vote_count" {
            o = ob;
        }
    }
    let mut l = 10;
    if let Some(lm) = limit {
        if lm >= 1 {
            l = lm;
        }
    }

    database::get_problems(db, p, o, l, creator)
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

#[test]
fn create_problem_test() {
    use super::game::decoder;
    use super::game::mov::{DiscardMove, MeepleMove, Move::*, TileMove};
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

    let all_mvs = decoder::decode("src/data/441437812.json".to_string());
    // let all_mvs = create_moves_manually();
    // let all_mvs = create_moves_from_game_against_ai(&db, 6705);

    let remaining_tile_count = 22;
    let problem_name = "Versa-tile 2".to_string();
    let start_at = chrono::DateTime::parse_from_rfc3339("2023-12-01T18:00:00+09:00")
        .unwrap()
        .naive_utc();
    let creator_id = Some(89);
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

    let mvs = all_mvs[0..mv_idx].to_vec();

    let you = -2;
    let opponent = -3;
    let (cur_tile_id, map) = match &all_mvs[mv_idx] {
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

    let first_player_id = map[mvs[2].player_id() as usize];

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

    for mv in &mvs {
        match mv {
            TMove(tm) => {
                database::create_move(
                    &db,
                    TMove(TileMove {
                        id: -1, // ignored
                        ord: tm.ord,
                        game_id: Some(g.id),
                        player_id: map[tm.player_id as usize],
                        tile: tm.tile,
                        rot: tm.rot,
                        pos: tm.pos,
                    }),
                )
                .unwrap();
            }
            MMove(mm) => {
                // FIXME
                let meeple_id = if mm.meeple_id == -1 {
                    -1
                } else {
                    if you /* player0 */ == map[mm.player_id as usize] {
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
                println!();
                database::create_move(
                    &db,
                    MMove(MeepleMove {
                        id: -1, // ignored
                        ord: mm.ord,
                        game_id: Some(g.id),
                        player_id: map[mm.player_id as usize],
                        meeple_id,
                        tile_pos: mm.tile_pos,
                        meeple_pos: mm.meeple_pos,
                    }),
                )
                .unwrap();
            }
            DMove(dm) => {
                database::create_move(
                    &db,
                    DMove(DiscardMove {
                        id: -1, // ignored
                        ord: dm.ord,
                        game_id: Some(g.id),
                        player_id: map[dm.player_id as usize],
                        tile: dm.tile,
                    }),
                )
                .unwrap();
            }
            _ => {
                panic!("move not supported");
            }
        }
    }

    create_problem(
        &db,
        g.id,
        problem_name,
        Some(start_at),
        creator_id,
        creator_name,
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
            favorite_count: 0,
            tile_move_id,
            meeple_move_id,
            player_profile_image_url: player.profile_image_url,
            problem_name: Some(problem.name),
            lang: None,
            translation: "".to_string(),
        },
    )?;

    database::update_problem(db, problem_id, problem.vote_count + 1)?;

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

    database::get_votes(db, problem_id, player_id, fill_moves)
}

pub fn update_vote_translation(db: &DbPool, vote_id: i32) {
    let v = database::get_vote(db, vote_id).unwrap();
    println!("before update vote = {:?}", v);

    let t = translate::Translator::new();
    let lang = t.detect_language(v.note.clone());

    let translation = t.translate(v.note.replace("\n", ""), lang.clone());

    let v = database::update_vote(
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

    println!("after update vote = {:?}", v);
}

pub fn create_favorite(
    db: &DbPool,
    vote_id: i32,
    player_id: i32,
    player_name: String,
) -> Result<Favorite, Error> {
    database::create_favorite(
        db,
        &database::NewFavorite {
            vote_id,
            player_id,
            player_name,
        },
    )
}

pub fn get_favorites(
    db: &DbPool,
    vote_id: Option<i32>,
    player_id: Option<i32>,
) -> Result<Vec<Favorite>, Error> {
    database::get_favorites(db, vote_id, player_id)
}
