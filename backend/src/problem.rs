use crate::{
    error::Error,
    game::{
        mov::{DiscardMove, MeepleMove, Move, TileMove},
        tile::Tile,
    },
};
use diesel::Queryable;
use rocket::serde::{Deserialize, Serialize};

use crate::database;

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
    game_id: i32,
    name: String,
    start_at: Option<chrono::NaiveDateTime>,
    creator_id: Option<i32>,
    creator_name: Option<String>,
) -> Result<Problem, Error> {
    database::create_problem(&database::NewProblem {
        game_id,
        name,
        start_at,
        creator_id,
        creator_name,
        vote_count: 0,
    })
}

pub fn get_problem(id: i32) -> Result<Problem, Error> {
    database::get_problem(id)
}

pub fn get_problems() -> Result<Vec<Problem>, Error> {
    database::get_problems()
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

fn create_moves() -> Vec<Move> {
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
    use super::game::mov::{MeepleMove, Move::*, TileMove};

    let all_mvs = decoder::decode("src/data/429335153.json".to_string());
    // let all_mvs = create_moves();

    let remaining_tile_count = 36;
    let problem_name = "Versa-tile".to_string();
    let start_at = chrono::DateTime::parse_from_rfc3339("2023-10-31T18:00:00+09:00")
        .unwrap()
        .naive_utc();
    let creator_id = None;
    let creator_name = None;

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
                database::create_move(TMove(TileMove {
                    id: -1, // ignored
                    ord: tm.ord,
                    game_id: Some(g.id),
                    player_id: map[tm.player_id as usize],
                    tile: tm.tile,
                    rot: tm.rot,
                    pos: tm.pos,
                }))
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
                database::create_move(MMove(MeepleMove {
                    id: -1, // ignored
                    ord: mm.ord,
                    game_id: Some(g.id),
                    player_id: map[mm.player_id as usize],
                    meeple_id,
                    tile_pos: mm.tile_pos,
                    meeple_pos: mm.meeple_pos,
                }))
                .unwrap();
            }
            DMove(dm) => {
                database::create_move(DMove(DiscardMove {
                    id: -1, // ignored
                    ord: dm.ord,
                    game_id: Some(g.id),
                    player_id: map[dm.player_id as usize],
                    tile: dm.tile,
                }))
                .unwrap();
            }
            _ => {
                panic!("move not supported");
            }
        }
    }

    create_problem(g.id, problem_name, Some(start_at), creator_id, creator_name).unwrap();
}

pub fn create_vote(
    problem_id: i32,
    player_id: i32,
    player_name: String,
    note: String,
    tile_move_id: i32,
    meeple_move_id: i32,
) -> Result<Vote, Error> {
    let player = match super::player::get_player(player_id) {
        Ok(p) => p,
        Err(e) => {
            return Err(e);
        }
    };

    let vote = database::create_vote(&database::NewVote {
        problem_id,
        player_id,
        player_name,
        note,
        favorite_count: 0,
        tile_move_id,
        meeple_move_id,
        player_profile_image_url: player.profile_image_url,
    })?;

    let problem = database::get_problem(problem_id)?;

    database::update_problem(problem_id, problem.vote_count + 1)?;

    Ok(vote)
}

pub fn get_vote(id: i32) -> Result<Vote, Error> {
    database::get_vote(id)
}

pub fn get_votes(problem_id: Option<i32>, player_id: Option<i32>) -> Result<Vec<Vote>, Error> {
    let mut fill_moves = true;
    if let Some(_) = player_id {
        fill_moves = false;
    }

    database::get_votes(problem_id, player_id, fill_moves)
}

pub fn create_favorite(
    vote_id: i32,
    player_id: i32,
    player_name: String,
) -> Result<Favorite, Error> {
    database::create_favorite(&database::NewFavorite {
        vote_id,
        player_id,
        player_name,
    })
}

pub fn get_favorites(vote_id: Option<i32>, player_id: Option<i32>) -> Result<Vec<Favorite>, Error> {
    database::get_favorites(vote_id, player_id)
}
