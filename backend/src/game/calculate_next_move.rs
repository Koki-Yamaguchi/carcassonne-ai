use std::collections::HashMap;

use super::calculate::calculate;
use super::calculate::TileItem;
use super::evaluate::evaluate;
use super::mov::{MeepleMove, Move, TileMove};
use super::solver::solve;
use super::solver::SolveResult;
use super::tile::Tile;

pub fn calculate_next_move(
    moves: &Vec<Move>,
    game_id: Option<i32>,
    player0_id: i32,
    player1_id: i32,
    player_id: i32,
    next_tile: Tile,
) -> Option<(TileMove, MeepleMove)> {
    let mut mvs = moves.clone();

    let mut tile_count = 0;
    for mv in &mvs {
        match mv {
            Move::TMove(_) | Move::DMove(_) => {
                tile_count += 1;
            }
            _ => {}
        }
    }
    if tile_count >= 72 - 2 {
        let ((tm, mm), solve_result) =
            solve(&mvs, game_id, player0_id, player1_id, next_tile, true);
        if solve_result != SolveResult::AlwaysLose {
            return Some((tm, mm));
        }
        // if the above doesn't return results, then lose 100% (or `solve` failed to calculate the results fast enough), so just play as usual
    }

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
        id: -1,
        ord: 0,
        game_id,
        player_id: player_id,
        tile: next_tile,
        rot: 0,
        pos: (-1, -1),
    };
    let mut meeple_move = MeepleMove {
        id: -1,
        ord: 1,
        game_id,
        player_id: player_id,
        meeple_id: -1,
        tile_pos: (-1, -1),
        meeple_pos: -1,
    };

    // let mut test = vec![];

    let mut updated = false;
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
            match checked.get(&(ny, nx)) {
                Some(_) => {
                    continue;
                }
                None => {}
            }
            checked.insert((ny, nx), true);

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
                    id: -1,
                    ord: tile_move_ord,
                    game_id,
                    player_id,
                    tile: next_tile,
                    rot: rot % 4,
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
                        id: -1,
                        ord: meeple_move_ord,
                        game_id,
                        player_id,
                        meeple_id,
                        tile_pos: (ny, nx),
                        meeple_pos: *mpos,
                    };
                    mvs.push(Move::MMove(mmove.clone()));

                    // let debug = tmove.pos == (0, 3) && tmove.rot == 3 && mmove.meeple_pos == 2;
                    // let debug = tmove.pos == (0, 4) && tmove.rot == 1 && mmove.meeple_pos == 4;
                    let debug = false;

                    let (res0, res1) = evaluate(&mvs, debug);

                    let val = if player_id == player0_id {
                        res0 - res1
                    } else {
                        res1 - res0
                    };

                    if val > max_val {
                        max_val = val;
                        tile_move = tmove.clone();
                        meeple_move = mmove.clone();
                        updated = true;
                    }

                    mvs.pop();
                }
                mvs.pop();
            }
        }
    }

    if !updated {
        return None;
    }

    /*
    test.sort_by(|a, b| a.4.cmp(&b.4));
    for t in test {
        println!(
            "m = {:?}, {:?}, res0 = {:?}, res1 = {:?}, val = {:?}",
            t.0, t.1, t.2, t.3, t.4
        );
    }
    println!("");
    */

    Some((tile_move, meeple_move))
}

#[test]
fn calculate_next_move_invade_test0() {
    let src_mvs =
        super::decoder::decode_from_file_path("src/data/365601037/moves.json".to_string());
    let mvs = src_mvs[0..10].to_vec();

    // 365601037/0.png
    let (tile_move, meeple_move) =
        calculate_next_move(&mvs, None, 0, 1, 1, Tile::Triangle).unwrap();
    assert_eq!(tile_move.pos, (-1, 1));
    assert_eq!(tile_move.rot, 1);
    assert_eq!(meeple_move.meeple_pos, 0);

    // 365601037/1.png
    let mvs = src_mvs[0..22].to_vec();
    let (tile_move, meeple_move) =
        calculate_next_move(&mvs, None, 0, 1, 1, Tile::TriangleWithCOA).unwrap();
    assert_eq!(tile_move.pos, (-2, 2));
    assert_eq!(tile_move.rot, 3);
    assert_eq!(meeple_move.meeple_pos, -1);

    /*
    // FIXME
    // 365601037/2.png
    let mvs = src_mvs[0..34].to_vec();
    let (tile_move, meeple_move) =
        calculate_next_move(&mvs, -1, 0, 1, 1, Tile::ConnectorWithCOA).unwrap();
    assert_eq!(tile_move.pos, (-3, 2));
    assert_eq!(tile_move.rot % 2, 0);
    assert_eq!(meeple_move.meeple_pos, -1);
    */
}

#[test]
fn calculate_next_move_test() {}
