use crate::{
    error::Error,
    game::mov::{MeepleMove, TileMove},
    storage,
};
use aws_sdk_s3::Client;
use diesel::Queryable;
use rocket::{
    serde::{Deserialize, Serialize},
    State,
};

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
    pub tile_move: TileMove,
    pub meeple_move_id: i32,
    pub meeple_move: MeepleMove,
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
) -> Result<Problem, Error> {
    database::create_problem(&database::NewProblem {
        game_id,
        name,
        start_at,
    })
}

pub fn get_problem(id: i32) -> Result<Problem, Error> {
    database::get_problem(id)
}

pub fn get_problems() -> Result<Vec<Problem>, Error> {
    database::get_problems()
}

#[test]
fn create_problem_test() {
    use super::game::decoder;
    use super::game::mov::{MeepleMove, Move::*, TileMove};

    let all_mvs = decoder::decode("src/data/424613817.json".to_string());
    let remaining_tile_count = 30;
    let problem_name = "Next Problem".to_string();
    let start_at = chrono::DateTime::parse_from_rfc3339("2023-10-18T05:00:00+09:00")
        .unwrap()
        .naive_utc();

    let mv_idx = (72 - (remaining_tile_count + 2)) * 2;
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
            _ => {
                panic!("discard move is not supported");
            }
        }
    }

    create_problem(g.id, problem_name, Some(start_at)).unwrap();
}

pub fn create_vote(
    problem_id: i32,
    player_id: i32,
    player_name: String,
    note: String,
    tile_move_id: i32,
    meeple_move_id: i32,
) -> Result<Vote, Error> {
    database::create_vote(&database::NewVote {
        problem_id,
        player_id,
        player_name,
        note,
        favorite_count: 0,
        tile_move_id,
        meeple_move_id,
    })
}

pub async fn get_vote(id: i32, storage_client: &State<Client>) -> Result<Vote, Error> {
    let mut vote = database::get_vote(id)?;

    let key = format!("profile-image/{}", vote.player_id);
    vote.player_profile_image_url = storage::get_object_url(storage_client, &key).await;

    Ok(vote)
}

pub async fn get_votes(
    problem_id: Option<i32>,
    storage_client: &State<Client>,
) -> Result<Vec<Vote>, Error> {
    let mut votes = database::get_votes(problem_id)?;
    for v in &mut votes {
        let key = format!("profile-image/{}", v.player_id);
        v.player_profile_image_url = storage::get_object_url(storage_client, &key).await;
    }

    Ok(votes)
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
