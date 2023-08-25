use std::collections::HashMap;

use super::calculate::calculate;
use super::calculate::TileItem;
use super::evaluate::evaluate;
use super::mov::{MeepleMove, Move, TileMove};
use super::solver::solve;
use super::tile::Tile;

pub fn calculate_next_move(
    moves: &Vec<Move>,
    game_id: i32,
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
        let ((tm, mm), winnable) = solve(&mvs, game_id, player0_id, player1_id, next_tile);
        println!("tm, mm, winnable = {:?}, {:?}, {:?}", tm, mm, winnable);
        if winnable {
            return Some((tm, mm));
        }
        // lose 100% or failed to calculate the results fast enough, so just play as usual
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

                    /*
                    // if !(tmove.pos == (-7, 2) && tmove.rot == 4 && mmove.meeple_pos == 0) {
                    if !(tmove.pos == (-7, 2) && tmove.rot == 4 && mmove.meeple_pos == -1) {
                        mvs.pop();
                        continue;
                    }
                    */

                    let (res0, res1) = evaluate(&mvs);

                    let val = if player_id == player0_id {
                        res0 - res1
                    } else {
                        res1 - res0
                    };

                    // test.push((tmove.clone(), mmove.clone(), res0, res1, val));

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
fn calculate_next_move_test() {
    use super::database;
    let game_id = 442;
    let game = database::get_game(game_id).unwrap();
    let mut mvs = database::list_moves(game_id, None).unwrap();
    mvs.pop();
    mvs.pop();
    let res = calculate_next_move(
        &mvs,
        game_id,
        game.player0_id,
        game.player1_id,
        1,
        Tile::CityCap,
    );
    println!("res = {:?}", res);
    assert!(true);
}
