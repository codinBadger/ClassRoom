use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{Flashcard, CreateFlashcard, UpdateFlashcard};

pub async fn create_flashcard(
    pool: web::Data<SqlitePool>,
    course_id: web::Path<String>,
    flashcard_data: web::Json<CreateFlashcard>,
) -> impl Responder {
    let flashcard_id = Uuid::new_v4().to_string();

    let result = sqlx::query(
        "INSERT INTO flashcards (id, course_id, front, back, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&flashcard_id)
    .bind(course_id.into_inner())
    .bind(&flashcard_data.front)
    .bind(&flashcard_data.back)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let flashcard = sqlx::query_as::<_, Flashcard>(
                "SELECT * FROM flashcards WHERE id = ?"
            )
            .bind(&flashcard_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            HttpResponse::Created().json(flashcard)
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Failed to create flashcard: {}", e)
        }))
    }
}

pub async fn get_flashcards(
    pool: web::Data<SqlitePool>,
    course_id: web::Path<String>,
) -> impl Responder {
    let flashcards = sqlx::query_as::<_, Flashcard>(
        "SELECT * FROM flashcards WHERE course_id = ? ORDER BY created_at DESC"
    )
    .bind(course_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match flashcards {
        Ok(flashcards) => HttpResponse::Ok().json(flashcards),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch flashcards: {}", e)
        }))
    }
}

pub async fn update_flashcard(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, String)>,
    update_data: web::Json<UpdateFlashcard>,
) -> impl Responder {
    let (course_id, flashcard_id) = path.into_inner();

    if let Some(front) = &update_data.front {
        sqlx::query("UPDATE flashcards SET front = ?, updated_at = ? WHERE id = ?")
            .bind(front)
            .bind(Utc::now())
            .bind(&flashcard_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }

    if let Some(back) = &update_data.back {
        sqlx::query("UPDATE flashcards SET back = ?, updated_at = ? WHERE id = ?")
            .bind(back)
            .bind(Utc::now())
            .bind(&flashcard_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }

    let flashcard = sqlx::query_as::<_, Flashcard>(
        "SELECT * FROM flashcards WHERE id = ? AND course_id = ?"
    )
    .bind(&flashcard_id)
    .bind(&course_id)
    .fetch_one(pool.get_ref())
    .await;

    match flashcard {
        Ok(flashcard) => HttpResponse::Ok().json(flashcard),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Flashcard not found"
        }))
    }
}

pub async fn delete_flashcard(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (course_id, flashcard_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM flashcards WHERE id = ? AND course_id = ?"
    )
    .bind(&flashcard_id)
    .bind(&course_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Flashcard not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to delete flashcard: {}", e)
        }))
    }
}
