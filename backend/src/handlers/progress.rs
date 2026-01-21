use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{CreateProgress, ProgressStats, DisciplineMetrics, TimelineEntry};

pub async fn track_progress(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    course_id: web::Path<String>,
    progress_data: web::Json<CreateProgress>,
) -> impl Responder {
    let progress_id = Uuid::new_v4().to_string();

    let result = sqlx::query(
        "INSERT INTO progress (id, user_id, course_id, activity_type, activity_id, completed, time_spent_seconds, created_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&progress_id)
    .bind(user_id.into_inner())
    .bind(course_id.into_inner())
    .bind(&progress_data.activity_type)
    .bind(&progress_data.activity_id)
    .bind(progress_data.completed)
    .bind(progress_data.time_spent_seconds)
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "message": "Progress tracked successfully"
        })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Failed to track progress: {}", e)
        }))
    }
}

pub async fn get_progress_stats(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    course_id: web::Path<String>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    let course_id = course_id.into_inner();

    let total_time: i32 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(time_spent_seconds), 0) FROM progress WHERE user_id = ? AND course_id = ?"
    )
    .bind(&user_id)
    .bind(&course_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    let completed_count: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM progress WHERE user_id = ? AND course_id = ? AND completed = 1"
    )
    .bind(&user_id)
    .bind(&course_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    let total_count: i32 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT activity_id) FROM progress WHERE user_id = ? AND course_id = ?"
    )
    .bind(&user_id)
    .bind(&course_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    let progress_percentage = if total_count > 0 {
        (completed_count as f64 / total_count as f64) * 100.0
    } else {
        0.0
    };

    let stats = ProgressStats {
        total_time_spent: total_time,
        completed_activities: completed_count,
        total_activities: total_count,
        progress_percentage,
    };

    HttpResponse::Ok().json(stats)
}

pub async fn get_timeline(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    course_id: web::Path<String>,
) -> impl Responder {
    let timeline = sqlx::query_as::<_, TimelineEntry>(
        "SELECT id, activity_type, activity_id, created_at FROM progress 
         WHERE user_id = ? AND course_id = ? 
         ORDER BY created_at DESC LIMIT 100"
    )
    .bind(user_id.into_inner())
    .bind(course_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match timeline {
        Ok(timeline) => HttpResponse::Ok().json(timeline),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch timeline: {}", e)
        }))
    }
}

pub async fn get_discipline_metrics(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    course_id: web::Path<String>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    let course_id = course_id.into_inner();

    // Count distinct days with activity
    let days_active: i32 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT DATE(created_at)) FROM progress WHERE user_id = ? AND course_id = ?"
    )
    .bind(&user_id)
    .bind(&course_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    // Calculate average daily time
    let total_time: i32 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(time_spent_seconds), 0) FROM progress WHERE user_id = ? AND course_id = ?"
    )
    .bind(&user_id)
    .bind(&course_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    let avg_daily_time = if days_active > 0 {
        total_time as f64 / days_active as f64
    } else {
        0.0
    };

    let metrics = DisciplineMetrics {
        days_active,
        current_streak: 0, // Simplified for now
        longest_streak: 0, // Simplified for now
        avg_daily_time,
    };

    HttpResponse::Ok().json(metrics)
}
