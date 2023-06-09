use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::error::{internal_server_error, not_found_error, Error};
use crate::game;
use crate::game::mov;
use crate::game::tile;
use crate::player;
use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::player)]
struct NewPlayer {
    name: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::game)]
struct NewGame {
    note: String,
    player0_id: i32,
    player1_id: i32,
    player0_point: i32,
    player1_point: i32,
    next_tile_id: Option<i32>,
    next_player_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::move_)]
pub struct InsertMove {
    pub ord: i32,
    pub game_id: i32,
    pub player_id: i32,
    pub tile_id: i32,
    pub meeple_id: i32,
    pub rot: i32,
    pub tile_pos_y: i32,
    pub tile_pos_x: i32,
    pub meeple_pos: i32,
}
#[derive(Queryable)]
#[diesel(table_name = schema::move_)]
pub struct QueryMove {
    pub id: i32,
    pub ord: i32,
    pub game_id: i32,
    pub player_id: i32,
    pub tile_id: i32,
    pub meeple_id: i32,
    pub rot: i32,
    pub tile_pos_y: i32,
    pub tile_pos_x: i32,
    pub meeple_pos: i32,
}

pub fn create_player(name: String) -> Result<player::Player, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    let new_player = NewPlayer { name: name };
    match diesel::insert_into(schema::player::table)
        .values(&new_player)
        .get_result(conn)
    {
        Ok(r) => Ok(r),
        Err(e) => Err(internal_server_error(e.to_string())),
    }
}

pub fn get_games(player_id: Option<i32>) -> Result<Vec<game::Game>, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::game::dsl::{game as g, player0_id, player1_id};

    match player_id {
        Some(pid) => {
            match g
                .filter(player0_id.eq(pid).or(player1_id.eq(pid)))
                .load::<game::Game>(conn)
            {
                Ok(gm) => {
                    return Ok(gm);
                }
                Err(e) => {
                    return Err(internal_server_error(e.to_string()));
                }
            }
        }
        None => match g.load::<game::Game>(conn) {
            Ok(gm) => {
                return Ok(gm);
            }
            Err(e) => {
                return Err(internal_server_error(e.to_string()));
            }
        },
    }
}

pub fn get_game(gmid: i32) -> Result<game::Game, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::game::dsl::{game as g, id};
    match g.filter(id.eq(gmid)).limit(1).load::<game::Game>(conn) {
        Ok(games) => {
            if games.len() == 0 {
                return Err(not_found_error("game".to_string()));
            }
            return Ok(games[0].clone());
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn create_game(
    note: String,
    player0_id: i32,
    player1_id: i32,
    next_tile_id: Option<i32>,
    next_player_id: Option<i32>,
) -> Result<game::Game, Error> {
    let new_game = NewGame {
        note,
        player0_id,
        player1_id,
        player0_point: 0,
        player1_point: 0,
        next_tile_id,
        next_player_id,
    };
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    match diesel::insert_into(schema::game::table)
        .values(&new_game)
        .get_result(conn)
    {
        Ok(gm) => {
            return Ok(gm);
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn update_game(
    gmid: i32,
    next_tid: i32,
    next_pid: i32,
    p0_point: i32,
    p1_point: i32,
) -> Result<game::Game, Error> {
    use self::schema::game::dsl::{
        game, next_player_id, next_tile_id, player0_point, player1_point,
    };
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    match diesel::update(game.find(gmid))
        .set((
            next_tile_id.eq(next_tid),
            next_player_id.eq(next_pid),
            player0_point.eq(p0_point),
            player1_point.eq(p1_point),
        ))
        .get_result(conn)
    {
        Ok(gm) => {
            return Ok(gm);
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn list_moves(gmid: i32, move_id: Option<i32>) -> Result<Vec<mov::Move>, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::move_::dsl::*;
    let max_ord = match move_id {
        Some(mid) => mid,
        None => 1000,
    };
    match move_
        .filter(game_id.eq(gmid))
        .filter(ord.le(max_ord))
        .order(ord.asc())
        .load::<QueryMove>(conn)
    {
        Ok(mvs) => {
            return Ok(mvs.into_iter().map(|v| to_move(v)).collect());
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn create_move(mv: mov::Move) -> Result<mov::Move, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time

    let new_move = match mv {
        mov::Move::TMove(m) => InsertMove {
            ord: m.ord,
            game_id: m.game_id,
            player_id: m.player_id,
            tile_id: m.tile.to_id(),
            meeple_id: -1,
            rot: m.rot,
            tile_pos_y: m.pos.0,
            tile_pos_x: m.pos.1,
            meeple_pos: -1,
        },
        mov::Move::MMove(m) => InsertMove {
            ord: m.ord,
            game_id: m.game_id,
            player_id: m.player_id,
            tile_id: -1,
            meeple_id: m.meeple_id,
            rot: -1,
            tile_pos_y: m.tile_pos.0,
            tile_pos_x: m.tile_pos.1,
            meeple_pos: m.meeple_pos,
        },
        mov::Move::InvalidMove => {
            return Ok(mov::Move::InvalidMove);
        }
    };

    match diesel::insert_into(schema::move_::table)
        .values(&new_move)
        .get_result::<QueryMove>(conn)
    {
        Ok(query_move) => {
            return Ok(to_move(query_move));
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

fn to_move(qm: QueryMove) -> mov::Move {
    match (qm.tile_id, qm.meeple_id) {
        (-1, _) => mov::Move::MMove(mov::MeepleMove {
            ord: qm.ord,
            game_id: qm.game_id,
            player_id: qm.player_id,
            meeple_id: qm.meeple_id,
            meeple_pos: qm.meeple_pos,
            tile_pos: (qm.tile_pos_y, qm.tile_pos_x),
        }),
        (_, -1) => mov::Move::TMove(mov::TileMove {
            ord: qm.ord,
            game_id: qm.game_id,
            player_id: qm.player_id,
            tile: tile::to_tile(qm.tile_id),
            rot: qm.rot,
            pos: (qm.tile_pos_y, qm.tile_pos_x),
        }),
        (_, _) => mov::Move::InvalidMove,
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
