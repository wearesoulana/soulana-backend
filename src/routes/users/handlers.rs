use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;

use super::models::{User, UserResponse};

#[get("/users")]
pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!(
        "SELECT id, name, email FROM users"
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => {
            let users: Vec<UserResponse> = rows
                .into_iter()
                .map(|row| UserResponse {
                    id: row.id,
                    name: row.name,
                    email: row.email,
                })
                .collect();
            HttpResponse::Ok().json(users)
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch users"
            }))
        }
    }
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<User>
) -> impl Responder {
    match sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        user.name,
        user.email
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(row) => {
            let created_user = UserResponse {
                id: row.id,
                name: row.name,
                email: row.email,
            };
            HttpResponse::Created().json(created_user)
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create user"
            }))
        }
    }
} 