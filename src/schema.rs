// @generated automatically by Diesel CLI.

diesel::table! {
    email_identities (id) {
        id -> Uuid,
        identity_id -> Nullable<Uuid>,
        email -> Varchar,
        email_verified -> Nullable<Bool>,
        password_hash -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    identities (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

diesel::table! {
    wallet_identities (id) {
        id -> Uuid,
        identity_id -> Nullable<Uuid>,
        wallet_address -> Varchar,
        nonce -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(email_identities -> identities (identity_id));
diesel::joinable!(wallet_identities -> identities (identity_id));

diesel::allow_tables_to_appear_in_same_query!(
    email_identities,
    identities,
    projects,
    users,
    wallet_identities,
);
