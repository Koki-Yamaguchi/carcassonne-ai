use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(crate = "rocket::serde")]
pub struct UpdateEvent {
    pub name: String,
    pub id: i32,
}
