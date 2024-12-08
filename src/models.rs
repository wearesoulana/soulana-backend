use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

// Ana kimlik modeli
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::identities)]
pub struct Identity {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Email kimlik modeli
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::email_identities)]
pub struct EmailIdentity {
    pub id: Uuid,
    pub identity_id: Option<Uuid>,
    pub email: String,
    pub email_verified: Option<bool>,
    pub password_hash: Option<String>,
    pub created_at: NaiveDateTime,
}

// Wallet kimlik modeli
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::wallet_identities)]
pub struct WalletIdentity {
    pub id: Uuid,
    pub identity_id: Option<Uuid>,
    pub wallet_address: String,
    pub nonce: String,
    pub created_at: NaiveDateTime,
}

// Yeni kayıt için kullanılacak modeller
#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::identities)]
pub struct NewIdentity {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::email_identities)]
pub struct NewEmailIdentity<'a> {
    pub identity_id: Option<Uuid>,
    pub email: &'a str,
    pub password_hash: &'a str,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::wallet_identities)]
pub struct NewWalletIdentity<'a> {
    pub identity_id: Option<Uuid>,
    pub wallet_address: &'a str,
    pub nonce: &'a str,
}

// API yanıtları için kullanılacak modeller
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Option<String>,
    pub wallet_address: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthTokenResponse {
    pub token: String,
    pub user: UserResponse,
} 