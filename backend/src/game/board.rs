use rocket::serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct BoardTile {
    pub id: i32,
    pub rot: i32,
    pub meeple_id: i32,
    pub meeple_pos: i32,
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Board {
    pub player0_point: i32,
    pub player1_point: i32,
    pub tiles: Vec<Vec<BoardTile>>,
    pub meepleable_positions: Vec<i32>,
    pub complete_events: Vec<super::CompleteEvent>,
}
