use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{CreateUser, LoginUser, User, UserResponse};
use crate::utils::create_jwt;

pub async fn register(
    pool: web::Data<SqlitePool>,
    user_data: web::Json<CreateUser>,
) -> impl Responder {
    let user_id = Uuid::new_v4().to_string();
    let password_hash = bcrypt::hash(&user_data.password, bcrypt::DEFAULT_COST)
        .unwrap_or_default();

    let result = sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind(&user_data.username)
    .bind(&user_data.email)
    .bind(&password_hash)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let token = create_jwt(&user_id).unwrap_or_default();
            HttpResponse::Created().json(serde_json::json!({
                "token": token,
                "user_id": user_id,
                "username": user_data.username,
                "email": user_data.email
            }))
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Failed to create user: {}", e)
        }))
    }
}

pub async fn login(
    pool: web::Data<SqlitePool>,
    credentials: web::Json<LoginUser>,
) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = ?"
    )
    .bind(&credentials.email)
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(user) => {
            if bcrypt::verify(&credentials.password, &user.password_hash).unwrap_or(false) {
                let token = create_jwt(&user.id).unwrap_or_default();
                HttpResponse::Ok().json(serde_json::json!({
                    "token": token,
                    "user": UserResponse::from(user)
                }))
            } else {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid credentials"
                }))
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        }))
    }
}

pub async fn get_profile(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = ?"
    )
    .bind(user_id.into_inner())
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        }))
    }
}
