// @generated automatically by Diesel CLI.

diesel::table! {
    color (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    favorite (id) {
        id -> Int4,
        vote_id -> Int4,
        player_id -> Int4,
        player_name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    game (id) {
        id -> Int4,
        player0_id -> Int4,
        player1_id -> Int4,
        player0_point -> Int4,
        player1_point -> Int4,
        next_tile_id -> Nullable<Int4>,
        next_player_id -> Nullable<Int4>,
        created_at -> Timestamp,
        ended_at -> Nullable<Timestamp>,
        current_player_id -> Nullable<Int4>,
        current_tile_id -> Nullable<Int4>,
        player0_name -> Text,
        player1_name -> Text,
        player0_color -> Int4,
        player1_color -> Int4,
        is_rated -> Bool,
        before_player0_rating -> Nullable<Int4>,
        before_player1_rating -> Nullable<Int4>,
        after_player0_rating -> Nullable<Int4>,
        after_player1_rating -> Nullable<Int4>,
        first_player_id -> Nullable<Int4>,
        winner_player_id -> Nullable<Int4>,
    }
}

diesel::table! {
    #[sql_name = "move"]
    move_ (id) {
        id -> Int4,
        ord -> Int4,
        game_id -> Nullable<Int4>,
        player_id -> Int4,
        tile_id -> Int4,
        meeple_id -> Int4,
        rot -> Int4,
        tile_pos_y -> Int4,
        tile_pos_x -> Int4,
        meeple_pos -> Int4,
    }
}

diesel::table! {
    optimal_move (id) {
        id -> Int4,
        game_id -> Int4,
        last_n -> Int4,
        tile_move_id -> Int4,
        meeple_move_id -> Int4,
    }
}

diesel::table! {
    player (id) {
        id -> Int4,
        name -> Varchar,
        email -> Text,
        user_id -> Text,
        meeple_color -> Int4,
        rating -> Nullable<Int4>,
        profile_image_url -> Varchar,
    }
}

diesel::table! {
    problem (id) {
        id -> Int4,
        game_id -> Int4,
        created_at -> Timestamp,
        name -> Varchar,
        start_at -> Nullable<Timestamp>,
        creator_id -> Nullable<Int4>,
        creator_name -> Nullable<Varchar>,
        vote_count -> Int4,
    }
}

diesel::table! {
    vote (id) {
        id -> Int4,
        problem_id -> Int4,
        player_id -> Int4,
        player_name -> Varchar,
        note -> Text,
        favorite_count -> Int4,
        tile_move_id -> Int4,
        meeple_move_id -> Int4,
        created_at -> Timestamp,
        player_profile_image_url -> Varchar,
    }
}

diesel::table! {
    waiting_game (id) {
        id -> Int4,
        player_id -> Int4,
        game_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::joinable!(favorite -> vote (vote_id));
diesel::joinable!(player -> color (meeple_color));
diesel::joinable!(vote -> problem (problem_id));

diesel::allow_tables_to_appear_in_same_query!(
    color,
    favorite,
    game,
    move_,
    optimal_move,
    player,
    problem,
    vote,
    waiting_game,
);
