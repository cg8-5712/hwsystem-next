use serde::Serialize;
use ts_rs::TS;

use crate::models::{PaginationInfo, class_users::entities::ClassUser};

/// 班级学生列表响应
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/class-user.ts")]
pub struct ClassUserListResponse {
    pub pagination: PaginationInfo,
    pub items: Vec<ClassUser>,
}
