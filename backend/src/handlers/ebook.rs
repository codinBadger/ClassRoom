use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::Ebook;

// Simplified ebook handler - multipart upload would require additional complexity
pub async fn upload_ebook(
    _pool: web::Data<SqlitePool>,
    _course_id: web::Path<String>,
) -> impl Responder {
    // This is a placeholder - in production, you'd handle multipart file upload
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Upload endpoint - requires multipart form data handling"
    }))
}

pub async fn get_ebooks(
    pool: web::Data<SqlitePool>,
    course_id: web::Path<String>,
) -> impl Responder {
    let ebooks = sqlx::query_as::<_, Ebook>(
        "SELECT * FROM ebooks WHERE course_id = ? ORDER BY created_at DESC"
    )
    .bind(course_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match ebooks {
        Ok(ebooks) => HttpResponse::Ok().json(ebooks),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch ebooks: {}", e)
        }))
    }
}

pub async fn delete_ebook(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (course_id, ebook_id) = path.into_inner();

    let ebook = sqlx::query_as::<_, Ebook>(
        "SELECT * FROM ebooks WHERE id = ? AND course_id = ?"
    )
    .bind(&ebook_id)
    .bind(&course_id)
    .fetch_one(pool.get_ref())
    .await;

    if let Ok(ebook) = ebook {
        std::fs::remove_file(&ebook.file_path).ok();
        
        sqlx::query("DELETE FROM ebooks WHERE id = ?")
            .bind(&ebook_id)
            .execute(pool.get_ref())
            .await
            .ok();

        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": "Ebook not found"
        }))
    }
}
