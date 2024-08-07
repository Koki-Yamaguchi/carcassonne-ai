use std::collections::HashMap;

use super::calculate::{calculate, calculate_tileable_positions};
use super::mov::{MeepleMove, Move, Move::*, TileMove};
use super::tile;
use super::tile::Tile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Win {
    pos: (i32, i32),
    rot: i32,
    meeple_pos: i32,
    win_player_id: i32,
}

#[derive(Debug, PartialEq)]
pub enum SolveResult {
    AlwaysWin,
    Winnable,
    AlwaysLose,
}

#[allow(dead_code)]
pub fn next_permutation<T: std::cmp::Ord>(arr: &mut [T]) -> bool {
    use std::cmp::Ordering;
    let last_ascending = match arr.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            arr.reverse();
            return false;
        }
    };
    let swap_with = arr[last_ascending + 1..]
        .binary_search_by(|n| match arr[last_ascending].cmp(n) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        })
        .unwrap_err();
    arr.swap(last_ascending, last_ascending + swap_with);
    arr[last_ascending + 1..].reverse();
    true
}

#[allow(dead_code)]
pub fn search(
    game_id: Option<i32>,
    mvs: &Vec<Move>,
    ordered_tiles: Vec<Tile>,
    player_id: i32,
    other_player_id: i32,
    player0_id: i32,
    player1_id: i32,
    depth: i32,
    second_player_id: i32,
    is_last_1_or_2: bool,
    debug: bool,
) -> (Vec<Win>, i32) {
    let mut moves = mvs.clone();
    let next_tile = ordered_tiles.first().unwrap();

    // O(n log n)
    let tileable_positions = calculate_tileable_positions(&moves, *next_tile);

    let next_ordered_tiles = ordered_tiles[1..].to_vec();

    let mut wins = vec![];
    let mut win = false;

    // FIXME: meepleable_positions must be calculated more efficiently
    for tileable_position in &tileable_positions {
        if (*next_tile == Tile::Monastery
            || *next_tile == Tile::QuadrupleRoad
            || *next_tile == Tile::QuadrupleCityWithCOA)
            && tileable_position.rot != 1
        {
            continue;
        }
        if (*next_tile == Tile::VerticalSeparator
            || *next_tile == Tile::Connector
            || *next_tile == Tile::ConnectorWithCOA
            || *next_tile == Tile::Straight)
            && (tileable_position.rot == 3 || tileable_position.rot == 4)
        {
            continue;
        }

        moves.push(TMove(TileMove {
            id: -1,
            ord: -1,
            game_id: game_id,
            player_id,
            tile: *next_tile,
            rot: tileable_position.rot,
            pos: tileable_position.pos,
        }));

        let (mut meepleable_positions, meeples0, meeples1) = match calculate(&moves, false) {
            Ok(s) => (
                s.meepleable_positions,
                s.player0_remaining_meeples,
                s.player1_remaining_meeples,
            ),
            Err(e) => {
                panic!("{}", e.detail.msg);
            }
        };
        meepleable_positions.push(-1); // not meeple the tile

        if debug && depth == 0 {
            println!("first mv = {:?}", moves.last().unwrap());
        }

        for meepleable_position in &meepleable_positions {
            let mut meeple_id = -1;
            if *meepleable_position != -1 {
                if player_id == player0_id {
                    if meeples0.len() == 0 {
                        continue;
                    }
                    meeple_id = meeples0.iter().next().unwrap().clone();
                } else {
                    if meeples1.len() == 0 {
                        continue;
                    }
                    meeple_id = meeples1.iter().next().unwrap().clone();
                }
            }

            moves.push(MMove(MeepleMove {
                id: -1,
                ord: -1,
                game_id: game_id,
                player_id,
                meeple_id,
                tile_pos: tileable_position.pos,
                meeple_pos: *meepleable_position,
            }));

            if next_ordered_tiles.len() == 0 {
                match calculate(&moves, true) {
                    Ok(s) => {
                        let winner = if s.player0_point > s.player1_point {
                            player0_id
                        } else if s.player0_point < s.player1_point {
                            player1_id
                        } else {
                            second_player_id
                        };
                        let w = Win {
                            pos: tileable_position.pos,
                            rot: tileable_position.rot,
                            meeple_pos: *meepleable_position,
                            win_player_id: winner,
                        };
                        wins.push(w);
                        if winner == player_id {
                            win = true;
                        }
                    }
                    Err(e) => {
                        panic!("{}", e.detail.msg);
                    }
                }
            } else {
                let (_res, winner) = search(
                    game_id,
                    &moves,
                    next_ordered_tiles.clone(),
                    other_player_id,
                    player_id,
                    player0_id,
                    player1_id,
                    depth + 1,
                    second_player_id,
                    is_last_1_or_2,
                    debug,
                );

                if winner == player_id {
                    win = true;
                }
                wins.push(Win {
                    pos: tileable_position.pos,
                    rot: tileable_position.rot,
                    meeple_pos: *meepleable_position,
                    win_player_id: winner,
                });
            }
            moves.pop(); // pop meeple move

            if win && (is_last_1_or_2 || depth != 0) {
                break;
            }
        }
        moves.pop(); // pop tile move

        if win && (is_last_1_or_2 || depth != 0) {
            break;
        }
    }

    (wins, if win { player_id } else { other_player_id })
}

#[allow(dead_code)]
pub fn solve(
    moves: &Vec<Move>,
    game_id: Option<i32>,
    player0_id: i32,
    player1_id: i32,
    next_tile: Tile,
    is_last_1_or_2: bool,
    debug: bool,
) -> ((TileMove, MeepleMove), SolveResult) {
    // check who is playing
    assert!(moves.len() >= 4);
    let second_player_id = moves[0].player_id();

    let last_move = moves.last().unwrap();
    let next_player_id = match last_move {
        Move::TMove(_) => {
            panic!("can't solve because the last move is tile move");
        }
        Move::MMove(mm) => {
            if player0_id == mm.player_id {
                player1_id
            } else {
                player0_id
            }
        }
        Move::DMove(dm) => dm.player_id,
        _ => {
            panic!("the last move is invalid");
        }
    };
    let other_player_id = if next_player_id == player0_id {
        player1_id
    } else {
        player0_id
    };

    let remaining_meeples = match calculate(moves, false) {
        Ok(s) => {
            if next_player_id == player0_id {
                s.player0_remaining_meeples
            } else {
                s.player1_remaining_meeples
            }
        }
        Err(e) => panic!("{:?}", e.detail.msg),
    };
    let meeple_id = if remaining_meeples.len() > 0 {
        *remaining_meeples.iter().next().unwrap()
    } else {
        -1
    };

    let last_ord = moves.last().unwrap().ord();

    // check what remaining tiles are
    let mut out_tiles = vec![next_tile];
    for mv in moves {
        match mv {
            Move::TMove(tm) => {
                out_tiles.push(tm.tile.clone());
            }
            Move::DMove(dm) => {
                out_tiles.push(dm.tile.clone());
            }
            _ => {}
        }
    }

    let remaining_tiles = tile::remaining_tiles(out_tiles.clone());

    let remaining_tiles_num = remaining_tiles.len();
    let mut remaining_tiles_idx: Vec<usize> = (0..remaining_tiles_num).collect();

    let mut order_count = 0;
    let mut win_count = HashMap::<(i32, i32, i32, i32), i32>::new();
    let mut total_wins = vec![];

    let mut memo = HashMap::<Vec<Tile>, Vec<Win>>::new();

    loop {
        let mut ordered_remaining_tiles = remaining_tiles_idx
            .clone()
            .into_iter()
            .map(|idx| remaining_tiles[idx])
            .collect();
        let mut ordered_tiles = vec![next_tile];
        ordered_tiles.append(&mut ordered_remaining_tiles);

        if debug {
            println!("ordered_tiles = {:?}", ordered_tiles);
        }

        let wins = if memo.contains_key(&ordered_tiles) {
            memo.get(&ordered_tiles).unwrap().clone().to_vec()
        } else {
            let ws = search(
                game_id,
                &moves,
                ordered_tiles.clone(),
                next_player_id,
                other_player_id,
                player0_id,
                player1_id,
                0,
                second_player_id,
                is_last_1_or_2,
                debug,
            )
            .0;
            memo.insert(ordered_tiles.clone(), ws.clone());
            ws.clone()
        };

        for win in &wins {
            if win.win_player_id != next_player_id {
                continue;
            }
            total_wins.push(*win);
        }

        order_count += 1;

        if !next_permutation(&mut remaining_tiles_idx) {
            break;
        }
    }

    for win in &total_wins {
        win_count.insert((win.pos.0, win.pos.1, win.rot, win.meeple_pos), 0);
    }
    for win in &total_wins {
        win_count
            .entry((win.pos.0, win.pos.1, win.rot, win.meeple_pos))
            .and_modify(|v| *v += 1);
    }

    let mut tm = TileMove {
        id: -1,
        ord: last_ord + 1,
        game_id: game_id,
        player_id: next_player_id,
        tile: next_tile,
        rot: 0,
        pos: (-1, -1),
    };
    let mut mm = MeepleMove {
        id: -1,
        ord: last_ord + 2,
        game_id: game_id,
        player_id: next_player_id,
        meeple_id: -1,
        tile_pos: (-1, -1),
        meeple_pos: -1,
    };

    if win_count.len() == 0 {
        println!("====== Always Lose ======");
        return ((tm, mm), SolveResult::AlwaysLose);
    }

    let mut max_count = -1;
    for (key, value) in &win_count {
        if *value > max_count {
            max_count = *value;
            tm = TileMove {
                id: -1,
                ord: tm.ord,
                game_id: tm.game_id,
                player_id: tm.player_id,
                tile: tm.tile,
                rot: key.2,
                pos: (key.0, key.1),
            };
            mm = MeepleMove {
                id: -1,
                ord: mm.ord,
                game_id: mm.game_id,
                player_id: mm.player_id,
                meeple_id: if key.3 == -1 { -1 } else { meeple_id },
                tile_pos: (key.0, key.1),
                meeple_pos: key.3,
            };
        }
    }

    println!("====== Winnable Moves ======");
    for (key, value) in &win_count {
        println!(
            "position ({}, {}), rotated {} times, meeple on feature {} (win probability {}%)",
            key.1,
            -key.0,
            key.2 % 4,
            key.3,
            *value as f64 * 100.0 / order_count as f64
        );
    }
    println!();

    let solve_result = if max_count == order_count {
        SolveResult::AlwaysWin
    } else {
        SolveResult::Winnable
    };

    ((tm, mm), solve_result)
}

#[allow(dead_code)]
fn add_move(
    mvs: &mut Vec<Move>,
    player_id: i32,
    tile: Tile,
    rot: i32,
    pos: (i32, i32),
    meeple_id: i32,
    meeple_pos: i32,
) {
    mvs.push(Move::TMove(TileMove {
        id: -1,
        ord: -1,
        game_id: None,
        player_id,
        tile,
        rot,
        pos,
    }));
    mvs.push(Move::MMove(MeepleMove {
        id: -1,
        ord: -1,
        game_id: None,
        player_id,
        meeple_id: meeple_id,
        tile_pos: pos,
        meeple_pos,
    }));
}

#[test]
fn solve_test0() {
    // this test works, but it takes about 40 sec only for this test.
    // actual game here: https://boardgamearena.com/table?table=367130620
    /*
    use super::decoder;
    let mut mvs = decoder::decode_from_file_path("src/data/367130620.json".to_string());

    mvs.pop();
    mvs.pop();
    mvs.pop();
    mvs.pop();
    mvs.pop();
    mvs.pop();

    let ((tm, mm), solve_result) = solve(&mvs, -1, 0, 1, Tile::VerticalSeparator, false, true);

    assert_eq!(solve_result, SolveResult::AlwaysWin);
    assert_eq!(tm.pos, (-5, 7));
    assert_eq!(tm.rot, 2);
    assert_eq!(mm.meeple_pos, 1);

    mvs.push(TMove(tm));
    mvs.push(MMove(mm));

    match calculate(&mvs, false) {
        Ok(_) => {}
        Err(e) => panic!("{:?}", e.detail.msg),
    }
    */
}

#[test]
fn solve_test1() {
    // solve 5 moves, not fast enough
    // actual game here: https://boardgamearena.com/table?table=318762179
    /*
    use super::decoder;
    let mut mvs = decoder::decode_from_file_path("src/data/318762179.json".to_string());

    for _ in 0..10 {
        mvs.pop();
    }

    let ((tm, mm), solve_result) = solve(&mvs, -1, 0, 1, Tile::Left, false, true);

    assert_eq!(solve_result, SolveResult::AlwaysWin);
    assert_eq!(tm.pos, (-2, -1));
    assert_eq!(tm.rot, 1);
    assert_eq!(mm.meeple_pos, -1);
    */
}

#[test]
fn solve_test2() {
    use super::decoder;
    let mut mvs = decoder::decode_from_file_path("src/data/449165550.json".to_string());

    for _ in 0..8 {
        mvs.pop();
    }

    for mv in &mvs {
        println!("mv = {:?}", mv);
    }
    let cand_mvs = vec![
        vec![
            TMove(TileMove {
                id: -1,
                ord: 134,
                game_id: None,
                player_id: 0,
                tile: Tile::MonasteryWithRoad,
                rot: 1,
                pos: (-5, 3),
            }),
            MMove(MeepleMove {
                id: -1,
                ord: 135,
                game_id: None,
                player_id: 0,
                meeple_id: 6,
                meeple_pos: 0,
                tile_pos: (-5, 3),
            }),
        ],
        vec![
            TMove(TileMove {
                id: -1,
                ord: 134,
                game_id: None,
                player_id: 0,
                tile: Tile::MonasteryWithRoad,
                rot: 1,
                pos: (-5, 3),
            }),
            MMove(MeepleMove {
                id: -1,
                ord: 135,
                game_id: None,
                player_id: 0,
                meeple_id: 6,
                meeple_pos: 2,
                tile_pos: (-5, 3),
            }),
        ],
        vec![
            TMove(TileMove {
                id: -1,
                ord: 134,
                game_id: None,
                player_id: 0,
                tile: Tile::MonasteryWithRoad,
                rot: 3,
                pos: (-8, 2),
            }),
            MMove(MeepleMove {
                id: -1,
                ord: 135,
                game_id: None,
                player_id: 0,
                meeple_id: 6,
                meeple_pos: 0,
                tile_pos: (-8, 2),
            }),
        ],
        vec![
            TMove(TileMove {
                id: -1,
                ord: 134,
                game_id: None,
                player_id: 0,
                tile: Tile::MonasteryWithRoad,
                rot: 0,
                pos: (-8, 1),
            }),
            MMove(MeepleMove {
                id: -1,
                ord: 135,
                game_id: None,
                player_id: 0,
                meeple_id: 6,
                meeple_pos: 2,
                tile_pos: (-8, 1),
            }),
        ],
        vec![
            TMove(TileMove {
                id: -1,
                ord: 134,
                game_id: None,
                player_id: 0,
                tile: Tile::MonasteryWithRoad,
                rot: 0,
                pos: (-6, -2),
            }),
            MMove(MeepleMove {
                id: -1,
                ord: 135,
                game_id: None,
                player_id: 0,
                meeple_id: 6,
                meeple_pos: 0,
                tile_pos: (-6, -2),
            }),
        ],
    ];

    for cand_mv in &cand_mvs {
        mvs.push(cand_mv[0].clone());
        mvs.push(cand_mv[1].clone());

        let next_tiles = vec![Tile::Curve, Tile::CityCap];
        println!("move = {:?}, {:?}", cand_mv[0], cand_mv[1]);

        for next_tile in &next_tiles {
            println!("next_tile = {:?}", next_tile);
            let ((_, _), _) = solve(&mvs, None, 0, 1, next_tile.clone(), false, true);
        }

        mvs.pop();
        mvs.pop();

        println!("");
    }
}

#[test]
fn solve_test3() {
    use super::decoder;
    let mut mvs = decoder::decode_from_file_path("src/data/473330986.json".to_string());

    for _ in 0..10 {
        mvs.pop();
    }

    for mv in &mvs {
        println!("mv = {:?}", mv);
    }

    let cand_mvs = vec![vec![
        TMove(TileMove {
            id: -1,
            ord: 134,
            game_id: None,
            player_id: 1,
            tile: Tile::TriangleWithRoad,
            rot: 3,
            pos: (3, 4),
        }),
        MMove(MeepleMove {
            id: -1,
            ord: 135,
            game_id: None,
            player_id: 1,
            meeple_id: 13,
            meeple_pos: 0,
            tile_pos: (3, 4),
        }),
    ]];

    for cand_mv in &cand_mvs {
        mvs.push(cand_mv[0].clone());
        mvs.push(cand_mv[1].clone());

        let next_tiles = vec![
            Tile::Curve,
            Tile::TripleRoad,
            Tile::TriangleWithCOA,
            Tile::TripleCityWithRoadWithCOA,
        ];
        println!("move = {:?}, {:?}", cand_mv[0], cand_mv[1]);

        for next_tile in &next_tiles {
            println!("next_tile = {:?}", next_tile);
            let ((_, _), _) = solve(&mvs, None, 0, 1, next_tile.clone(), false, true);
        }

        mvs.pop();
        mvs.pop();

        println!("");
    }
}
