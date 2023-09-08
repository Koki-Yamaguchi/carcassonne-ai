use std::collections::HashMap;
use std::fmt::Debug;

use super::calculate::calculate;
use super::calculate::Feature::*;
use super::calculate::Side::*;
use super::calculate::TileItem;
use super::mergeable_feature::MergeableFeature;
use super::mov;
use super::mov::Move;
use super::{tile, tile::Tile};

#[derive(Debug, Clone)]
struct Tl {
    id: i32,
    pos: (i32, i32),
}

#[derive(Debug, Clone)]
struct Ft {
    tiles: Vec<Tl>,
    empty_positions: Vec<(i32, i32)>,
    point: i32,
    root_feature_id: usize,
    player0_meeples: i32,
    player1_meeples: i32,
}

#[derive(Debug)]
struct City {
    features: Vec<Ft>,
    connecting_positions: Vec<(i32, i32)>,
}

pub fn is_fitting(board: &HashMap<(i32, i32), TileItem>, t: TileItem, y: i32, x: i32) -> bool {
    let right = match board.get(&(y, x + 1)) {
        Some(t) => t.left(),
        None => NoSide,
    };
    let top = match board.get(&(y - 1, x)) {
        Some(t) => t.bottom(),
        None => NoSide,
    };
    let left = match board.get(&(y, x - 1)) {
        Some(t) => t.right(),
        None => NoSide,
    };
    let bottom = match board.get(&(y + 1, x)) {
        Some(t) => t.top(),
        None => NoSide,
    };
    if right == NoSide && top == NoSide && left == NoSide && bottom == NoSide {
        return false;
    }
    (right == NoSide || right == t.right())
        && (top == NoSide || top == t.top())
        && (left == NoSide || left == t.left())
        && (bottom == NoSide || bottom == t.bottom())
}

pub fn list_fitting_tiles(
    board: &HashMap<(i32, i32), TileItem>,
    remaining_tiles: &Vec<Tile>,
    exclude_tile: Tile,
    y: i32,
    x: i32,
) -> Option<Vec<Tile>> {
    let right = match board.get(&(y, x + 1)) {
        Some(t) => t.left(),
        None => NoSide,
    };
    let top = match board.get(&(y - 1, x)) {
        Some(t) => t.bottom(),
        None => NoSide,
    };
    let left = match board.get(&(y, x - 1)) {
        Some(t) => t.right(),
        None => NoSide,
    };
    let bottom = match board.get(&(y + 1, x)) {
        Some(t) => t.top(),
        None => NoSide,
    };
    if right == NoSide && top == NoSide && left == NoSide && bottom == NoSide {
        return None;
    }
    let mut tiles = vec![];
    let mut excluded = false;
    for rem_tile in remaining_tiles {
        if *rem_tile == exclude_tile && !excluded {
            excluded = true;
            continue;
        }
        let mut t = TileItem {
            tile: rem_tile.clone(),
            rot: 0,
            id: -1,
            feature_starting_id: -1,
            meeple_id: None,
            meeple_pos: None,
        };

        let mut ok = false;
        for _ in 0..4 {
            if (right == NoSide || right == t.right())
                && (top == NoSide || top == t.top())
                && (left == NoSide || left == t.left())
                && (bottom == NoSide || bottom == t.bottom())
            {
                ok = true;
                break;
            }
            t.rotate();
        }
        if ok {
            tiles.push(rem_tile.clone());
        }
    }
    Some(tiles)
}

pub fn count_fitting_tiles(
    board: &HashMap<(i32, i32), TileItem>,
    remaining_tiles: &Vec<Tile>,
    exclude_tile: Tile,
    y: i32,
    x: i32,
) -> Option<i32> {
    match list_fitting_tiles(board, remaining_tiles, exclude_tile, y, x) {
        Some(ts) => Some(ts.len() as i32),
        None => None,
    }
}

pub fn count_fitting_roadends(
    board: &HashMap<(i32, i32), TileItem>,
    remaining_tiles: &Vec<Tile>,
    y: i32,
    x: i32,
) -> Option<i32> {
    let mut roadends = vec![];
    for t in remaining_tiles {
        match t {
            Tile::MonasteryWithRoad
            | Tile::TripleRoad
            | Tile::QuadrupleRoad
            | Tile::CityCapWithCrossroad
            | Tile::TripleCityWithRoad
            | Tile::TripleCityWithRoadWithCOA => {
                roadends.push(t.clone());
            }
            _ => {}
        }
    }
    count_fitting_tiles(board, &roadends, Tile::Invalid, y, x)
}

pub fn last_x_for_city(x: f64) -> i32 {
    if x >= 3.0 {
        last_n_for_city(x as i32)
    } else if 2.7 <= x && x < 3.0 {
        55
    } else if 2.4 <= x && x < 2.7 {
        52
    } else if 2.1 <= x && x < 2.4 {
        49
    } else if 1.8 <= x && x < 2.1 {
        46
    } else if 1.5 <= x && x < 1.8 {
        43
    } else if 0.9 <= x && x < 1.5 {
        40
    } else {
        0
    }
}

pub fn last_n_for_city(n: i32) -> i32 {
    let naive = match n {
        0 => 0,
        1 => 40,
        2 => 48,
        3 => 58,
        4 => 67,
        5 | 6 => 75,
        7 | 8 | 9 | 10 => 80,
        11 | 12 | 13 | 14 => 85,
        15 | 16 | 17 | 18 => 90,
        19 | 20 | 21 | 22 => 95,
        _ => 99,
    };
    naive
}

pub fn last_n(n: i32) -> i32 {
    let naive = match n {
        0 => 0,
        1 => 40,
        2 => 45,
        3 => 50,
        4 => 60,
        5 | 6 => 67,
        7 | 8 | 9 | 10 => 74,
        11 | 12 | 13 | 14 => 81,
        15 | 16 | 17 | 18 => 88,
        19 | 20 | 21 | 22 => 95,
        _ => 99,
    };
    naive
}

fn remaining_meeple_values(num: usize) -> i32 {
    match num {
        0 => -320,
        1 => -250,
        2 => -200,
        3 => -150,
        4 => -110,
        5 => -70,
        6 => -30,
        _ => 0,
    }
}

fn city_fill_probability(
    board: &mut HashMap<(i32, i32), TileItem>,
    remaining_tiles: &Vec<Tile>,
    y0: i32,
    x0: i32,
    debug: bool,
) -> i32 {
    let dy = [0, -1, 0, 1];
    let dx = [1, 0, -1, 0];

    let need_tile_count =
        count_fitting_tiles(board, remaining_tiles, Tile::Invalid, y0, x0).unwrap();

    if need_tile_count == 0 {
        return 0;
    }

    let mut max_block_tile_count = 0;
    let mut min_after_need_tile_count = 100.0;
    for dir0 in 0..dx.len() {
        let y1 = y0 + dy[dir0];
        let x1 = x0 + dx[dir0];
        match board.get(&(y1, x1)) {
            Some(_) => {}
            None => {
                let adjacent_fitting_tiles =
                    match list_fitting_tiles(board, remaining_tiles, Tile::Invalid, y1, x1) {
                        Some(ts) => ts,
                        None => {
                            continue;
                        }
                    };
                let mut min_fitting_count_tiles = vec![];
                for adjacent_tile in &adjacent_fitting_tiles {
                    let mut min = 100;
                    for rot in 0..4 {
                        let t = TileItem {
                            tile: adjacent_tile.clone(),
                            rot,
                            id: -1,
                            feature_starting_id: -1,
                            meeple_id: None,
                            meeple_pos: None,
                        };
                        if is_fitting(&board, t, y1, x1) {
                            board.insert((y1, x1), t);

                            let updated_need_tile_count =
                                count_fitting_tiles(board, remaining_tiles, *adjacent_tile, y0, x0)
                                    .unwrap();
                            if min > updated_need_tile_count {
                                min = updated_need_tile_count;
                            }

                            board.remove(&(y1, x1));
                        }
                    }
                    min_fitting_count_tiles.push(min);
                }
                min_fitting_count_tiles.sort();
                let mut prev = -1;
                let mut increased = false;
                let mut after_need_tile_count = 0.0;
                let mut block_tile_count = 0;
                for e in &min_fitting_count_tiles {
                    if prev == -1 {
                        prev = *e;
                    }
                    if prev != *e {
                        if increased {
                            break;
                        }
                        increased = true;
                        prev = *e;
                    }
                    after_need_tile_count += *e as f64;
                    block_tile_count += 1;
                }
                max_block_tile_count = i32::max(max_block_tile_count, block_tile_count);
                min_after_need_tile_count = f64::min(
                    min_after_need_tile_count,
                    after_need_tile_count / block_tile_count as f64,
                );
            }
        }
    }
    assert!(max_block_tile_count + need_tile_count != 0);
    let block_prob = max_block_tile_count * 100 / (max_block_tile_count + need_tile_count);
    let mut fill_prob = (100 - block_prob) * last_n_for_city(need_tile_count) / 100;
    let blocked_but_fill_prob = block_prob * last_x_for_city(min_after_need_tile_count) / 100;
    fill_prob = fill_prob + blocked_but_fill_prob;

    fill_prob
}

fn fill_probability(
    board: &mut HashMap<(i32, i32), TileItem>,
    remaining_tiles: &Vec<Tile>,
    y0: i32,
    x0: i32,
    only_roadend: bool,
) -> i32 {
    let dy = [0, -1, 0, 1];
    let dx = [1, 0, -1, 0];

    let n_roads = number_of_roads(board, y0, x0);

    let need_tile_count = if only_roadend && n_roads != 2 {
        count_fitting_roadends(board, remaining_tiles, y0, x0).unwrap()
    } else {
        count_fitting_tiles(board, remaining_tiles, Tile::Invalid, y0, x0).unwrap()
    };
    let mut after_need_tile_count = 0;
    let mut tot = 0;
    let mut dead = 0;
    for dir0 in 0..dx.len() {
        let y1 = y0 + dy[dir0];
        let x1 = x0 + dx[dir0];
        match board.get(&(y1, x1)) {
            Some(_) => {}
            None => {
                let adjacent_fitting_tiles =
                    match list_fitting_tiles(board, remaining_tiles, Tile::Invalid, y1, x1) {
                        Some(ts) => ts,
                        None => {
                            continue;
                        }
                    };
                for adjacent_tile in &adjacent_fitting_tiles {
                    let mut min = 100;
                    for rot in 0..4 {
                        let t = TileItem {
                            tile: adjacent_tile.clone(),
                            rot,
                            id: -1,
                            feature_starting_id: -1,
                            meeple_id: None,
                            meeple_pos: None,
                        };
                        if is_fitting(&board, t, y1, x1) {
                            board.insert((y1, x1), t);

                            let updated_need_tile_count = if only_roadend && n_roads != 2 {
                                count_fitting_roadends(board, remaining_tiles, y0, x0).unwrap()
                            } else {
                                count_fitting_tiles(board, remaining_tiles, *adjacent_tile, y0, x0)
                                    .unwrap()
                            };
                            if min > updated_need_tile_count {
                                min = updated_need_tile_count;
                            }

                            board.remove(&(y1, x1));
                        }
                    }
                    if min == 0 {
                        dead += 1;
                    }
                    after_need_tile_count += min;
                }
                tot += adjacent_fitting_tiles.len() as i32;
            }
        }
    }
    let mut fill_prob = last_n(need_tile_count);
    if tot != 0 {
        after_need_tile_count /= tot;
        fill_prob = (fill_prob + last_n(after_need_tile_count)) / 2;
    }
    fill_prob = fill_prob * (100 - dead) / 100; // ?

    fill_prob
}

fn number_of_roads(board: &mut HashMap<(i32, i32), TileItem>, y: i32, x: i32) -> i32 {
    let mut num = 0;
    match board.get(&(y, x)) {
        Some(_) => {
            assert!(false);
        }
        None => {
            match board.get(&(y, x + 1)) {
                Some(t) => {
                    if t.left() == Road {
                        num += 1;
                    }
                }
                None => {}
            }
            match board.get(&(y - 1, x)) {
                Some(t) => {
                    if t.bottom() == Road {
                        num += 1;
                    }
                }
                None => {}
            }
            match board.get(&(y, x - 1)) {
                Some(t) => {
                    if t.right() == Road {
                        num += 1;
                    }
                }
                None => {}
            }
            match board.get(&(y + 1, x)) {
                Some(t) => {
                    if t.top() == Road {
                        num += 1;
                    }
                }
                None => {}
            }
        }
    }
    num
}

fn search(
    board: &HashMap<(i32, i32), TileItem>,
    mf: &mut MergeableFeature,
    tile_id_to_pos: &HashMap<i32, (i32, i32)>,
    connecting_positions: &mut Vec<(i32, i32)>,
    features: &mut Vec<Ft>,
    feature_id: usize,
    remaining_tiles: &Vec<Tile>,
) {
    let dy = [0, -1, 0, 1];
    let dx = [1, 0, -1, 0];
    mf.set_as_done(feature_id);

    let point = mf.size(feature_id) as i32;
    let mut tiles = vec![];

    let tile_ids = mf.get_tile_ids(feature_id);

    let meeples = mf.get_meeples(feature_id);
    let mut player0_meeples = 0;
    let mut player1_meeples = 0;
    for meeple in meeples {
        if meeple < 7 {
            player0_meeples += 1;
        } else {
            player1_meeples += 1;
        }
    }

    let mut empty_positions = vec![];

    for tile_id in tile_ids {
        let (y0, x0) = *tile_id_to_pos.get(&tile_id).unwrap();

        let tile = board.get(&(y0, x0)).unwrap();

        tiles.push(Tl {
            id: tile_id,
            pos: (y0, x0),
        });

        for dir0 in 0..dy.len() {
            let side_features0 = tile.features_by_dir(dir0);
            if side_features0.len() != 1 {
                continue;
            }
            if !mf.is_same_set(feature_id, side_features0[0].id as usize) {
                continue;
            }

            let y1 = y0 + dy[dir0];
            let x1 = x0 + dx[dir0];

            let adjacent_tile = board.get(&(y1, x1));
            match adjacent_tile {
                Some(_) => {
                    continue;
                }
                None => {}
            }

            let mut connecting_city_count = 0;

            let c = count_fitting_tiles(board, remaining_tiles, Tile::Invalid, y1, x1).unwrap();
            if c == 0 {
                if !empty_positions.contains(&(y1, x1)) {
                    empty_positions.push((y1, x1));
                }
                continue;
            }

            for dir1 in 0..dy.len() {
                // going back
                if (dir0 + 2) % 4 == dir1 % 4 {
                    continue;
                }

                let y2 = y1 + dy[dir1];
                let x2 = x1 + dx[dir1];

                let adjacent_adjacent_tile = board.get(&(y2, x2));
                let opposite_dir1 = (dir1 + 2) % 4;

                match adjacent_adjacent_tile {
                    None => {
                        continue;
                    }
                    Some(t) => {
                        let side_features1 = t.features_by_dir(opposite_dir1);
                        if side_features1.len() != 1 {
                            continue;
                        }
                        if side_features1[0].feature != CityFeature {
                            continue;
                        }

                        let adjacent_adjacent_feature_id = side_features1[0].id as usize;

                        if mf.is_same_set(feature_id, adjacent_adjacent_feature_id) {
                            continue;
                        }

                        connecting_city_count += 1;

                        if mf.is_done(adjacent_adjacent_feature_id) {
                            continue;
                        }

                        if !connecting_positions.contains(&(y1, x1)) {
                            connecting_positions.push((y1, x1));
                        }

                        search(
                            board,
                            mf,
                            tile_id_to_pos,
                            connecting_positions,
                            features,
                            adjacent_adjacent_feature_id,
                            remaining_tiles,
                        );
                    }
                }
            }

            if connecting_city_count == 0 {
                if !empty_positions.contains(&(y1, x1)) {
                    empty_positions.push((y1, x1));
                }
            }
        }
    }

    features.push(Ft {
        tiles,
        empty_positions,
        point,
        root_feature_id: feature_id,
        player0_meeples,
        player1_meeples,
    });
}

fn search_cities(
    board: &HashMap<(i32, i32), TileItem>,
    mf: &mut MergeableFeature,
    tile_id_to_pos: &HashMap<i32, (i32, i32)>,
    remaining_tiles: &Vec<Tile>,
) -> Vec<City> {
    let mut cities = vec![];
    for t in board.values() {
        let fs = t.features();
        for f in &fs {
            if f.feature != CityFeature {
                continue;
            }
            if mf.is_completed(f.id as usize) {
                continue;
            }
            if mf.is_done(f.id as usize) {
                continue;
            }

            let mut connecting_positions = vec![];
            let mut features = vec![];

            search(
                board,
                mf,
                tile_id_to_pos,
                &mut connecting_positions,
                &mut features,
                f.id as usize,
                &remaining_tiles,
            );

            cities.push(City {
                connecting_positions,
                features,
            });
        }
    }

    cities
}

pub fn evaluate(moves: &Vec<Move>, debug: bool) -> (i32, i32) {
    let dy = [0, -1, 0, 1];
    let dx = [1, 0, -1, 0];

    let s = match calculate(&moves, false) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e.detail.msg),
    };

    let mut mf = s.mergeable_features;
    let mut board = s.board.clone();
    let tile_id_to_pos = s.tile_id_to_pos;

    let mut out_tiles = vec![];
    for mv in moves {
        match mv {
            mov::Move::TMove(tm) => {
                out_tiles.push(tm.tile.clone());
            }
            _ => {}
        }
    }

    let mut roads = vec![vec![]; 2];
    let mut monasteries = vec![vec![]; 2];
    let mut fields = vec![vec![]; 2];

    let mut results = vec![0; 2];

    let meeple_value = 30;

    let remaining_tiles = tile::remaining_tiles(out_tiles.clone());

    let cities = search_cities(&board, &mut mf, &tile_id_to_pos, &remaining_tiles);

    for city in &cities {
        // FIXME: case for city.connecting_positions.len() >= 2, which feels too complicated
        let mut result0 = 0;
        let mut result1 = 0;
        if city.connecting_positions.len() != 1 {
            for feature in &city.features {
                let mut complete_prob = 100;
                for empty_position in &feature.empty_positions {
                    complete_prob *= city_fill_probability(
                        &mut board,
                        &remaining_tiles,
                        empty_position.0,
                        empty_position.1,
                        debug,
                    );
                    complete_prob /= 100;
                }
                for connecting_position in &city.connecting_positions {
                    complete_prob *= city_fill_probability(
                        &mut board,
                        &remaining_tiles,
                        connecting_position.0,
                        connecting_position.1,
                        debug,
                    );
                    complete_prob /= 100;
                }
                let c = feature.point * 10
                    + ((feature.point * 10 + 20 + meeple_value) * complete_prob / 100);
                if feature.player0_meeples > feature.player1_meeples {
                    result0 = c;
                } else if feature.player0_meeples < feature.player1_meeples {
                    result1 = c;
                }
            }
        }
        if city.connecting_positions.len() == 1 {
            let y0 = city.connecting_positions[0].0;
            let x0 = city.connecting_positions[0].1;
            assert!(2 <= city.features.len() && city.features.len() <= 4);
            let mut total_empty_positions = vec![];
            let mut total_point = 0;
            let mut total_player0_meeples = 0;
            let mut total_player1_meeples = 0;
            let mut player0_point = 0;
            let mut player1_point = 0;
            for feature in &city.features {
                total_point += feature.point;
                total_empty_positions.append(&mut feature.empty_positions.clone());
                if feature.player0_meeples > feature.player1_meeples {
                    player0_point += feature.point;
                } else if feature.player0_meeples < feature.player1_meeples {
                    player1_point += feature.point;
                }
                total_player0_meeples += feature.player0_meeples;
                total_player1_meeples += feature.player1_meeples;
            }

            let fill_prob = city_fill_probability(&mut board, &remaining_tiles, y0, x0, debug);

            if total_player0_meeples == total_player1_meeples {
                // whether it is likely to get completed or not doesn't really matter
                // only how points change matters
                let diff = i32::abs(player0_point - player1_point);
                let c = fill_prob * diff * 10 / 100;
                if player0_point > player1_point {
                    result0 = player0_point * 10;
                    result1 = player1_point * 10 + c;
                } else {
                    result0 = player0_point * 10 + c;
                    result1 = player1_point * 10;
                }
            } else {
                let mut complete_prob = fill_prob;
                for empty_position in &total_empty_positions {
                    let f_p = city_fill_probability(
                        &mut board,
                        &remaining_tiles,
                        empty_position.0,
                        empty_position.1,
                        debug,
                    );
                    complete_prob *= f_p;
                    complete_prob /= 100;
                }
                if total_player0_meeples > total_player1_meeples {
                    let diff = total_point + 1 - player0_point + player1_point;
                    let fill_value = diff * 10 * fill_prob / 100;
                    let complete_value =
                        (total_point * 10 + 20 + meeple_value) * complete_prob / 100;
                    result0 = player0_point * 10 + fill_value + complete_value;
                    result1 = player1_point * 10;
                } else if total_player0_meeples < total_player1_meeples {
                    let diff = total_point + 1 - player1_point + player0_point;
                    let fill_value = diff * 10 * fill_prob / 100;
                    let complete_value =
                        (total_point * 10 + 20 + meeple_value) * complete_prob / 100;
                    result0 = player0_point * 10;
                    result1 = player1_point * 10 + fill_value + complete_value;
                } else {
                    assert!(false);
                }
            }
        }
        results[0] += result0;
        results[1] += result1;
    }

    for t in board.values() {
        let fs = t.features();
        for f in &fs {
            let meeple_ids = mf.get_meeples(f.id as usize);
            if mf.is_completed(f.id as usize) {
                continue;
            }
            if mf.is_done(f.id as usize) {
                continue;
            }
            mf.set_as_done(f.id as usize);

            let mut player0_meeples = 0;
            let mut player1_meeples = 0;
            for meeple_id in &meeple_ids {
                if *meeple_id < 7 {
                    player0_meeples += 1;
                } else {
                    player1_meeples += 1;
                }
            }
            let feat = Ft {
                tiles: mf
                    .get_tile_ids(f.id as usize)
                    .into_iter()
                    .map(|id| Tl {
                        id: -1,
                        pos: *tile_id_to_pos.get(&id).unwrap(),
                    })
                    .collect(),
                point: mf.size(f.id as usize) as i32,
                root_feature_id: f.id as usize,
                empty_positions: vec![],
                player0_meeples: -1,
                player1_meeples: -1,
            };
            let player = if player0_meeples > player1_meeples {
                0
            } else if player0_meeples < player1_meeples {
                1
            } else {
                2
            };
            if player == 2 {
                continue;
            }
            match f.feature {
                RoadFeature => {
                    roads[player].push(feat);
                }
                CityFeature => {}
                MonasteryFeature => {
                    monasteries[player].push(feat);
                }
                FieldFeature => {
                    fields[player].push(feat);
                }
            }
        }
    }

    for player in 0..2 {
        for road in &roads[player] {
            let mut complete_prob = 100;
            let mut need_fill = vec![];
            for tile in &road.tiles {
                let y0 = tile.pos.0;
                let x0 = tile.pos.1;
                let tile = board.get(&(y0, x0)).unwrap().clone();

                for dir0 in 0..dx.len() {
                    if (dx[dir0] == 1 && tile.right() == Road)
                        || (dy[dir0] == -1 && tile.top() == Road)
                        || (dx[dir0] == -1 && tile.left() == Road)
                        || (dy[dir0] == 1 && tile.bottom() == Road)
                    {
                        if dx[dir0] == 1 {
                            if tile.right_features().len() != 3
                                || !mf.is_same_set(
                                    road.root_feature_id,
                                    tile.right_features()[1].id as usize,
                                )
                            {
                                continue;
                            }
                        }
                        if dy[dir0] == -1 {
                            if tile.top_features().len() != 3
                                || !mf.is_same_set(
                                    road.root_feature_id,
                                    tile.top_features()[1].id as usize,
                                )
                            {
                                continue;
                            }
                        }
                        if dx[dir0] == -1 {
                            if tile.left_features().len() != 3
                                || !mf.is_same_set(
                                    road.root_feature_id,
                                    tile.left_features()[1].id as usize,
                                )
                            {
                                continue;
                            }
                        }
                        if dy[dir0] == 1 {
                            if tile.bottom_features().len() != 3
                                || !mf.is_same_set(
                                    road.root_feature_id,
                                    tile.bottom_features()[1].id as usize,
                                )
                            {
                                continue;
                            }
                        }
                        let y1 = y0 + dy[dir0];
                        let x1 = x0 + dx[dir0];
                        match board.get(&(y1, x1)) {
                            Some(_) => {}
                            None => {
                                need_fill.push((y1, x1));
                            }
                        }
                    }
                }
            }

            assert!(need_fill.len() <= 2);
            if need_fill.len() == 2 {
                if need_fill[0].0 == need_fill[1].0 && need_fill[0].1 == need_fill[1].1 {
                    // ring road
                    let y0 = need_fill[0].0;
                    let x0 = need_fill[0].1;
                    complete_prob *= fill_probability(&mut board, &remaining_tiles, y0, x0, false);
                    complete_prob /= 100;
                } else if (need_fill[0].0 == need_fill[1].0
                    && i32::abs(need_fill[0].1 - need_fill[1].1) == 1)
                    || (need_fill[0].1 == need_fill[1].1
                        && i32::abs(need_fill[0].0 - need_fill[1].0) == 1)
                {
                    // could be ring road
                    for (y0, x0) in &need_fill {
                        complete_prob *=
                            fill_probability(&mut board, &remaining_tiles, *y0, *x0, false);
                        complete_prob /= 100;
                    }
                } else {
                    // probably need two road ends
                    for (y0, x0) in &need_fill {
                        complete_prob *=
                            fill_probability(&mut board, &remaining_tiles, *y0, *x0, true);
                        complete_prob /= 100;
                    }
                }
            } else {
                // need one road end
                let y0 = need_fill[0].0;
                let x0 = need_fill[0].1;
                complete_prob *= fill_probability(&mut board, &remaining_tiles, y0, x0, true);
                complete_prob /= 100;
            }

            let result = road.point * 10 + (meeple_value / 2 * complete_prob / 100);
            results[player] += result;
        }
    }

    let ex_dy = [0, -1, -1, -1, 0, 1, 1, 1];
    let ex_dx = [1, 1, 0, -1, -1, -1, 0, 1];

    for player in 0..2 {
        for monastery in &monasteries[player] {
            let mut complete_prob = 100;
            for tile in &monastery.tiles {
                let y0 = tile.pos.0;
                let x0 = tile.pos.1;

                for dir0 in 0..ex_dx.len() {
                    let y1 = y0 + ex_dy[dir0];
                    let x1 = x0 + ex_dx[dir0];
                    match board.get(&(y1, x1)) {
                        Some(_) => {}
                        None => {
                            let need_tile_count = match count_fitting_tiles(
                                &board,
                                &remaining_tiles,
                                Tile::Invalid,
                                y1,
                                x1,
                            ) {
                                Some(c) => c,
                                None => {
                                    continue;
                                }
                            };

                            let mut after_need_tile_count = 0;
                            let mut tot = 0;
                            for dir1 in 0..dx.len() {
                                let y2 = y1 + dy[dir1];
                                let x2 = x1 + dx[dir1];
                                if y2 == y0 && x2 == x0 {
                                    continue;
                                }
                                match board.get(&(y2, x2)) {
                                    Some(_) => {}
                                    None => {
                                        let adjacent_fitting_tiles = match list_fitting_tiles(
                                            &board,
                                            &remaining_tiles,
                                            Tile::Invalid,
                                            y2,
                                            x2,
                                        ) {
                                            Some(ts) => ts,
                                            None => {
                                                continue;
                                            }
                                        };
                                        for adjacent_tile in &adjacent_fitting_tiles {
                                            let mut min = 100;
                                            for rot in 0..4 {
                                                let t = TileItem {
                                                    tile: adjacent_tile.clone(),
                                                    rot,
                                                    id: -1,
                                                    feature_starting_id: -1,
                                                    meeple_id: None,
                                                    meeple_pos: None,
                                                };
                                                if is_fitting(&board, t, y2, x2) {
                                                    board.insert((y2, x2), t);

                                                    let updated_need_tile_count =
                                                        count_fitting_tiles(
                                                            &board,
                                                            &remaining_tiles,
                                                            *adjacent_tile,
                                                            y1,
                                                            x1,
                                                        )
                                                        .unwrap();

                                                    if min > updated_need_tile_count {
                                                        min = updated_need_tile_count;
                                                    }

                                                    board.remove(&(y2, x2));
                                                }
                                            }
                                            after_need_tile_count += min;
                                        }
                                        tot += adjacent_fitting_tiles.len() as i32;
                                    }
                                }
                            }
                            let mut fill_prob = last_n(need_tile_count);
                            if tot != 0 {
                                after_need_tile_count /= tot;
                                fill_prob = (fill_prob + last_n(after_need_tile_count)) / 2;
                            }
                            complete_prob *= fill_prob;
                            complete_prob /= 100;
                        }
                    }
                }
            }

            let point = 9 - mf.get_open_sides(monastery.root_feature_id);
            let result = point * 10 + meeple_value * complete_prob / 100;
            results[player] += result;
        }
    }

    for player in 0..2 {
        for field in &fields[player] {
            let mut p = 0;
            let cs = mf.get_facing_cities(field.root_feature_id as usize);
            for c in &cs {
                if mf.is_completed(*c) {
                    p += 3;
                }
            }
            let result = p * 10;
            results[player] += result;
        }
    }

    results[0] += s.player0_point * 12;
    results[1] += s.player1_point * 12;

    results[0] += remaining_meeple_values(s.player0_remaining_meeples.len());
    results[1] += remaining_meeple_values(s.player1_remaining_meeples.len());

    return (results[0], results[1]);
}

#[test]
fn evaluate_test() {}
