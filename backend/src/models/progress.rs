use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Progress {
    pub id: String,
    pub user_id: String,
    pub course_id: String,
    pub activity_type: String, // note, flashcard, questionnaire, code_session
    pub activity_id: String,
    pub completed: bool,
    pub time_spent_seconds: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProgress {
    pub activity_type: String,
    pub activity_id: String,
    pub completed: bool,
    pub time_spent_seconds: i32,
}

#[derive(Debug, Serialize)]
pub struct ProgressStats {
    pub total_time_spent: i32,
    pub completed_activities: i32,
    pub total_activities: i32,
    pub progress_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct DisciplineMetrics {
    pub days_active: i32,
    pub current_streak: i32,
    pub longest_streak: i32,
    pub avg_daily_time: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TimelineEntry {
    pub id: String,
    pub activity_type: String,
    pub activity_id: String,
    pub created_at: DateTime<Utc>,
}
