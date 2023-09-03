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
}
