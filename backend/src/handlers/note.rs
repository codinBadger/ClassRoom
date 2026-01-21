use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{Note, CreateNote, UpdateNote};

pub async fn create_note(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    course_id: web::Path<String>,
    note_data: web::Json<CreateNote>,
) -> impl Responder {
    let note_id = Uuid::new_v4().to_string();

    let result = sqlx::query(
        "INSERT INTO notes (id, course_id, user_id, title, content, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&note_id)
    .bind(course_id.into_inner())
    .bind(user_id.into_inner())
    .bind(&note_data.title)
    .bind(&note_data.content)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let note = sqlx::query_as::<_, Note>(
                "SELECT * FROM notes WHERE id = ?"
            )
            .bind(&note_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            HttpResponse::Created().json(note)
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Failed to create note: {}", e)
        }))
    }
}

pub async fn get_notes(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    course_id: web::Path<String>,
) -> impl Responder {
    let notes = sqlx::query_as::<_, Note>(
        "SELECT * FROM notes WHERE course_id = ? AND user_id = ? ORDER BY updated_at DESC"
    )
    .bind(course_id.into_inner())
    .bind(user_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match notes {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch notes: {}", e)
        }))
    }
}

pub async fn update_note(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    path: web::Path<(String, String)>,
    update_data: web::Json<UpdateNote>,
) -> impl Responder {
    let (course_id, note_id) = path.into_inner();
    let user_id = user_id.into_inner();

    if let Some(title) = &update_data.title {
        sqlx::query("UPDATE notes SET title = ?, updated_at = ? WHERE id = ? AND user_id = ?")
            .bind(title)
            .bind(Utc::now())
            .bind(&note_id)
            .bind(&user_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }

    if let Some(content) = &update_data.content {
        sqlx::query("UPDATE notes SET content = ?, updated_at = ? WHERE id = ? AND user_id = ?")
            .bind(content)
            .bind(Utc::now())
            .bind(&note_id)
            .bind(&user_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }

    let note = sqlx::query_as::<_, Note>(
        "SELECT * FROM notes WHERE id = ? AND course_id = ? AND user_id = ?"
    )
    .bind(&note_id)
    .bind(&course_id)
    .bind(&user_id)
    .fetch_one(pool.get_ref())
    .await;

    match note {
        Ok(note) => HttpResponse::Ok().json(note),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Note not found"
        }))
    }
}

pub async fn delete_note(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (course_id, note_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM notes WHERE id = ? AND course_id = ? AND user_id = ?"
    )
    .bind(&note_id)
    .bind(&course_id)
    .bind(user_id.into_inner())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Note not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to delete note: {}", e)
        }))
    }
}
