use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::error::{internal_server_error, not_found_error, Error};
use crate::game;
use crate::game::mov;
use crate::game::tile;
use crate::optimal_move;
use crate::player::{self};
use crate::problem;
use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::player)]
struct NewPlayer {
    name: String,
    email: String,
    user_id: String,
    meeple_color: i32,
    rating: Option<i32>,
}

#[derive(Queryable, Clone)]
#[diesel(table_name = schema::player)]
pub struct QueryPlayer {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub user_id: String,
    pub meeple_color: i32,
    pub rating: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::waiting_game)]
struct NewWaitingGame {
    player_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = schema::game)]
struct NewGame {
    player0_id: i32,
    player1_id: i32,
    player0_point: i32,
    player1_point: i32,
    next_tile_id: Option<i32>,
    next_player_id: Option<i32>,
    current_tile_id: Option<i32>,
    current_player_id: Option<i32>,
    player0_name: String,
    player1_name: String,
    player0_color: i32,
    player1_color: i32,
    is_rated: bool,
    before_player0_rating: Option<i32>,
    before_player1_rating: Option<i32>,
    after_player0_rating: Option<i32>,
    after_player1_rating: Option<i32>,
    first_player_id: Option<i32>,
    winner_player_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::optimal_move)]
struct NewOptimalMove {
    game_id: i32,
    last_n: i32,
    tile_move_id: i32,
    meeple_move_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = schema::move_)]
pub struct InsertMove {
    pub ord: i32,
    pub game_id: Option<i32>,
    pub player_id: i32,
    pub tile_id: i32,
    pub meeple_id: i32,
    pub rot: i32,
    pub tile_pos_y: i32,
    pub tile_pos_x: i32,
    pub meeple_pos: i32,
}
#[derive(Queryable, Clone)]
#[diesel(table_name = schema::move_)]
pub struct QueryMove {
    pub id: i32,
    pub ord: i32,
    pub game_id: Option<i32>,
    pub player_id: i32,
    pub tile_id: i32,
    pub meeple_id: i32,
    pub rot: i32,
    pub tile_pos_y: i32,
    pub tile_pos_x: i32,
    pub meeple_pos: i32,
}

#[derive(Insertable)]
#[diesel(table_name = schema::problem)]
pub struct NewProblem {
    pub game_id: i32,
    pub name: String,
    pub start_at: Option<chrono::NaiveDateTime>,
    pub creator_id: Option<i32>,
    pub creator_name: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::favorite)]
pub struct NewFavorite {
    pub vote_id: i32,
    pub player_id: i32,
    pub player_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::vote)]
pub struct NewVote {
    pub problem_id: i32,
    pub player_id: i32,
    pub player_name: String,
    pub note: String,
    pub favorite_count: i32,
    pub tile_move_id: i32,
    pub meeple_move_id: i32,
}

#[derive(Queryable, Clone)]
#[diesel(table_name = schema::vote)]
pub struct QueryVote {
    id: i32,
    pub problem_id: i32,
    pub player_id: i32,
    pub player_name: String,
    pub note: String,
    pub favorite_count: i32,
    pub tile_move_id: i32,
    pub meeple_move_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

pub fn get_player(pid: i32) -> Result<player::Player, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::player::dsl::{id, player as p};

    match p.filter(id.eq(pid)).load::<QueryPlayer>(conn) {
        Ok(ps) => {
            if ps.len() == 0 {
                return Err(not_found_error("player".to_string()));
            }
            return Ok(to_player(ps[0].clone()));
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn get_players() -> Result<Vec<player::Player>, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::player::dsl::{id, player as p, rating};

    match p
        .filter(rating.is_not_null())
        .filter(id.ne(1)) // not AI
        .order(rating.desc())
        .limit(10)
        .load::<QueryPlayer>(conn)
    {
        Ok(ps) => {
            return Ok(ps.into_iter().map(|v| to_player(v)).collect());
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn get_player_by_uid(uid: String) -> Result<player::Player, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::player::dsl::{player as p, user_id};

    match p.filter(user_id.eq(uid)).load::<QueryPlayer>(conn) {
        Ok(ps) => {
            if ps.len() == 0 {
                return Err(not_found_error("player".to_string()));
            }
            return Ok(to_player(ps[0].clone()));
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn create_player(
    name: String,
    email: String,
    user_id: String,
    meeple_color: i32,
) -> Result<player::Player, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    let new_player = NewPlayer {
        name,
        email,
        user_id,
        meeple_color,
        rating: None,
    };
    match diesel::insert_into(schema::player::table)
        .values(&new_player)
        .get_result(conn)
    {
        Ok(p) => Ok(to_player(p)),
        Err(e) => Err(internal_server_error(e.to_string())),
    }
}

pub fn update_player(
    pid: i32,
    nam: String,
    m_color: i32,
    rat: Option<i32>,
) -> Result<player::Player, Error> {
    use self::schema::player::dsl::{meeple_color, name, player, rating};
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    match diesel::update(player.find(pid))
        .set((name.eq(nam), meeple_color.eq(m_color), rating.eq(rat)))
        .get_result(conn)
    {
        Ok(p) => {
            return Ok(to_player(p));
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn get_games(
    player_id: Option<i32>,
    input_is_rated: Option<bool>,
    input_limit: Option<i32>,
) -> Result<Vec<game::Game>, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::game::dsl::{created_at, game as g, is_rated, player0_id, player1_id};
    let is_rtd = match input_is_rated {
        Some(i) => i,
        None => false,
    };
    let lmt = match input_limit {
        Some(l) => l,
        None => 100,
    };

    match player_id {
        Some(pid) => {
            match g
                .filter(player0_id.eq(pid).or(player1_id.eq(pid)))
                .filter(is_rated.eq(is_rtd))
                .order(created_at.desc())
                .limit(lmt as i64)
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
        None => match g
            .filter(is_rated.eq(is_rtd))
            .order(created_at.desc())
            .limit(lmt as i64)
            .load::<game::Game>(conn)
        {
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

pub fn get_waiting_games() -> Result<Vec<game::WaitingGame>, Error> {
    let conn = &mut establish_connection();
    use self::schema::waiting_game::dsl::*;
    match waiting_game.load::<game::WaitingGame>(conn) {
        Ok(gs) => Ok(gs),
        Err(e) => return Err(internal_server_error(e.to_string())),
    }
}

pub fn create_waiting_game(player_id: i32) -> Result<game::WaitingGame, Error> {
    let new_waiting_game = NewWaitingGame { player_id };
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    match diesel::insert_into(schema::waiting_game::table)
        .values(&new_waiting_game)
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

pub fn update_waiting_game(wid: i32, gid: i32) -> Result<game::WaitingGame, Error> {
    use self::schema::waiting_game::dsl::*;
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    match diesel::update(waiting_game.find(wid))
        .set(game_id.eq(gid))
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

pub fn delete_waiting_game(pid: i32) -> Result<(), Error> {
    use self::schema::waiting_game::dsl::*;
    let conn = &mut establish_connection();
    match diesel::delete(waiting_game.filter(player_id.eq(pid))).execute(conn) {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn create_game(
    player0_id: i32,
    player1_id: i32,
    next_tile_id: Option<i32>,
    next_player_id: Option<i32>,
    current_tile_id: Option<i32>,
    current_player_id: Option<i32>,
    player0_name: String,
    player1_name: String,
    player0_color: i32,
    player1_color: i32,
    is_rated: bool,
    first_player_id: Option<i32>,
) -> Result<game::Game, Error> {
    let new_game = NewGame {
        player0_id,
        player1_id,
        player0_point: 0,
        player1_point: 0,
        next_tile_id,
        next_player_id,
        current_tile_id,
        current_player_id,
        player0_name,
        player1_name,
        player0_color,
        player1_color,
        is_rated,
        before_player0_rating: None,
        before_player1_rating: None,
        after_player0_rating: None,
        after_player1_rating: None,
        first_player_id,
        winner_player_id: None,
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
    cur_tid: i32,
    cur_pid: i32,
    b_rating0: Option<i32>,
    b_rating1: Option<i32>,
    a_rating0: Option<i32>,
    a_rating1: Option<i32>,
    first_pid: Option<i32>,
    winner_pid: Option<i32>,
) -> Result<game::Game, Error> {
    use self::schema::game::dsl::{
        after_player0_rating, after_player1_rating, before_player0_rating, before_player1_rating,
        current_player_id, current_tile_id, first_player_id, game, next_player_id, next_tile_id,
        player0_point, player1_point, winner_player_id,
    };
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    match diesel::update(game.find(gmid))
        .set((
            player0_point.eq(p0_point),
            player1_point.eq(p1_point),
            next_tile_id.eq(next_tid),
            next_player_id.eq(next_pid),
            current_tile_id.eq(cur_tid),
            current_player_id.eq(cur_pid),
            before_player0_rating.eq(b_rating0),
            before_player1_rating.eq(b_rating1),
            after_player0_rating.eq(a_rating0),
            after_player1_rating.eq(a_rating1),
            first_player_id.eq(first_pid),
            winner_player_id.eq(winner_pid),
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

pub fn get_tile_move(mid: i32) -> Result<mov::TileMove, Error> {
    match get_move(mid)? {
        mov::Move::TMove(tm) => Ok(tm),
        _ => return Err(not_found_error("tile_move".to_string())),
    }
}

pub fn get_meeple_move(mid: i32) -> Result<mov::MeepleMove, Error> {
    match get_move(mid)? {
        mov::Move::MMove(mm) => Ok(mm),
        _ => return Err(not_found_error("meeple_move".to_string())),
    }
}

pub fn get_move(mid: i32) -> Result<mov::Move, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::move_::dsl::{id, move_ as m};

    match m.filter(id.eq(mid)).limit(1).load::<QueryMove>(conn) {
        Ok(ms) => {
            if ms.len() == 0 {
                return Err(not_found_error("move".to_string()));
            }
            Ok(to_move(ms[0].clone()))
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
        .filter(ord.ne(-1))
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
        mov::Move::DMove(m) => InsertMove {
            ord: m.ord,
            game_id: m.game_id,
            player_id: m.player_id,
            tile_id: m.tile.to_id(),
            meeple_id: -1,
            rot: -1,
            tile_pos_y: -1,
            tile_pos_x: -1,
            meeple_pos: -1,
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

pub fn create_optimal_move(
    game_id: i32,
    last_n: i32,
    tile_move_id: i32,
    meeple_move_id: i32,
) -> Result<optimal_move::OptimalMove, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    let nom = NewOptimalMove {
        game_id,
        last_n,
        tile_move_id,
        meeple_move_id,
    };
    match diesel::insert_into(schema::optimal_move::table)
        .values(&nom)
        .get_result(conn)
    {
        Ok(r) => Ok(r),
        Err(e) => Err(internal_server_error(e.to_string())),
    }
}

pub fn create_problem(new_problem: &NewProblem) -> Result<problem::Problem, Error> {
    let conn = &mut establish_connection();

    match diesel::insert_into(schema::problem::table)
        .values(new_problem)
        .get_result(conn)
    {
        Ok(prb) => Ok(prb),
        Err(e) => Err(internal_server_error(e.to_string())),
    }
}

pub fn get_problem(prid: i32) -> Result<problem::Problem, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::problem::dsl::{id, problem as p};

    match p
        .filter(id.eq(prid))
        .limit(1)
        .load::<problem::Problem>(conn)
    {
        Ok(ps) => {
            if ps.len() == 0 {
                return Err(not_found_error("problem".to_string()));
            }
            return Ok(ps[0].clone());
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}
pub fn get_problems() -> Result<Vec<problem::Problem>, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::problem::dsl::{created_at, problem as p, start_at};
    let now = chrono::Utc::now().naive_utc();

    match p
        .filter(start_at.le(now))
        .order(created_at.desc())
        .limit(300)
        .load::<problem::Problem>(conn)
    {
        Ok(ps) => {
            return Ok(ps);
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn create_vote(nv: &NewVote) -> Result<problem::Vote, Error> {
    let conn = &mut establish_connection();

    match diesel::insert_into(schema::vote::table)
        .values(nv)
        .get_result(conn)
    {
        Ok(v) => to_vote(v),
        Err(e) => Err(internal_server_error(e.to_string())),
    }
}

pub fn get_vote(vid: i32) -> Result<problem::Vote, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::vote::dsl::{id, vote as v};

    match v.filter(id.eq(vid)).limit(1).load::<QueryVote>(conn) {
        Ok(vs) => {
            if vs.len() == 0 {
                return Err(not_found_error("vote".to_string()));
            }
            return to_vote(vs[0].clone());
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn get_votes(prid: Option<i32>) -> Result<Vec<problem::Vote>, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::vote::dsl::{created_at, problem_id, vote as v};
    let mut query = v.order(created_at.desc()).into_boxed();
    if let Some(pr) = prid {
        query = query.filter(problem_id.eq(pr))
    }
    query = query.limit(100);

    match query.load::<QueryVote>(conn) {
        Ok(vts) => {
            return Ok(vts.into_iter().map(|vt| to_vote(vt).unwrap()).collect());
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

pub fn create_favorite(nf: &NewFavorite) -> Result<problem::Favorite, Error> {
    let conn = &mut establish_connection();

    match diesel::insert_into(schema::favorite::table)
        .values(nf)
        .get_result(conn)
    {
        Ok(f) => Ok(f),
        Err(e) => Err(internal_server_error(e.to_string())),
    }
}

pub fn get_favorites(vid: Option<i32>, pid: Option<i32>) -> Result<Vec<problem::Favorite>, Error> {
    let conn = &mut establish_connection(); // FIXME: establish connection once, not every time
    use self::schema::favorite::dsl::{created_at, favorite as f, player_id, vote_id};

    let mut query = f.order(created_at.desc()).into_boxed();
    if let Some(v) = vid {
        query = query.filter(vote_id.eq(v))
    }
    if let Some(p) = pid {
        query = query.filter(player_id.eq(p))
    }
    query = query.limit(100);

    match query.load::<problem::Favorite>(conn) {
        Ok(fs) => {
            return Ok(fs);
        }
        Err(e) => {
            return Err(internal_server_error(e.to_string()));
        }
    }
}

fn to_move(qm: QueryMove) -> mov::Move {
    if qm.tile_id != -1 && qm.rot == -1 {
        return mov::Move::DMove(mov::DiscardMove {
            id: qm.id,
            ord: qm.ord,
            game_id: qm.game_id,
            player_id: qm.player_id,
            tile: tile::to_tile(qm.tile_id),
        });
    }
    match (qm.tile_id, qm.meeple_id) {
        (-1, _) => mov::Move::MMove(mov::MeepleMove {
            id: qm.id,
            ord: qm.ord,
            game_id: qm.game_id,
            player_id: qm.player_id,
            meeple_id: qm.meeple_id,
            meeple_pos: qm.meeple_pos,
            tile_pos: (qm.tile_pos_y, qm.tile_pos_x),
        }),
        (_, -1) => mov::Move::TMove(mov::TileMove {
            id: qm.id,
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

fn to_player(v: QueryPlayer) -> player::Player {
    player::Player {
        id: v.id,
        name: v.name,
        email: v.email,
        user_id: v.user_id,
        meeple_color: v.meeple_color,
        rating: v.rating,
        profile_image_url: "".to_string(),
    }
}

fn to_vote(v: QueryVote) -> Result<problem::Vote, Error> {
    Ok(problem::Vote {
        id: v.id,
        problem_id: v.problem_id,
        player_id: v.player_id,
        player_name: v.player_name,
        player_profile_image_url: "".to_string(),
        note: v.note,
        favorite_count: v.favorite_count,
        tile_move_id: v.tile_move_id,
        tile_move: get_tile_move(v.tile_move_id)?,
        meeple_move_id: v.meeple_move_id,
        meeple_move: get_meeple_move(v.meeple_move_id)?,
        created_at: v.created_at,
    })
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
