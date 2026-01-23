use super::entities::Class;
use crate::models::common::PaginationInfo;
use serde::Serialize;
use ts_rs::TS;

// 班级列表响应
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/class.ts")]
pub struct ClassListResponse {
    pub pagination: PaginationInfo,
    pub items: Vec<Class>,
}
