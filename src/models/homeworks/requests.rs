use crate::models::common::pagination::PaginationQuery;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../frontend/src/types/generated/homework.ts")]
pub struct HomeworkListQuery {
    #[serde(flatten)]
    #[ts(flatten)]
    pub pagination: PaginationQuery,
    pub status: Option<String>,
    pub search: Option<String>,
    pub order_by: Option<String>,
    pub order: Option<String>,
}
