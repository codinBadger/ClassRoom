use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{Course, CreateCourse, UpdateCourse};

pub async fn create_course(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    classroom_id: web::Path<String>,
    course_data: web::Json<CreateCourse>,
) -> impl Responder {
    let classroom_id = classroom_id.into_inner();
    let user_id = user_id.into_inner();

    // Verify classroom ownership
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

    let course_id = Uuid::new_v4().to_string();

    let result = sqlx::query(
        "INSERT INTO courses (id, classroom_id, name, description, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&course_id)
    .bind(&classroom_id)
    .bind(&course_data.name)
    .bind(&course_data.description)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let course = sqlx::query_as::<_, Course>(
                "SELECT * FROM courses WHERE id = ?"
            )
            .bind(&course_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            HttpResponse::Created().json(course)
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Failed to create course: {}", e)
        }))
    }
}

pub async fn get_courses(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    classroom_id: web::Path<String>,
) -> impl Responder {
    let classroom_id = classroom_id.into_inner();
    let user_id = user_id.into_inner();

    // Verify classroom ownership
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

    let courses = sqlx::query_as::<_, Course>(
        "SELECT * FROM courses WHERE classroom_id = ? ORDER BY created_at DESC"
    )
    .bind(&classroom_id)
    .fetch_all(pool.get_ref())
    .await;

    match courses {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch courses: {}", e)
        }))
    }
}

pub async fn get_course(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (classroom_id, course_id) = path.into_inner();

    let course = sqlx::query_as::<_, Course>(
        "SELECT c.* FROM courses c 
         INNER JOIN classrooms cl ON c.classroom_id = cl.id 
         WHERE c.id = ? AND c.classroom_id = ?"
    )
    .bind(&course_id)
    .bind(&classroom_id)
    .fetch_one(pool.get_ref())
    .await;

    match course {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Course not found"
        }))
    }
}

pub async fn update_course(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    path: web::Path<(String, String)>,
    update_data: web::Json<UpdateCourse>,
) -> impl Responder {
    let (classroom_id, course_id) = path.into_inner();
    let user_id = user_id.into_inner();

    // Verify ownership
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM courses c 
         INNER JOIN classrooms cl ON c.classroom_id = cl.id 
         WHERE c.id = ? AND cl.id = ? AND cl.user_id = ?"
    )
    .bind(&course_id)
    .bind(&classroom_id)
    .bind(&user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    if exists == 0 {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": "Course not found"
        }));
    }

    if let Some(name) = &update_data.name {
        sqlx::query("UPDATE courses SET name = ?, updated_at = ? WHERE id = ?")
            .bind(name)
            .bind(Utc::now())
            .bind(&course_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }

    if let Some(description) = &update_data.description {
        sqlx::query("UPDATE courses SET description = ?, updated_at = ? WHERE id = ?")
            .bind(description)
            .bind(Utc::now())
            .bind(&course_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }

    let course = sqlx::query_as::<_, Course>(
        "SELECT * FROM courses WHERE id = ?"
    )
    .bind(&course_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(course)
}

pub async fn delete_course(
    pool: web::Data<SqlitePool>,
    user_id: web::ReqData<String>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (classroom_id, course_id) = path.into_inner();
    let user_id = user_id.into_inner();

    let result = sqlx::query(
        "DELETE FROM courses WHERE id = ? AND classroom_id = ? AND classroom_id IN 
         (SELECT id FROM classrooms WHERE user_id = ?)"
    )
    .bind(&course_id)
    .bind(&classroom_id)
    .bind(&user_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Course not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to delete course: {}", e)
        }))
    }
}
