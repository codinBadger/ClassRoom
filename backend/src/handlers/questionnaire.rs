use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::{Questionnaire, Question, CreateQuestionnaire, CreateQuestion, SubmitAnswer, QuestionnaireResult};

pub async fn create_questionnaire(
    pool: web::Data<SqlitePool>,
    course_id: web::Path<String>,
    questionnaire_data: web::Json<CreateQuestionnaire>,
) -> impl Responder {
    let questionnaire_id = Uuid::new_v4().to_string();

    let result = sqlx::query(
        "INSERT INTO questionnaires (id, course_id, title, description, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&questionnaire_id)
    .bind(course_id.into_inner())
    .bind(&questionnaire_data.title)
    .bind(&questionnaire_data.description)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let questionnaire = sqlx::query_as::<_, Questionnaire>(
                "SELECT * FROM questionnaires WHERE id = ?"
            )
            .bind(&questionnaire_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            HttpResponse::Created().json(questionnaire)
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Failed to create questionnaire: {}", e)
        }))
    }
}

pub async fn add_question(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, String)>,
    question_data: web::Json<CreateQuestion>,
) -> impl Responder {
    let (_, questionnaire_id) = path.into_inner();
    let question_id = Uuid::new_v4().to_string();

    let options_json = question_data.options.as_ref()
        .map(|opts| serde_json::to_string(opts).unwrap());

    let result = sqlx::query(
        "INSERT INTO questions (id, questionnaire_id, question_text, question_type, options, correct_answer, points, created_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&question_id)
    .bind(&questionnaire_id)
    .bind(&question_data.question_text)
    .bind(&question_data.question_type)
    .bind(&options_json)
    .bind(&question_data.correct_answer)
    .bind(question_data.points)
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let question = sqlx::query_as::<_, Question>(
                "SELECT * FROM questions WHERE id = ?"
            )
            .bind(&question_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();

            HttpResponse::Created().json(question)
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": format!("Failed to add question: {}", e)
        }))
    }
}

pub async fn get_questionnaires(
    pool: web::Data<SqlitePool>,
    course_id: web::Path<String>,
) -> impl Responder {
    let questionnaires = sqlx::query_as::<_, Questionnaire>(
        "SELECT * FROM questionnaires WHERE course_id = ? ORDER BY created_at DESC"
    )
    .bind(course_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match questionnaires {
        Ok(questionnaires) => HttpResponse::Ok().json(questionnaires),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch questionnaires: {}", e)
        }))
    }
}

pub async fn get_questions(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (_, questionnaire_id) = path.into_inner();

    let questions = sqlx::query_as::<_, Question>(
        "SELECT * FROM questions WHERE questionnaire_id = ? ORDER BY created_at ASC"
    )
    .bind(&questionnaire_id)
    .fetch_all(pool.get_ref())
    .await;

    match questions {
        Ok(questions) => HttpResponse::Ok().json(questions),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch questions: {}", e)
        }))
    }
}

pub async fn submit_questionnaire(
    pool: web::Data<SqlitePool>,
    path: web::Path<(String, String)>,
    answers: web::Json<Vec<SubmitAnswer>>,
) -> impl Responder {
    let (_, questionnaire_id) = path.into_inner();

    let questions = sqlx::query_as::<_, Question>(
        "SELECT * FROM questions WHERE questionnaire_id = ?"
    )
    .bind(&questionnaire_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let mut total_points = 0;
    let mut earned_points = 0;

    for question in &questions {
        total_points += question.points;
        
        if let Some(answer) = answers.iter().find(|a| a.question_id == question.id) {
            if answer.answer.trim().to_lowercase() == question.correct_answer.trim().to_lowercase() {
                earned_points += question.points;
            }
        }
    }

    let percentage = if total_points > 0 {
        (earned_points as f64 / total_points as f64) * 100.0
    } else {
        0.0
    };

    let result = QuestionnaireResult {
        total_points,
        earned_points,
        percentage,
    };

    HttpResponse::Ok().json(result)
}
