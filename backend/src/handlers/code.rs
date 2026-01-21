use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{CodeSession, ExecuteCode, CreateCodeSession};
use crate::utils::execute_code;

pub async fn execute(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    course_id: web::Path<String>,
    code_data: web::Json<ExecuteCode>,
) -> impl Responder {
    let start = std::time::Instant::now();
    let result = execute_code(&code_data.language, &code_data.code).await;
    let duration_seconds = start.elapsed().as_secs() as i32;

    // Save session
    let session_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO code_sessions (id, user_id, course_id, language, code, output, duration_seconds, created_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&session_id)
    .bind(user_id.into_inner())
    .bind(course_id.into_inner())
    .bind(&code_data.language)
    .bind(&code_data.code)
    .bind(if result.success { Some(&result.output) } else { result.error.as_ref() })
    .bind(duration_seconds)
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await
    .ok();

    HttpResponse::Ok().json(result)
}

pub async fn create_timed_session(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    course_id: web::Path<String>,
    session_data: web::Json<CreateCodeSession>,
) -> impl Responder {
    let session_id = Uuid::new_v4().to_string();

    let result = sqlx::query(
        "INSERT INTO code_sessions (id, user_id, course_id, language, code, time_limit_seconds, created_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&session_id)
    .bind(user_id.into_inner())
    .bind(course_id.into_inner())
    .bind(&session_data.language)
    .bind(&session_data.code)
    .bind(session_data.time_limit_seconds)
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let session = sqlx::query_as::<_, CodeSession>(
                "SELECT * FROM code_sessions WHERE id = ?"
            )
            .bind(&session_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            HttpResponse::Created().json(session)
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Failed to create session: {}", e)
        }))
    }
}

pub async fn get_sessions(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    course_id: web::Path<String>,
) -> impl Responder {
    let sessions = sqlx::query_as::<_, CodeSession>(
        "SELECT * FROM code_sessions WHERE user_id = ? AND course_id = ? ORDER BY created_at DESC LIMIT 50"
    )
    .bind(user_id.into_inner())
    .bind(course_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match sessions {
        Ok(sessions) => HttpResponse::Ok().json(sessions),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch sessions: {}", e)
        }))
    }
}
