use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::StreamExt;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use std::io::Write;
use crate::models::{Ebook, CreateEbook};

pub async fn upload_ebook(
    pool: web::Data<SqlitePool>,
    course_id: web::Path<String>,
    mut payload: Multipart,
) -> impl Responder {
    let course_id = course_id.into_inner();
    let ebook_id = Uuid::new_v4().to_string();
    let mut title = String::new();
    let mut author = None;
    let mut file_path = String::new();
    let mut file_size: i64 = 0;
    let mut mime_type = String::from("application/pdf");

    // Create uploads directory
    std::fs::create_dir_all("uploads/ebooks").ok();

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition();

        if let Some(name) = content_disposition.get_name() {
            match name {
                "title" => {
                    while let Some(chunk) = field.next().await {
                        title.push_str(&String::from_utf8_lossy(&chunk.unwrap()));
                    }
                }
                "author" => {
                    let mut author_str = String::new();
                    while let Some(chunk) = field.next().await {
                        author_str.push_str(&String::from_utf8_lossy(&chunk.unwrap()));
                    }
                    author = Some(author_str);
                }
                "file" => {
                    let filename = content_disposition
                        .get_filename()
                        .unwrap_or("upload.pdf");
                    
                    file_path = format!("uploads/ebooks/{}_{}", ebook_id, filename);
                    let mut file = std::fs::File::create(&file_path).unwrap();

                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        file_size += data.len() as i64;
                        file.write_all(&data).unwrap();
                    }

                    if filename.ends_with(".pdf") {
                        mime_type = String::from("application/pdf");
                    } else if filename.ends_with(".epub") {
                        mime_type = String::from("application/epub+zip");
                    }
                }
                _ => {}
            }
        }
    }

    let result = sqlx::query(
        "INSERT INTO ebooks (id, course_id, title, author, file_path, file_size, mime_type, created_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&ebook_id)
    .bind(&course_id)
    .bind(&title)
    .bind(&author)
    .bind(&file_path)
    .bind(file_size)
    .bind(&mime_type)
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let ebook = sqlx::query_as::<_, Ebook>(
                "SELECT * FROM ebooks WHERE id = ?"
            )
            .bind(&ebook_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            HttpResponse::Created().json(ebook)
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Failed to upload ebook: {}", e)
        }))
    }
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
