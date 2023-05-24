use std::collections::HashMap;

use super::calculate::calculate;
use super::calculate::Feature::*;
use super::calculate::Side::*;
use super::calculate::TileItem;
use super::decoder;
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
    point: i32,
    root_feature_id: usize,
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
) -> Vec<Tile> {
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
        return remaining_tiles.clone();
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
    tiles
}

pub fn count_fitting_tiles(
    board: &HashMap<(i32, i32), TileItem>,
    remaining_tiles: &Vec<Tile>,
    exclude_tile: Tile,
    y: i32,
    x: i32,
) -> i32 {
    list_fitting_tiles(board, remaining_tiles, exclude_tile, y, x).len() as i32
}

pub fn last_n(n: i32) -> i32 {
    let naive = match n {
        0 => 0,
        1 => 20,
        2 => 45,
        3 => 58,
        4 => 64,
        5 => 70,
        6 => 90,
        _ => 95,
    };
    naive
}

pub fn last_n_to_last_m_prob(n: i32, m: i32) -> i32 {
    if n == 0 || m == 0 {
        return 0;
    }
    return last_n(m) * 100 / last_n(n);
}

pub fn evaluate(moves: &Vec<Move>) -> i32 {
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
    let remaining_tiles = tile::remaining_tiles(out_tiles.clone());

    let mut roads = vec![vec![]; 2];
    let mut cities = vec![vec![]; 2];
    let mut monasteries = vec![vec![]; 2];
    let mut fields = vec![vec![]; 2];
    let mut results = vec![0; 2];

    let meeple_value = 30;

    for t in board.values() {
        let fs = t.features();
        for f in &fs {
            let meeple_ids = mf.get_meeples(f.id as usize);
            if meeple_ids.len() == 0 {
                continue;
            }
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
                        id,
                        pos: *tile_id_to_pos.get(&id).unwrap(),
                    })
                    .collect(),
                point: mf.size(f.id as usize) as i32,
                root_feature_id: f.id as usize,
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
                CityFeature => {
                    cities[player].push(feat);
                }
                MonasteryFeature => {
                    monasteries[player].push(feat);
                }
                FieldFeature => {
                    fields[player].push(feat);
                }
            }
        }
    }

    let dy = [0, -1, 0, 1];
    let dx = [1, 0, -1, 0];

    for player in 0..2 {
        for city in &cities[player] {
            let mut complete_prob = 90;
            for tile in &city.tiles {
                let y0 = tile.pos.0;
                let x0 = tile.pos.1;
                let tile = board.get(&(y0, x0)).unwrap().clone();

                for dir0 in 0..dx.len() {
                    if (dx[dir0] == 1 && tile.right() == City)
                        || (dy[dir0] == -1 && tile.top() == City)
                        || (dx[dir0] == -1 && tile.left() == City)
                        || (dy[dir0] == 1 && tile.bottom() == City)
                    {
                        if dx[dir0] == 1 {
                            if !mf.is_same_set(
                                city.root_feature_id,
                                tile.right_features()[0].id as usize,
                            ) {
                                continue;
                            }
                        }
                        if dy[dir0] == -1 {
                            if !mf.is_same_set(
                                city.root_feature_id,
                                tile.top_features()[0].id as usize,
                            ) {
                                continue;
                            }
                        }
                        if dx[dir0] == -1 {
                            if !mf.is_same_set(
                                city.root_feature_id,
                                tile.left_features()[0].id as usize,
                            ) {
                                continue;
                            }
                        }
                        if dy[dir0] == 1 {
                            if !mf.is_same_set(
                                city.root_feature_id,
                                tile.bottom_features()[0].id as usize,
                            ) {
                                continue;
                            }
                        }
                        let y1 = y0 + dy[dir0];
                        let x1 = x0 + dx[dir0];
                        match board.get(&(y1, x1)) {
                            Some(_) => {}
                            None => {
                                let need_tile_count = count_fitting_tiles(
                                    &board,
                                    &remaining_tiles,
                                    Tile::Invalid,
                                    y1,
                                    x1,
                                );
                                let mut fill_prob_coef = 0;
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
                                            let adjacent_fitting_tiles = list_fitting_tiles(
                                                &board,
                                                &remaining_tiles,
                                                Tile::Invalid,
                                                y2,
                                                x2,
                                            );
                                            for adjacent_tile in &adjacent_fitting_tiles {
                                                let mut min = 100;
                                                for rot in 0..4 {
                                                    let t = TileItem {
                                                        tile: adjacent_tile.clone(),
                                                        rot,
                                                        id: -1,
                                                        feature_starting_id: -1,
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
                                                            );

                                                        if min > updated_need_tile_count {
                                                            min = updated_need_tile_count;
                                                        }

                                                        board.remove(&(y2, x2));
                                                    }
                                                }
                                                fill_prob_coef +=
                                                    last_n_to_last_m_prob(need_tile_count, min);
                                            }
                                            tot += adjacent_fitting_tiles.len() as i32;
                                        }
                                    }
                                }
                                let fill_prob = last_n(need_tile_count);
                                if tot != 0 {
                                    fill_prob_coef /= tot;
                                    complete_prob *= fill_prob_coef;
                                    complete_prob /= 100;
                                }
                                complete_prob *= fill_prob;
                                complete_prob /= 100;
                            }
                        }
                    }
                }
            }

            let result = city.point * 10 + ((city.point * 10 + meeple_value) * complete_prob / 100);
            results[player] += result;
        }
    }

    for player in 0..2 {
        for road in &roads[player] {
            let mut complete_prob = 90;
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
                                let need_tile_count = count_fitting_tiles(
                                    &board,
                                    &remaining_tiles,
                                    Tile::Invalid,
                                    y1,
                                    x1,
                                );
                                let mut fill_prob_coef = 0;
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
                                            let adjacent_fitting_tiles = list_fitting_tiles(
                                                &board,
                                                &remaining_tiles,
                                                Tile::Invalid,
                                                y2,
                                                x2,
                                            );
                                            for adjacent_tile in &adjacent_fitting_tiles {
                                                let mut min = 100;
                                                for rot in 0..4 {
                                                    let t = TileItem {
                                                        tile: adjacent_tile.clone(),
                                                        rot,
                                                        id: -1,
                                                        feature_starting_id: -1,
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
                                                            );

                                                        if min > updated_need_tile_count {
                                                            min = updated_need_tile_count;
                                                        }

                                                        board.remove(&(y2, x2));
                                                    }
                                                }
                                                fill_prob_coef +=
                                                    last_n_to_last_m_prob(need_tile_count, min);
                                            }
                                            tot += adjacent_fitting_tiles.len() as i32;
                                        }
                                    }
                                }
                                let fill_prob = last_n(need_tile_count);
                                if tot != 0 {
                                    fill_prob_coef /= tot;
                                    complete_prob *= fill_prob_coef;
                                    complete_prob /= 100;
                                }
                                complete_prob *= fill_prob;
                                complete_prob /= 100;
                            }
                        }
                    }
                }
            }

            let result = road.point * 10 + (meeple_value * complete_prob / 100);
            results[player] += result;
        }
    }

    let ex_dy = [0, -1, -1, -1, 0, 1, 1, 1];
    let ex_dx = [1, 1, 0, -1, -1, -1, 0, 1];

    for player in 0..2 {
        for monastery in &monasteries[player] {
            let mut complete_prob = 90;
            for tile in &monastery.tiles {
                let y0 = tile.pos.0;
                let x0 = tile.pos.1;

                for dir0 in 0..ex_dx.len() {
                    let y1 = y0 + ex_dy[dir0];
                    let x1 = x0 + ex_dx[dir0];
                    match board.get(&(y1, x1)) {
                        Some(_) => {}
                        None => {
                            let need_tile_count = count_fitting_tiles(
                                &board,
                                &remaining_tiles,
                                Tile::Invalid,
                                y1,
                                x1,
                            );
                            let mut fill_prob_coef = 0;
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
                                        let adjacent_fitting_tiles = list_fitting_tiles(
                                            &board,
                                            &remaining_tiles,
                                            Tile::Invalid,
                                            y2,
                                            x2,
                                        );
                                        for adjacent_tile in &adjacent_fitting_tiles {
                                            let mut min = 100;
                                            for rot in 0..4 {
                                                let t = TileItem {
                                                    tile: adjacent_tile.clone(),
                                                    rot,
                                                    id: -1,
                                                    feature_starting_id: -1,
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
                                                        );

                                                    if min > updated_need_tile_count {
                                                        min = updated_need_tile_count;
                                                    }

                                                    board.remove(&(y2, x2));
                                                }
                                            }
                                            fill_prob_coef +=
                                                last_n_to_last_m_prob(need_tile_count, min);
                                        }
                                        tot += adjacent_fitting_tiles.len() as i32;
                                    }
                                }
                            }
                            let fill_prob = last_n(need_tile_count);
                            if tot != 0 {
                                fill_prob_coef /= tot;
                                complete_prob *= fill_prob_coef;
                                complete_prob /= 100;
                            }
                            complete_prob *= fill_prob;
                            complete_prob /= 100;
                        }
                    }
                }
            }

            let point = 9 - mf.get_open_sides(monastery.root_feature_id);
            let result = point * 10 + (((9 - point) * 10 + meeple_value) * complete_prob / 100);
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

    results[0] += s.player0_point * 11;
    results[1] += s.player1_point * 11;

    if s.player0_remaining_meeples.len() == 0 {
        results[0] -= 80;
    }
    if s.player1_remaining_meeples.len() == 0 {
        results[1] -= 80;
    }

    println!("last move = {:?}", moves.last().unwrap());
    println!("result = {}", results[0] - results[1]);
    return results[0] - results[1];
}

#[test]
fn evaluate_test() {
    let mvs = decoder::decode("src/data/379255560.json".to_string());
    evaluate(&mvs);
    assert!(false);
}
