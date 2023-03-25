// @generated automatically by Diesel CLI.

diesel::table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    users (username) {
        username -> Bpchar,
        team -> Nullable<Int4>,
    }
}

diesel::joinable!(users -> teams (team));

diesel::allow_tables_to_appear_in_same_query!(
    teams,
    users,
);
