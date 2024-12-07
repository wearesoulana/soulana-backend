// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        image -> Varchar,
        target -> Varchar,
        raised -> Varchar,
        min_donation -> Float8,
        wallet -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    users,
);
