use std::collections::HashMap;

use crate::{database, game::solver::Win};

pub fn calculate_rating(
    player_a_rating: i32,
    player_b_rating: i32,
    player_a_won: bool,
) -> (i32, i32) {
    let mut ra = f64::from(player_a_rating);
    let mut rb = f64::from(player_b_rating);
    let base: f64 = 10.0;
    let k = 32.0;
    let win_ab = 1.0 / (base.powf((rb - ra) / 400.0) + 1.0);
    let win_ba = 1.0 - win_ab;
    if player_a_won {
        ra = ra + k * win_ba;
        rb = rb - k * win_ba;
    } else {
        ra = ra - k * win_ab;
        rb = rb + k * win_ab;
    }
    (ra.round() as i32, rb.round() as i32)
}

#[test]
fn calculate_rating_test() {
    let (ra, rb) = calculate_rating(1500, 1700, true);
    assert_eq!(ra, 1524);
    assert_eq!(rb, 1676);
    let (ra, rb) = calculate_rating(1700, 1500, false);
    assert_eq!(ra, 1676);
    assert_eq!(rb, 1524);
    let (ra, rb) = calculate_rating(1800, 1800, true);
    assert_eq!(ra, 1816);
    assert_eq!(rb, 1784);

    let games = match database::get_games(None) {
        Ok(gs) => gs,
        Err(_) => panic!(""),
    };
    let mut finished_games = vec![];
    let mut rating = HashMap::new();
    for game in &games {
        if game.current_tile_id == Some(-1)
            && game.player0_name != "KokiYamaguchi"
            && game.player0_point != 0
            && game.id > 760
        {
            finished_games.push(game.clone());
            if !rating.contains_key(&game.player0_name) {
                rating.insert(&game.player0_name, 1500);
            }
            if !rating.contains_key(&game.player1_name) {
                rating.insert(&game.player1_name, 1500);
            }
        }
    }
    finished_games.reverse();

    for r in &rating {
        println!("r = {:?}", r);
    }

    for game in &finished_games {
        let (winner, loser) = if game.player0_point > game.player1_point {
            (game.player0_name.clone(), game.player1_name.clone())
        } else {
            (game.player1_name.clone(), game.player0_name.clone())
        };
        let winner_r = *rating.get(&winner).unwrap();
        let loser_r = *rating.get(&loser).unwrap();
        println!(
            "game {:?}: {:?} (with rating {:?}) wins against {:?} (with rating {:?})",
            game.id, winner, winner_r, loser, loser_r
        );
        let (winner_r, loser_r) = calculate_rating(winner_r, loser_r, true);
        *rating.get_mut(&winner).unwrap() = winner_r;
        *rating.get_mut(&loser).unwrap() = loser_r;
    }

    let mut res = vec![];
    for r in &rating {
        res.push((r.0, r.1));
    }

    res.sort_by(|a, b| b.1.cmp(&a.1));

    for r in res {
        println!("{:?}: {:?}", r.0, r.1);
    }

    assert!(true);
}
