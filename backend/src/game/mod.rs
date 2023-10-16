pub mod board;
pub mod calculate;
pub mod calculate_next_move;
pub mod debug_moves;
pub mod decoder;
pub mod evaluate;
pub mod mergeable_feature;
pub mod mov;
pub mod rating;
pub mod solver;
pub mod tile;

use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

use crate::database::{self, get_player, update_player};
use crate::error::{bad_request_error, Error};
use crate::game::rating::calculate_rating;
use crate::game::solver::SolveResult;
use crate::game::tile::to_tile;

use self::board::{Board, BoardTile};
use self::calculate::calculate;
use self::solver::solve;
use self::tile::{remaining_tiles, tiles, Tile};
use mov::Move::*;
use mov::{DiscardMove, MeepleMove, TileMove};
use rand::Rng;
use tile::Tile::*;

#[derive(Serialize, Queryable, Clone, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = schema::waiting_game)]
pub struct WaitingGame {
    pub id: i32,
    pub player_id: i32,
    pub game_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UpdateWaitingGame {
    pub game_id: i32,
}

#[derive(Serialize, Queryable, Clone, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = schema::game)]
pub struct Game {
    pub id: i32,
    pub player0_id: i32,
    pub player1_id: i32,
    pub player0_point: i32,
    pub player1_point: i32,
    pub next_tile_id: Option<i32>,
    pub next_player_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub ended_at: Option<chrono::NaiveDateTime>,
    pub current_player_id: Option<i32>,
    pub current_tile_id: Option<i32>,
    pub player0_name: String,
    pub player1_name: String,
    pub player0_color: i32,
    pub player1_color: i32,
    pub is_rated: bool,
    pub before_player0_rating: Option<i32>,
    pub before_player1_rating: Option<i32>,
    pub after_player0_rating: Option<i32>,
    pub after_player1_rating: Option<i32>,
    pub first_player_id: Option<i32>,
    pub winner_player_id: Option<i32>,
}

#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct CompleteEvent {
    pub meeple_ids: Vec<i32>,
    pub feature: String,
    pub point: i32,
}
#[derive(Serialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct MeepleMoveResult {
    pub complete_events: Vec<CompleteEvent>,
    pub next_tile_id: i32,
    pub next_player_id: i32,
    pub current_tile_id: i32,
    pub current_player_id: i32,
}

pub fn get_waiting_games() -> Result<Vec<WaitingGame>, Error> {
    database::get_waiting_games()
}

pub fn create_waiting_game(player_id: i32) -> Result<WaitingGame, Error> {
    database::create_waiting_game(player_id)
}

pub fn update_waiting_game(id: i32, game_id: i32) -> Result<WaitingGame, Error> {
    database::update_waiting_game(id, game_id)
}

pub fn delete_waiting_games(player_id: i32) -> Result<(), Error> {
    database::delete_waiting_game(player_id)
}

pub fn create_game(
    player0_id: i32,
    player1_id: i32,
    player0_color: i32,
    player1_color: i32,
    is_rated: bool,
) -> Result<Game, Error> {
    let mut rng = rand::thread_rng();
    let first_player_id = if rng.gen_range(0..2) < 1 {
        player0_id
    } else {
        player1_id
    };
    let second_player_id = if first_player_id == player0_id {
        player1_id
    } else {
        player0_id
    };

    let tiles = tiles();
    let cur_tile = tiles[rng.gen_range(0..tiles.len())];
    let rem_tiles = remaining_tiles(vec![cur_tile]);
    let next_tile = rem_tiles[rng.gen_range(0..rem_tiles.len())];

    let player0_name = match database::get_player(player0_id) {
        Ok(p) => p.name,
        Err(e) => {
            return Err(e);
        }
    };
    let player1_name = match database::get_player(player1_id) {
        Ok(p) => p.name,
        Err(e) => {
            return Err(e);
        }
    };

    let g = match database::create_game(
        player0_id,
        player1_id,
        Some(next_tile.to_id()),
        Some(second_player_id),
        Some(cur_tile.to_id()),
        Some(first_player_id),
        player0_name,
        player1_name,
        player0_color,
        player1_color,
        is_rated,
        Some(first_player_id),
    ) {
        Ok(g) => g,
        Err(e) => {
            return Err(e);
        }
    };

    let mv0 = TMove(TileMove {
        id: -1, // ignored
        ord: 0,
        game_id: Some(g.id),
        player_id: second_player_id,
        tile: StartingTile,
        rot: 0,
        pos: (0, 0),
    });
    let mv1 = MMove(MeepleMove {
        id: -1, // ignored
        ord: 1,
        game_id: Some(g.id),
        player_id: second_player_id,
        meeple_id: -1,
        tile_pos: (0, 0),
        meeple_pos: -1,
    });

    match database::create_move(mv0.clone()) {
        Err(e) => return Err(e),
        _ => {}
    }
    match database::create_move(mv1.clone()) {
        Err(e) => return Err(e),
        _ => {}
    }

    Ok(g)
}

pub fn create_tile_move(
    game_id: Option<i32>,
    player_id: i32,
    tile: tile::Tile,
    rot: i32,
    pos: (i32, i32),
) -> Result<mov::Move, Error> {
    // dangling move for voting
    if let None = game_id {
        return database::create_move(TMove(TileMove {
            id: -1, // ignored
            ord: -1,
            game_id,
            player_id,
            tile,
            rot,
            pos,
        }));
    }

    let mut moves = match database::list_moves(game_id.unwrap(), None) {
        Ok(mvs) => mvs,
        Err(e) => {
            return Err(e);
        }
    };
    assert!(moves.len() != 0);
    let last_move = moves.last().unwrap();

    match last_move {
        TMove(_) => {
            return Err(bad_request_error(
                "move before a tile move must not be a tile move".to_string(),
            ))
        }
        MMove(mm) => {
            if mm.player_id == player_id {
                return Err(bad_request_error(
                    "player of the previous meeple move must not be the same player who is going to play".to_string(),
                ));
            }
        }
        _ => {}
    }

    let ord = last_move.ord() + 1;

    let mv = TMove(TileMove {
        id: -1, // ignored
        ord,
        game_id,
        player_id,
        tile,
        rot,
        pos,
    });
    moves.push(mv.clone());

    calculate::calculate(&moves, false)?;

    database::create_move(mv)
}

pub fn create_meeple_move(
    game_id: Option<i32>,
    player_id: i32,
    meeple_id: i32,
    tile_pos: (i32, i32),
    meeple_pos: i32,
) -> Result<mov::Move, Error> {
    // dangling move for voting
    if let None = game_id {
        return database::create_move(MMove(MeepleMove {
            id: -1, // ignored
            ord: -1,
            game_id,
            player_id,
            meeple_id,
            tile_pos,
            meeple_pos,
        }));
    }

    let gm = match database::get_game(game_id.unwrap()) {
        Ok(game) => game,
        Err(e) => {
            return Err(e);
        }
    };

    let mut rng = rand::thread_rng();
    let mut moves = match database::list_moves(game_id.unwrap(), None) {
        Ok(mvs) => mvs,
        Err(e) => {
            return Err(e);
        }
    };
    assert!(moves.len() != 0);
    let last_move = moves.last().unwrap();

    match last_move {
        TMove(_) => {}
        _ => {
            return Err(bad_request_error(
                "move before a meeple move must be a tile move".to_string(),
            ))
        }
    }

    let ord = last_move.ord() + 1;

    let mv = MMove(MeepleMove {
        id: -1, // ignored
        ord,
        game_id,
        player_id,
        meeple_id,
        tile_pos,
        meeple_pos,
    });
    moves.push(mv.clone());

    let mut complete_events = vec![];

    let res = calculate::calculate(&moves, false);
    let (player0_point, player1_point) = match res {
        Ok(s) => {
            for e in &s.complete_events {
                complete_events.push(CompleteEvent {
                    meeple_ids: e.meeple_ids.clone(),
                    feature: e.feature.clone().to_string(),
                    point: e.point,
                })
            }
            (s.player0_point, s.player1_point)
        }
        Err(e) => {
            return Err(e);
        }
    };

    let mut out_tiles = vec![];
    match gm.next_tile_id {
        Some(tid) => {
            if tid != -1 {
                out_tiles.push(to_tile(tid))
            }
        }
        None => {}
    }
    for mv in moves {
        match mv {
            mov::Move::TMove(tm) => {
                out_tiles.push(tm.tile.clone());
            }
            mov::Move::DMove(dm) => {
                out_tiles.push(dm.tile.clone());
            }
            _ => {}
        }
    }
    let remaining_tiles = tile::remaining_tiles(out_tiles.clone());

    let next_tile = if remaining_tiles.len() == 0 {
        Invalid
    } else {
        remaining_tiles[rng.gen_range(0..remaining_tiles.len())]
    };

    let cur_player_id = if player_id == gm.player0_id {
        gm.player1_id
    } else {
        gm.player0_id
    };
    let next_player_id = if player_id == gm.player0_id {
        gm.player0_id
    } else {
        gm.player1_id
    };

    match database::update_game(
        game_id.unwrap(),
        next_tile.to_id(),
        next_player_id,
        player0_point,
        player1_point,
        gm.next_tile_id.unwrap(),
        cur_player_id,
        gm.before_player0_rating,
        gm.before_player1_rating,
        gm.after_player0_rating,
        gm.after_player1_rating,
        gm.first_player_id,
        gm.winner_player_id,
    ) {
        Err(e) => {
            return Err(e);
        }
        Ok(_) => {}
    }

    database::create_move(mv)
}

pub fn create_discard_move(
    game_id: Option<i32>,
    player_id: i32,
    tile: tile::Tile,
) -> Result<mov::Move, Error> {
    // dangling move for voting (although discard move is not really used)
    if let None = game_id {
        return database::create_move(DMove(DiscardMove {
            id: -1, // ignored
            ord: -1,
            game_id,
            player_id,
            tile,
        }));
    }

    let mut rng = rand::thread_rng();

    let gm = match database::get_game(game_id.unwrap()) {
        Ok(game) => game,
        Err(e) => {
            return Err(e);
        }
    };

    let mut moves = match database::list_moves(game_id.unwrap(), None) {
        Ok(mvs) => mvs,
        Err(e) => {
            return Err(e);
        }
    };
    assert!(moves.len() != 0);

    let ord = moves.last().unwrap().ord() + 1;

    let mv = DMove(DiscardMove {
        id: -1, // ignored
        ord,
        game_id,
        player_id,
        tile,
    });
    moves.push(mv.clone());

    let mut out_tiles = vec![];
    match gm.next_tile_id {
        Some(tid) => {
            if tid != -1 {
                out_tiles.push(to_tile(tid))
            }
        }
        None => {}
    }
    for mv in moves {
        match mv {
            mov::Move::TMove(tm) => {
                out_tiles.push(tm.tile.clone());
            }
            mov::Move::DMove(dm) => {
                out_tiles.push(dm.tile.clone());
            }
            _ => {}
        }
    }
    let remaining_tiles = tile::remaining_tiles(out_tiles.clone());

    let draw_tile = if remaining_tiles.len() == 0 {
        Invalid
    } else {
        remaining_tiles[rng.gen_range(0..remaining_tiles.len())]
    };

    // TODO: what if thre's no tile to draw on the second last turn?

    match database::update_game(
        game_id.unwrap(),
        gm.next_tile_id.unwrap(),
        gm.next_player_id.unwrap(),
        gm.player0_point,
        gm.player1_point,
        draw_tile.to_id(),
        gm.current_player_id.unwrap(),
        gm.before_player0_rating,
        gm.before_player1_rating,
        gm.after_player0_rating,
        gm.after_player1_rating,
        gm.first_player_id,
        gm.winner_player_id,
    ) {
        Err(e) => {
            return Err(e);
        }
        Ok(_) => {}
    }

    database::create_move(mv)
}

pub fn wait_ai_move(game_id: i32) -> Result<(), Error> {
    let game = match database::get_game(game_id) {
        Ok(gm) => gm,
        Err(e) => {
            return Err(e);
        }
    };

    let moves = match database::list_moves(game.id, None) {
        Ok(mvs) => mvs,
        Err(e) => {
            return Err(e);
        }
    };
    assert!(moves.len() != 0);

    let placing_tile = tile::to_tile(game.current_tile_id.unwrap());

    match calculate_next_move::calculate_next_move(
        &moves,
        Some(game.id),
        game.player0_id,
        game.player1_id,
        1,
        placing_tile,
    ) {
        Some((tile_move, meeple_move)) => {
            create_tile_move(Some(game.id), 1, placing_tile, tile_move.rot, tile_move.pos)?;

            create_meeple_move(
                Some(game.id),
                1,
                meeple_move.meeple_id,
                meeple_move.tile_pos,
                meeple_move.meeple_pos,
            )?;

            Ok(())
        }
        None => {
            create_discard_move(Some(game.id), 1, placing_tile)?;

            Ok(())
        }
    }
}

pub fn get_game(game_id: i32) -> Result<Game, Error> {
    database::get_game(game_id)
}

pub fn get_games(
    player_id: Option<i32>,
    is_rated: Option<bool>,
    limit: Option<i32>,
) -> Result<Vec<Game>, Error> {
    database::get_games(player_id, is_rated, limit)
}

pub fn get_moves(game_id: Option<i32>, move_id: Option<i32>) -> Result<Vec<mov::Move>, Error> {
    match game_id {
        Some(gid) => database::list_moves(gid, move_id),
        None => Err(bad_request_error(
            "parameter `game_id` is required".to_string(),
        )),
    }
}

pub fn get_final_events(game_id: Option<i32>) -> Result<MeepleMoveResult, Error> {
    let gid = match game_id {
        Some(gid) => gid,
        None => {
            return Err(bad_request_error(
                "parameter `game_id` is required".to_string(),
            ))
        }
    };

    let gm = match database::get_game(gid) {
        Ok(game) => game,
        Err(e) => {
            return Err(e);
        }
    };

    let moves = match database::list_moves(gid, None) {
        Ok(mvs) => mvs,
        Err(e) => {
            return Err(e);
        }
    };

    let first_player_id = match gm.first_player_id {
        Some(fpi) => fpi,
        None => {
            assert!(moves.len() >= 3);
            match &moves[2] {
                TMove(m) => m.player_id,
                _ => -1,
            }
        }
    };

    let mut complete_events = vec![];

    let res = calculate::calculate(&moves, true);
    let (player0_point, player1_point) = match res {
        Ok(s) => {
            for e in &s.complete_events {
                complete_events.push(CompleteEvent {
                    meeple_ids: e.meeple_ids.clone(),
                    feature: e.feature.clone().to_string(),
                    point: e.point,
                })
            }
            (s.player0_point, s.player1_point)
        }
        Err(e) => {
            return Err(e);
        }
    };

    if gm.winner_player_id == None {
        let player0 = match get_player(gm.player0_id) {
            Ok(p) => p,
            Err(e) => {
                return Err(e);
            }
        };
        let player1 = match get_player(gm.player1_id) {
            Ok(p) => p,
            Err(e) => {
                return Err(e);
            }
        };

        let mut before_player0_rating = player0.rating;
        let mut before_player1_rating = player1.rating;
        let player0_win = player0_point > player1_point
            || (player0_point == player1_point && first_player_id == gm.player1_id);
        let winner_player_id = if player0_win {
            gm.player0_id
        } else {
            gm.player1_id
        };
        let mut after_player0_rating = player0.rating;
        let mut after_player1_rating = player1.rating;
        if gm.is_rated {
            before_player0_rating = match before_player0_rating {
                Some(r) => Some(r),
                None => Some(1500),
            };
            before_player1_rating = match before_player1_rating {
                Some(r) => Some(r),
                None => Some(1500),
            };
            (after_player0_rating, after_player1_rating) = match calculate_rating(
                before_player0_rating.unwrap(),
                before_player1_rating.unwrap(),
                player0_win,
            ) {
                (r0, r1) => (Some(r0), Some(r1)),
            };

            // Currently & experimentally, AI's rating is always 1500
            if player0.id == 1 {
                after_player0_rating = Some(1500);
            }
            if player1.id == 1 {
                after_player1_rating = Some(1500);
            }

            match update_player(
                player0.id,
                player0.name,
                player0.meeple_color,
                after_player0_rating,
            ) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
            match update_player(
                player1.id,
                player1.name,
                player1.meeple_color,
                after_player1_rating,
            ) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        match database::update_game(
            gm.id,
            gm.next_tile_id.unwrap(),
            gm.next_player_id.unwrap(),
            player0_point,
            player1_point,
            gm.current_tile_id.unwrap(),
            gm.current_player_id.unwrap(),
            before_player0_rating,
            before_player1_rating,
            after_player0_rating,
            after_player1_rating,
            Some(first_player_id),
            Some(winner_player_id),
        ) {
            Err(e) => {
                return Err(e);
            }
            Ok(_) => {}
        }

        /*
        thread::spawn(move || {
            create_optimal_move(gm.id, 3, gm.player0_id, gm.player1_id);
        });
        */
    }

    Ok(MeepleMoveResult {
        complete_events,
        next_tile_id: gm.next_tile_id.unwrap(),
        next_player_id: gm.next_player_id.unwrap(),
        current_tile_id: gm.current_tile_id.unwrap(),
        current_player_id: gm.current_player_id.unwrap(),
    })
}

#[allow(dead_code, unused_assignments)]
fn create_optimal_move(game_id: i32, last_n: i32, player0_id: i32, player1_id: i32) {
    let mut moves = match database::list_moves(game_id, None) {
        Ok(mvs) => mvs,
        Err(_) => {
            return;
        }
    };

    let mut tile_count = 0;

    let mut next_tile = Tile::Invalid;
    loop {
        match moves.last().unwrap() {
            TMove(tm) => {
                tile_count += 1;
                if tile_count == last_n {
                    next_tile = tm.tile;
                    moves.pop();
                    break;
                }
            }
            _ => {}
        }
        moves.pop();
    }

    let ((mut tile_move, mut meeple_move), solve_result) = solve(
        &moves,
        Some(game_id),
        player0_id,
        player1_id,
        next_tile,
        false,
    );

    if solve_result != SolveResult::AlwaysWin {
        return;
    }

    tile_move.ord = -1;
    meeple_move.ord = -1;

    let tile_move_id = match database::create_move(TMove(tile_move)) {
        Ok(m) => match m {
            TMove(tm) => tm.id,
            _ => {
                return;
            }
        },
        Err(e) => {
            panic!("{:?}", e.detail.msg);
        }
    };
    let meeple_move_id = match database::create_move(MMove(meeple_move)) {
        Ok(m) => match m {
            MMove(mm) => mm.id,
            _ => {
                return;
            }
        },
        Err(e) => {
            panic!("{:?}", e.detail.msg);
        }
    };

    match database::create_optimal_move(game_id, last_n, tile_move_id, meeple_move_id) {
        Ok(_) => {}
        Err(e) => {
            panic!("{:?}", e.detail.msg);
        }
    }
}

pub fn get_board(game_id: Option<i32>, move_id: Option<i32>) -> Result<Board, Error> {
    let gid = match game_id {
        Some(gid) => gid,
        None => {
            return Err(bad_request_error(
                "parameter `game_id` is required".to_string(),
            ))
        }
    };

    let moves = match database::list_moves(gid, move_id) {
        Ok(mvs) => mvs,
        Err(e) => {
            return Err(e);
        }
    };

    let (player0_point, player1_point, b, meepleable_positions, complete_events) =
        match calculate(&moves, false) {
            Ok(s) => {
                let mut complete_events = vec![];
                for e in &s.complete_events {
                    complete_events.push(CompleteEvent {
                        meeple_ids: e.meeple_ids.clone(),
                        feature: e.feature.clone().to_string(),
                        point: e.point,
                    })
                }
                (
                    s.player0_point,
                    s.player1_point,
                    s.board,
                    s.meepleable_positions,
                    complete_events,
                )
            }
            Err(e) => {
                return Err(e);
            }
        };

    let board_size = 20 * 2 + 1;
    let mut tiles = vec![
        vec![
            BoardTile {
                id: -1,
                rot: -1,
                meeple_id: -1,
                meeple_pos: -1,
            };
            board_size
        ];
        board_size
    ];
    for ((y, x), t) in b.iter() {
        tiles[(*y + board_size as i32 / 2) as usize][(*x + board_size as i32 / 2) as usize] =
            BoardTile {
                id: t.tile.to_id(),
                rot: t.rot,
                meeple_id: match t.meeple_id {
                    Some(mid) => mid,
                    None => -1,
                },
                meeple_pos: match t.meeple_pos {
                    Some(mpos) => mpos,
                    None => -1,
                },
            }
    }

    Ok(Board {
        player0_point,
        player1_point,
        tiles,
        meepleable_positions,
        complete_events,
    })
}
