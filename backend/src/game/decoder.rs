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

    match v["data"]["data"].as_array() {
        Some(packets) => {
            for packet in packets {
                // check if there's a discard move
                let mut is_discard = false;
                for d in packet["data"].as_array().unwrap() {
                    if let "cantPlay" = d["type"].as_str().unwrap() {
                        is_discard = true;
                    }
                }
                if is_discard {
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
                        Move::MMove(_) => {
                            player_id = 1 - player_id;
                        }
                        _ => {}
                    }

                    for d in packet["data"].as_array().unwrap() {
                        if let Some(args0) = d["args"].as_object() {
                            if !args0.contains_key("args") {
                                continue;
                            }
                            if let Some(args1) = args0["args"].as_object() {
                                if let Some(tile_type) = args1["tile_id"].as_str() {
                                    let tile_id = convert_tile(tile_type.parse().unwrap());
                                    moves.push(Move::DMove(DiscardMove {
                                        id: -1,
                                        ord,
                                        tile: to_tile(tile_id),
                                        game_id: None,
                                        player_id,
                                    }));
                                    ord += 1;
                                }
                            }
                        }
                    }
                }
                match packet["data"][0]["type"].as_str() {
                    Some(t) => {
                        match t {
                            "playTile" => {
                                // add empty meeple move if there is no playPartisan packet
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
                                    Move::MMove(_) => {
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

                                let tile_type = packet["data"][0]["args"]["type"]
                                    .as_str()
                                    .unwrap()
                                    .parse()
                                    .unwrap();
                                let y = packet["data"][0]["args"]["y"]
                                    .as_str()
                                    .unwrap()
                                    .parse()
                                    .unwrap();
                                let x = packet["data"][0]["args"]["x"]
                                    .as_str()
                                    .unwrap()
                                    .parse()
                                    .unwrap();
                                let rot = packet["data"][0]["args"]["ori"]
                                    .as_str()
                                    .unwrap()
                                    .parse::<i32>()
                                    .unwrap()
                                    - 1;
                                tile_id = convert_tile(tile_type);

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
                            "playPartisan" => {
                                let y = packet["data"][0]["args"]["y"]
                                    .as_str()
                                    .unwrap()
                                    .parse()
                                    .unwrap();
                                let x = packet["data"][0]["args"]["x"]
                                    .as_str()
                                    .unwrap()
                                    .parse()
                                    .unwrap();
                                let pos = packet["data"][0]["args"]["pos"]
                                    .as_str()
                                    .unwrap()
                                    .parse()
                                    .unwrap();

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
                            }
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
        }
        None => {}
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

fn convert_tile(typ: i32) -> i32 {
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
