use rocket::serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct BoardTile {
    pub id: i32,
    pub rot: i32,
    pub meeple_id: i32,
    pub meeple_pos: i32,
}
