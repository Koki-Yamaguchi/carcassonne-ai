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
    }
}

diesel::table! {
    player (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    game,
    player,
);
