use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{Classroom, CreateClassroom, UpdateClassroom};

pub async fn create_classroom(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    classroom_data: web::Json<CreateClassroom>,
) -> impl Responder {
    let classroom_id = Uuid::new_v4().to_string();
    let user_id = user_id.into_inner();

    let result = sqlx::query(
        "INSERT INTO classrooms (id, user_id, name, description, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&classroom_id)
    .bind(&user_id)
    .bind(&classroom_data.name)
    .bind(&classroom_data.description)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let classroom = sqlx::query_as::<_, Classroom>(
                "SELECT * FROM classrooms WHERE id = ?"
            )
            .bind(&classroom_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            HttpResponse::Created().json(classroom)
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Failed to create classroom: {}", e)
        }))
    }
}

pub async fn get_classrooms(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
) -> impl Responder {
    let classrooms = sqlx::query_as::<_, Classroom>(
        "SELECT * FROM classrooms WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(user_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match classrooms {
        Ok(classrooms) => HttpResponse::Ok().json(classrooms),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch classrooms: {}", e)
        }))
    }
}

pub async fn get_classroom(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    classroom_id: web::Path<String>,
) -> impl Responder {
    let classroom = sqlx::query_as::<_, Classroom>(
        "SELECT * FROM classrooms WHERE id = ? AND user_id = ?"
    )
    .bind(classroom_id.into_inner())
    .bind(user_id.into_inner())
    .fetch_one(pool.get_ref())
    .await;

    match classroom {
        Ok(classroom) => HttpResponse::Ok().json(classroom),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Classroom not found"
        }))
    }
}

pub async fn update_classroom(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    classroom_id: web::Path<String>,
    update_data: web::Json<UpdateClassroom>,
) -> impl Responder {
    let classroom_id = classroom_id.into_inner();
    let user_id = user_id.into_inner();

    // Verify ownership
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM classrooms WHERE id = ? AND user_id = ?"
    )
    .bind(&classroom_id)
    .bind(&user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    if exists == 0 {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Classroom not found"
        }));
    }

    if let Some(name) = &update_data.name {
        sqlx::query("UPDATE classrooms SET name = ?, updated_at = ? WHERE id = ?")
            .bind(name)
            .bind(Utc::now())
            .bind(&classroom_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }

    if let Some(description) = &update_data.description {
        sqlx::query("UPDATE classrooms SET description = ?, updated_at = ? WHERE id = ?")
            .bind(description)
            .bind(Utc::now())
            .bind(&classroom_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }

    let classroom = sqlx::query_as::<_, Classroom>(
        "SELECT * FROM classrooms WHERE id = ?"
    )
    .bind(&classroom_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(classroom)
}

pub async fn delete_classroom(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    classroom_id: web::Path<String>,
) -> impl Responder {
    let result = sqlx::query(
        "DELETE FROM classrooms WHERE id = ? AND user_id = ?"
    )
    .bind(classroom_id.into_inner())
    .bind(user_id.into_inner())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Classroom not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to delete classroom: {}", e)
        }))
    }
}
