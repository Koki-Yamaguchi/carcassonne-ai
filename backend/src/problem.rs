use crate::error::Error;
use diesel::Queryable;
use rocket::serde::Serialize;

use crate::database;

#[derive(Serialize, Queryable, Clone, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = schema::problem)]
pub struct Problem {
    pub id: i32,
    pub game_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Queryable, Clone, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = schema::vote)]
pub struct Vote {
    pub id: i32,
    pub problem_id: i32,
    pub player_id: i32,
    pub player_name: String,
    pub note: String,
    pub favorite_count: i32,
    pub tile_move_id: i32,
    pub meeple_move_id: i32,
    pub created_at: chrono::NaiveDateTime,
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

#[allow(dead_code)]
pub fn create_problem(game_id: i32) -> Result<Problem, Error> {
    database::create_problem(game_id)
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

    let all_mvs = decoder::decode("src/data/426597340.json".to_string());
    let remaining_tile_count = 45;
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
                    game_id: g.id,
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
                    game_id: g.id,
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

    create_problem(g.id).unwrap();
}
