mod models;
mod handlers;
mod middleware;
mod utils;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Initialize database
    let pool = utils::init_db()
        .await
        .expect("Failed to initialize database");

    println!("🚀 Server starting on http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            // Public routes
            .route("/api/auth/register", web::post().to(handlers::register))
            .route("/api/auth/login", web::post().to(handlers::login))
            // Protected routes
            .service(
                web::scope("")
                    .wrap(middleware::AuthMiddleware)
                    // Profile
                    .route("/api/auth/profile", web::get().to(handlers::get_profile))
                    // Classrooms
                    .route("/api/classrooms", web::post().to(handlers::create_classroom))
                    .route("/api/classrooms", web::get().to(handlers::get_classrooms))
                    .route("/api/classrooms/{id}", web::get().to(handlers::get_classroom))
                    .route("/api/classrooms/{id}", web::put().to(handlers::update_classroom))
                    .route("/api/classrooms/{id}", web::delete().to(handlers::delete_classroom))
                    // Courses
                    .route("/api/classrooms/{classroom_id}/courses", web::post().to(handlers::create_course))
                    .route("/api/classrooms/{classroom_id}/courses", web::get().to(handlers::get_courses))
                    .route("/api/classrooms/{classroom_id}/courses/{course_id}", web::get().to(handlers::get_course))
                    .route("/api/classrooms/{classroom_id}/courses/{course_id}", web::put().to(handlers::update_course))
                    .route("/api/classrooms/{classroom_id}/courses/{course_id}", web::delete().to(handlers::delete_course))
                    // Ebooks
                    .route("/api/courses/{course_id}/ebooks", web::post().to(handlers::upload_ebook))
                    .route("/api/courses/{course_id}/ebooks", web::get().to(handlers::get_ebooks))
                    .route("/api/courses/{course_id}/ebooks/{ebook_id}", web::delete().to(handlers::delete_ebook))
                    // Flashcards
                    .route("/api/courses/{course_id}/flashcards", web::post().to(handlers::create_flashcard))
                    .route("/api/courses/{course_id}/flashcards", web::get().to(handlers::get_flashcards))
                    .route("/api/courses/{course_id}/flashcards/{flashcard_id}", web::put().to(handlers::update_flashcard))
                    .route("/api/courses/{course_id}/flashcards/{flashcard_id}", web::delete().to(handlers::delete_flashcard))
                    // Questionnaires
                    .route("/api/courses/{course_id}/questionnaires", web::post().to(handlers::create_questionnaire))
                    .route("/api/courses/{course_id}/questionnaires", web::get().to(handlers::get_questionnaires))
                    .route("/api/courses/{course_id}/questionnaires/{questionnaire_id}/questions", web::post().to(handlers::add_question))
                    .route("/api/courses/{course_id}/questionnaires/{questionnaire_id}/questions", web::get().to(handlers::get_questions))
                    .route("/api/courses/{course_id}/questionnaires/{questionnaire_id}/submit", web::post().to(handlers::submit_questionnaire))
                    // Notes
                    .route("/api/courses/{course_id}/notes", web::post().to(handlers::create_note))
                    .route("/api/courses/{course_id}/notes", web::get().to(handlers::get_notes))
                    .route("/api/courses/{course_id}/notes/{note_id}", web::put().to(handlers::update_note))
                    .route("/api/courses/{course_id}/notes/{note_id}", web::delete().to(handlers::delete_note))
                    // Code execution
                    .route("/api/courses/{course_id}/code/execute", web::post().to(handlers::execute))
                    .route("/api/courses/{course_id}/code/sessions", web::post().to(handlers::create_timed_session))
                    .route("/api/courses/{course_id}/code/sessions", web::get().to(handlers::get_sessions))
                    // Progress tracking
                    .route("/api/courses/{course_id}/progress", web::post().to(handlers::track_progress))
                    .route("/api/courses/{course_id}/progress/stats", web::get().to(handlers::get_progress_stats))
                    .route("/api/courses/{course_id}/progress/timeline", web::get().to(handlers::get_timeline))
                    .route("/api/courses/{course_id}/progress/metrics", web::get().to(handlers::get_discipline_metrics))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
