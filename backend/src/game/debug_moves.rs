use std::collections::HashMap;

use crate::game::evaluate::evaluate;

use super::{
    calculate::{calculate, TileItem},
    mov::{MeepleMove, Move, TileMove},
    tile::Tile,
};

#[allow(dead_code)]
pub fn list_evaluate_results(moves: &Vec<Move>, next_tile: Tile) {
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

    let mut results = vec![];

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
        for dir0 in 0..4 {
            let ny = y + dy[dir0];
            let nx = x + dx[dir0];
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
                let mut fit = true;
                for dir1 in 0..dy.len() {
                    let nny = ny + dy[dir1];
                    let nnx = nx + dx[dir1];
                    let op_dir = (dir1 + 2) % 4;
                    match board.get(&(nny, nnx)) {
                        Some(t) => {
                            if t.side_by_dir(op_dir) != tile.side_by_dir(dir1) {
                                fit = false;
                                break;
                            }
                        }
                        None => {}
                    }
                }

                if !fit {
                    continue;
                }

                let tmove = TileMove {
                    id: -1,
                    ord: tile_move_ord,
                    game_id: None,
                    player_id: -1,
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

                let remaining_meeples = s.player1_remaining_meeples;
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
                        game_id: None,
                        player_id: -1,
                        meeple_id,
                        tile_pos: (ny, nx),
                        meeple_pos: *mpos,
                    };
                    mvs.push(Move::MMove(mmove.clone()));

                    let debug = false; /* tmove.pos == (-5, 2) && tmove.rot == 2 && mmove.meeple_pos == 1; */
                    let (res0, res1) = evaluate(&mvs, debug);

                    results.push((tmove.clone(), mmove, res0, res1, res1 - res0));

                    mvs.pop();
                }
                mvs.pop();
            }
        }
    }

    println!("Scores for each move");
    results.sort_by(|a, b| a.4.cmp(&b.4));
    for t in results {
        println!(
            "Tile Move = {:?}, Meeple Move = {:?}, score for opponent = {:?}, score for AI = {:?}, total score = {:?}",
            t.0, t.1, t.2, t.3, t.4
        );
    }
    println!("");
}

#[derive(Debug)]
struct CompareMove {
    pub pos: (i32, i32),
    pub rot: i32,
    pub meeple_pos: i32,
}

#[allow(dead_code)]
fn compare_evaluate_results(moves: &Vec<Move>, next_tile: Tile, compare_moves: &Vec<CompareMove>) {
    let mut mvs = moves.clone();

    let tile_move_ord = mvs.last().unwrap().ord() + 1;
    let meeple_move_ord = tile_move_ord + 1;
    let s = match calculate(&mvs, false) {
        Ok(s) => s,
        Err(e) => {
            panic!("{:?}", e.detail.msg);
        }
    };
    let remaining_meeples = s.player1_remaining_meeples;

    for compare_move in compare_moves {
        println!("move = {:?}", compare_move);
        let tmove = TileMove {
            id: -1,
            ord: tile_move_ord,
            game_id: None,
            player_id: -1,
            tile: next_tile,
            rot: compare_move.rot % 4,
            pos: compare_move.pos,
        };

        let mut meeple_id = -1;
        if compare_move.meeple_pos != -1 {
            if remaining_meeples.len() == 0 {
                continue;
            }
            meeple_id = remaining_meeples.iter().next().unwrap().clone();
        }
        let mmove = MeepleMove {
            id: -1,
            ord: meeple_move_ord,
            game_id: None,
            player_id: -1,
            meeple_id,
            meeple_pos: compare_move.meeple_pos,
            tile_pos: compare_move.pos,
        };

        mvs.push(Move::TMove(tmove.clone()));
        mvs.push(Move::MMove(mmove.clone()));

        let (_, _) = evaluate(&mvs, true);

        mvs.pop();
        mvs.pop();
    }
}

/*
#[test]
fn test_evaluate_results() {
    use diesel::pg::PgConnection;
    use diesel::r2d2::ConnectionManager;
    use diesel::r2d2::Pool;
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

    let game_id = 11720;
    let mut mvs = super::database::list_moves(&db, game_id, None).unwrap();

    mvs = mvs[0..18].to_vec();
    println!("mvs = {:?}", mvs);
    println!();

    let next_tile = Tile::TriangleWithRoad;

    list_evaluate_results(&mvs, next_tile);

    compare_evaluate_results(
        &mvs,
        next_tile,
        &vec![
            CompareMove {
                pos: (-4, 3),
                rot: 0,
                meeple_pos: -1,
            },
            CompareMove {
                pos: (-3, 2),
                rot: 0,
                meeple_pos: -1,
            },
        ],
    );
    assert!(true);
}
*/
