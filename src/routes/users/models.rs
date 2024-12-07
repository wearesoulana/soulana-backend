use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
} 