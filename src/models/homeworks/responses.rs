use crate::models::common::pagination::PaginationInfo;
use crate::models::files::responses::FileInfo;
use crate::models::homeworks::entities::Homework;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/homework.ts")]
pub struct HomeworkCreator {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/homework.ts")]
pub struct HomeworkResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub deadline: Option<String>,
    pub max_score: f64,
    pub allow_late_submission: bool,
    pub attachments: Vec<String>,
    pub submission_count: i32,
    pub status: String,
    pub created_by: HomeworkCreator,
    pub created_at: String,
    pub updated_at: String,
}

/// 带创建者信息的作业（用于列表，旧版兼容）
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/homework.ts")]
pub struct HomeworkWithCreator {
    #[serde(flatten)]
    pub homework: Homework,
    pub creator: Option<HomeworkCreator>,
}

/// 我的提交摘要（用于作业列表显示提交状态）
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/homework.ts")]
pub struct MySubmissionSummary {
    pub id: i64,
    pub version: i32,
    pub status: String,
    pub is_late: bool,
    pub score: Option<f64>,
}

/// 作业列表项（包含创建者和我的提交状态）
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/homework.ts")]
pub struct HomeworkListItem {
    #[serde(flatten)]
    pub homework: Homework,
    pub creator: Option<HomeworkCreator>,
    /// 当前用户的最新提交（仅学生视角有值）
    pub my_submission: Option<MySubmissionSummary>,
}

/// 作业详情（包含附件和创建者）
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/homework.ts")]
pub struct HomeworkDetail {
    #[serde(flatten)]
    pub homework: Homework,
    pub attachments: Vec<FileInfo>,
    pub creator: Option<HomeworkCreator>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/homework.ts")]
pub struct HomeworkListResponse {
    pub items: Vec<HomeworkListItem>,
    pub pagination: PaginationInfo,
}
