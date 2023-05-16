use rocket::{http::Status, serde::Serialize};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Detail {
  pub title: String,
  pub msg: String,
}

#[derive(Debug)]
pub struct Error {
  pub status: Status,
  pub detail: Detail,
}

pub fn not_found_error(name: String) -> Error {
  Error {
    status: Status::NotFound,
    detail: Detail {
      title: "not_found".to_string(),
      msg: format!("{} not found", name),
    },
  }
}

pub fn internal_server_error(msg: String) -> Error {
  Error {
    status: Status::InternalServerError,
    detail: Detail {
      title: "internal".to_string(),
      msg,
    }
  }
}

pub fn bad_request_error(msg: String) -> Error {
  Error {
    status: Status::BadRequest,
    detail: Detail {
      title: "bad_request".to_string(),
      msg,
    }
  }
}

pub fn moves_invalid_error(msg: String) -> Error {
  Error {
    status: Status::Conflict,
    detail: Detail {
      title: "moves_invalid".to_string(),
      msg,
    }
  }
}
