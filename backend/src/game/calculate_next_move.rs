use std::collections::HashMap;

use super::calculate::calculate;
use super::calculate::TileItem;
use super::evaluate::evaluate;
use super::mov::{MeepleMove, Move, TileMove};
use super::tile::Tile;

pub fn calculate_next_move(
    moves: &Vec<Move>,
    game_id: i32,
    player0_id: i32,
    _player1_id: i32,
    player_id: i32,
    next_tile: Tile,
) -> (TileMove, MeepleMove) {
    let mut mvs = moves.clone();

    let mut tile = TileItem {
        id: next_tile.to_id(),
        tile: next_tile,
        rot: 0,
        feature_starting_id: 0,
        meeple_id: None,
        meeple_pos: None,
    };

    let tile_move_ord = mvs.last().unwrap().ord() + 1;
    let meeple_move_ord = tile_move_ord + 1;

    let board = match calculate(&mvs, false) {
        Ok(s) => s.board,
        Err(_) => HashMap::new(),
    };

    let mut checked = HashMap::new();
    let mut max_val = -100000;
    let mut tile_move = TileMove {
        ord: 0,
        game_id,
        player_id: player_id,
        tile: next_tile,
        rot: 0,
        pos: (-1, -1),
    };
    let mut meeple_move = MeepleMove {
        ord: 1,
        game_id,
        player_id: player_id,
        meeple_id: -1,
        tile_pos: (-1, -1),
        meeple_pos: -1,
    };
    for pos in board.keys() {
        match checked.get(pos) {
            Some(_) => {
                continue;
            }
            None => {}
        }
        checked.insert(pos.clone(), true);

        let y = pos.0;
        let x = pos.1;
        let dy = [0, -1, 0, 1];
        let dx = [1, 0, -1, 0];
        for i in 0..4 {
            let ny = y + dy[i];
            let nx = x + dx[i];
            match board.get(&(ny, nx)) {
                Some(_) => {
                    continue;
                }
                None => {}
            }
            for rot in vec![1, 2, 3, 4] {
                tile.rotate();

                match board.get(&(ny - 1, nx)) {
                    Some(t) => {
                        if t.bottom() != tile.top() {
                            continue;
                        }
                    }
                    None => {}
                }
                match board.get(&(ny + 1, nx)) {
                    Some(t) => {
                        if t.top() != tile.bottom() {
                            continue;
                        }
                    }
                    None => {}
                }
                match board.get(&(ny, nx - 1)) {
                    Some(t) => {
                        if t.right() != tile.left() {
                            continue;
                        }
                    }
                    None => {}
                }
                match board.get(&(ny, nx + 1)) {
                    Some(t) => {
                        if t.left() != tile.right() {
                            continue;
                        }
                    }
                    None => {}
                }
                let tmove = TileMove {
                    ord: tile_move_ord,
                    game_id,
                    player_id,
                    tile: next_tile,
                    rot: rot,
                    pos: (ny, nx),
                };

                mvs.push(Move::TMove(tmove.clone()));

                let s = match calculate(&mvs, false) {
                    Ok(s) => s,
                    Err(e) => {
                        panic!("{:?}", e.detail.msg);
                    }
                };

                let remaining_meeples = if player_id == player0_id {
                    s.player0_remaining_meeples
                } else {
                    s.player1_remaining_meeples
                };
                let mut meepleable_positions = s.meepleable_positions;
                meepleable_positions.push(-1);
                for mpos in &meepleable_positions {
                    let mut meeple_id = -1;
                    if *mpos != -1 {
                        if remaining_meeples.len() == 0 {
                            continue;
                        }
                        meeple_id = remaining_meeples.iter().next().unwrap().clone();
                    }

                    let mmove = MeepleMove {
                        ord: meeple_move_ord,
                        game_id,
                        player_id,
                        meeple_id,
                        tile_pos: (ny, nx),
                        meeple_pos: *mpos,
                    };
                    mvs.push(Move::MMove(mmove.clone()));

                    let val = if player_id == player0_id {
                        evaluate(&mvs)
                    } else {
                        -evaluate(&mvs)
                    };
                    if val > max_val {
                        max_val = val;
                        tile_move = tmove.clone();
                        meeple_move = mmove.clone();
                    }

                    mvs.pop();
                }
                mvs.pop();
            }
        }
    }

    (tile_move, meeple_move)
}
