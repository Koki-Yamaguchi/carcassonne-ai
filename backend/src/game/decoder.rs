use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use super::calculate::calculate;
use super::mov::{DiscardMove, MeepleMove, Move, TileMove};
use super::tile::to_tile;
use super::tile::Tile;

pub fn decode(data: String) -> Vec<Move> {
    let v: Value = serde_json::from_str(&data).unwrap();

    let mut ord = 0;
    let mut player_id = 0;
    let mut tile_id = -1;

    let mut remaining_meeples = vec![
        HashSet::from([0, 1, 2, 3, 4, 5, 6]),     // player0
        HashSet::from([7, 8, 9, 10, 11, 12, 13]), // player1
    ];

    let mut moves = vec![
        Move::TMove(TileMove {
            id: -1,
            ord,
            game_id: None,
            player_id: player_id,
            tile: Tile::StartingTile,
            rot: 0,
            pos: (0, 0),
        }),
        Move::MMove(MeepleMove {
            id: -1,
            ord: ord + 1,
            game_id: None,
            player_id: player_id,
            meeple_id: -1,
            tile_pos: (0, 0),
            meeple_pos: -1,
        }),
    ];
    ord += 2;
    player_id = 1 - player_id;

    if let Some(packets) = v["data"]["data"].as_array() {
        for packet in packets {
            if let Some(ds) = packet["data"].as_array() {
                for d in ds {
                    let args = &d["args"];
                    match d["type"].as_str() {
                        Some("playTile") | Some("cantPlay") => {
                            // create empty meeple move if there is no playPartisan packet before
                            let prev_move = moves.last().unwrap().clone();
                            match prev_move {
                                Move::TMove(t) => {
                                    moves.push(Move::MMove(MeepleMove {
                                        id: -1,
                                        ord,
                                        game_id: None,
                                        player_id,
                                        meeple_id: -1,
                                        tile_pos: t.pos,
                                        meeple_pos: -1,
                                    }));
                                    ord += 1;
                                    player_id = 1 - player_id;
                                }
                                _ => {}
                            }

                            // check which meeples are retrieved (which can't be known easily from the data)
                            let status = calculate(&moves, false);
                            match status {
                                Ok(res) => {
                                    for e in &res.complete_events {
                                        for meeple_id in &e.meeple_ids {
                                            if *meeple_id < 7 {
                                                remaining_meeples[0].insert(*meeple_id);
                                            } else {
                                                remaining_meeples[1].insert(*meeple_id);
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    panic!("Error: {:?}", e.detail);
                                }
                            }

                            match d["type"].as_str() {
                                Some("playTile") => {
                                    let tile_type = parse_number(&args["type"]);
                                    let y = parse_number(&args["y"]);
                                    let x = parse_number(&args["x"]);
                                    let rot = parse_number(&args["ori"]) - 1;
                                    tile_id = convert_from_bga_tile_type_to_tile_id(tile_type);

                                    moves.push(Move::TMove(TileMove {
                                        id: -1,
                                        ord,
                                        game_id: None,
                                        player_id,
                                        tile: to_tile(tile_id),
                                        rot,
                                        pos: (y, x),
                                    }));

                                    ord += 1;
                                }
                                Some("cantPlay") => {
                                    let bga_tile_id = parse_number(&args["tile_id"]);
                                    let tile_id = convert_from_bga_tile_type_to_tile_id(
                                        convert_from_bga_tile_id_to_bga_tile_type(bga_tile_id),
                                    );

                                    moves.push(Move::DMove(DiscardMove {
                                        id: -1,
                                        ord,
                                        tile: to_tile(tile_id),
                                        game_id: None,
                                        player_id,
                                    }));

                                    ord += 1;
                                }
                                _ => {}
                            }
                        }
                        Some("playPartisan") => {
                            let y = parse_number(&args["y"]);
                            let x = parse_number(&args["x"]);
                            let pos = parse_number(&args["pos"]);

                            if remaining_meeples[player_id as usize].len() == 0 {
                                panic!("decode failed: no meeple is available");
                            }
                            let meeple_id = remaining_meeples[player_id as usize]
                                .iter()
                                .next()
                                .unwrap()
                                .clone();
                            remaining_meeples[player_id as usize].remove(&meeple_id);

                            moves.push(Move::MMove(MeepleMove {
                                id: -1,
                                ord,
                                game_id: None,
                                player_id,
                                meeple_id,
                                tile_pos: (y, x),
                                meeple_pos: convert_pos(tile_id, pos),
                            }));

                            ord += 1;
                            player_id = 1 - player_id;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // add empty meeple move if there is no playPartisan packet in the end
    match moves.last().unwrap() {
        Move::TMove(t) => {
            moves.push(Move::MMove(MeepleMove {
                id: -1,
                ord,
                game_id: None,
                player_id,
                meeple_id: -1,
                tile_pos: t.pos,
                meeple_pos: -1,
            }));
        }
        _ => {}
    }

    moves
}

#[allow(dead_code)]
pub fn decode_from_file_path(file_path: String) -> Vec<Move> {
    let mut file = File::open(file_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    decode(data)
}

fn convert_from_bga_tile_type_to_tile_id(typ: i32) -> i32 {
    match typ {
        15 => 0,
        20 => 1,
        21 => 2,
        14 => 3,
        5 => 4,
        6 => 5,
        16 => 6,
        11 => 7,
        9 => 8,
        18 => 9,
        17 => 10,
        19 => 11,
        7 => 12,
        8 => 13,
        12 => 14,
        13 => 15,
        23 => 16,
        24 => 17,
        10 => 18,
        1 => 19,
        2 => 20,
        3 => 21,
        4 => 22,
        22 => 23,
        _ => -1,
    }
}

fn convert_from_bga_tile_id_to_bga_tile_type(tid: i32) -> i32 {
    match tid {
        1 => 1,
        2 => 2,
        3 => 2,
        4 => 3,
        5 => 3,
        6 => 3,
        7 => 4,
        8 => 4,
        9 => 5,
        10 => 5,
        11 => 5,
        12 => 6,
        13 => 6,
        14 => 7,
        15 => 8,
        16 => 8,
        17 => 9,
        18 => 9,
        19 => 10,
        20 => 10,
        21 => 10,
        22 => 11,
        23 => 11,
        24 => 11,
        25 => 11,
        26 => 11,
        27 => 12,
        28 => 12,
        29 => 12,
        30 => 13,
        31 => 13,
        32 => 13,
        33 => 14,
        34 => 14,
        35 => 14,
        36 => 15,
        37 => 15,
        38 => 15,
        39 => 15,
        40 => 16,
        41 => 16,
        42 => 16,
        43 => 16,
        44 => 16,
        45 => 16,
        46 => 16,
        47 => 16,
        48 => 17,
        49 => 17,
        50 => 17,
        51 => 17,
        52 => 17,
        53 => 17,
        54 => 17,
        55 => 17,
        56 => 17,
        57 => 18,
        58 => 18,
        59 => 18,
        60 => 18,
        61 => 19,
        62 => 20,
        63 => 20,
        64 => 20,
        65 => 20,
        66 => 21,
        67 => 21,
        68 => 22,
        69 => 23,
        70 => 23,
        71 => 23,
        72 => 24,
        _ => -1,
    }
}

fn convert_pos(tile_id: i32, pos: i32) -> i32 {
    match tile_id {
        0 => match pos {
            2 => 0,
            3 => 1,
            1 => 2,
            4 => 3,
            _ => -1,
        },
        1 => match pos {
            1 => 0,
            2 => 1,
            _ => -1,
        },
        2 => match pos {
            1 => 0,
            3 => 1,
            2 => 2,
            _ => -1,
        },
        3 => match pos {
            4 => 0,
            5 => 1,
            1 => 2,
            2 => 3,
            6 => 4,
            3 => 5,
            7 => 6,
            _ => -1,
        },
        4 | 5 => match pos {
            2 => 0,
            3 => 1,
            1 => 2,
            4 => 3,
            _ => -1,
        },
        6 => match pos {
            3 => 0,
            1 => 1,
            4 => 2,
            _ => -1,
        },
        7 => match pos {
            1 => 0,
            2 => 1,
            _ => -1,
        },
        8 => match pos {
            1 => 0,
            2 => 1,
            3 => 2,
            _ => -1,
        },
        9 => match pos {
            4 => 0,
            1 => 1,
            2 => 2,
            5 => 3,
            3 => 4,
            6 => 5,
            _ => -1,
        },
        10 => match pos {
            2 => 0,
            1 => 1,
            3 => 2,
            _ => -1,
        },
        11 => match pos {
            5 => 0,
            4 => 1,
            6 => 2,
            1 => 3,
            2 => 4,
            8 => 5,
            3 => 6,
            7 => 7,
            _ => -1,
        },
        12 | 13 => match pos {
            2 => 0,
            1 => 1,
            3 => 2,
            _ => -1,
        },
        14 | 15 => match pos {
            2 => 0,
            3 => 1,
            1 => 2,
            4 => 3,
            _ => -1,
        },
        16 | 17 => match pos {
            1 => 0,
            2 => 1,
            _ => -1,
        },
        18 => match pos {
            1 => 0,
            3 => 1,
            2 => 2,
            _ => -1,
        },
        19 | 20 => match pos {
            2 => 0,
            3 => 1,
            1 => 2,
            4 => 3,
            _ => -1,
        },
        21 | 22 => match pos {
            1 => 0,
            2 => 1,
            _ => -1,
        },
        23 => match pos {
            1 => 0,
            _ => -1,
        },
        _ => -1,
    }
}

fn parse_number(v: &Value) -> i32 {
    match v {
        Value::Number(n) => n.as_i64().unwrap() as i32,
        Value::String(s) => s.parse().unwrap(),
        _ => panic!("parse failed for {:?}", v),
    }
}
