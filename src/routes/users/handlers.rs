use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::DbPool;
use super::models::{User, UserResponse};

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Database connection error"
        })),
    };

    match users.load::<UserResponse>(&mut conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to fetch users"
        })),
    }
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user: web::Json<User>
) -> impl Responder {
    use crate::schema::users::dsl::*;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Database connection error"
        })),
    };

    match diesel::insert_into(users)
        .values(&user.into_inner())
        .get_result::<UserResponse>(&mut conn)
    {
        Ok(result) => HttpResponse::Created().json(result),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to create user"
        })),
    }
} 