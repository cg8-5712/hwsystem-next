use crate::models::common::pagination::PaginationInfo;
use crate::models::homeworks::entities::Homework;
use serde::Serialize;
use serde_json::Value;
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/homework.ts")]
pub struct HomeworkResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub deadline: String,
    pub max_score: i32,
    pub allow_late_submission: bool,
    pub attachments: Vec<Option<String>>,
    pub submission_count: i32,
    pub status: String,
    pub created_by: Value,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/homework.ts")]
pub struct HomeworkListResponse {
    pub items: Vec<Homework>,
    pub pagination: PaginationInfo,
}
