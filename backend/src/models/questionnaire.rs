use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
    FillInBlank,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Questionnaire {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Question {
    pub id: String,
    pub questionnaire_id: String,
    pub question_text: String,
    pub question_type: String,
    pub options: Option<String>, // JSON string of options
    pub correct_answer: String,
    pub points: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestionnaire {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestion {
    pub question_text: String,
    pub question_type: String,
    pub options: Option<Vec<String>>,
    pub correct_answer: String,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct SubmitAnswer {
    pub question_id: String,
    pub answer: String,
}

#[derive(Debug, Serialize)]
pub struct QuestionnaireResult {
    pub total_points: i32,
    pub earned_points: i32,
    pub percentage: f64,
}
