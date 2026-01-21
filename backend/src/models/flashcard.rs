use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Flashcard {
    pub id: String,
    pub course_id: String,
    pub front: String,
    pub back: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFlashcard {
    pub front: String,
    pub back: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFlashcard {
    pub front: Option<String>,
    pub back: Option<String>,
}
