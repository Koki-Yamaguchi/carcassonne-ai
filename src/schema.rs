// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Int4,
        note -> Text,
        player0_id -> Int4,
        player1_id -> Int4,
        player0_point -> Int4,
        player1_point -> Int4,
        created_at -> Timestamp,
        ended_at -> Nullable<Timestamp>,
        next_tile_id -> Nullable<Int4>,
    }
}

diesel::table! {
    #[sql_name = "move"]
    move_ (id) {
        id -> Int4,
        ord -> Int4,
        game_id -> Int4,
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
    player (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(move_ -> game (game_id));
diesel::joinable!(move_ -> player (player_id));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    move_,
    player,
);
