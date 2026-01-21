use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CodeSession {
    pub id: String,
    pub user_id: String,
    pub course_id: String,
    pub language: String,
    pub code: String,
    pub output: Option<String>,
    pub duration_seconds: Option<i32>,
    pub time_limit_seconds: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCodeSession {
    pub language: String,
    pub code: String,
    pub time_limit_seconds: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteCode {
    pub language: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct ExecutionResult {
    pub output: String,
    pub execution_time_ms: u128,
    pub success: bool,
    pub error: Option<String>,
}
