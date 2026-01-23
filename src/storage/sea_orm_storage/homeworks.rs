//! 作业存储操作

use super::SeaOrmStorage;
use crate::entity::homeworks::{Column, Entity as Homeworks};
use crate::errors::{HWSystemError, Result};
use crate::models::{
    PaginationInfo,
    homeworks::{requests::HomeworkListQuery, responses::HomeworkListResponse},
};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

impl SeaOrmStorage {
    /// 分页列出作业
    pub async fn list_homeworks_with_pagination_impl(
        &self,
        query: HomeworkListQuery,
    ) -> Result<HomeworkListResponse> {
        let page = query.pagination.page.max(1) as u64;
        let size = query.pagination.size.clamp(1, 100) as u64;

        let mut select = Homeworks::find();

        // 搜索条件（按标题搜索）
        if let Some(ref search) = query.search
            && !search.trim().is_empty()
        {
            select = select.filter(Column::Title.contains(search.trim()));
        }

        // 排序
        select = select.order_by_desc(Column::CreatedAt);

        // 分页查询
        let paginator = select.paginate(&self.db, size);
        let total = paginator
            .num_items()
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询作业总数失败: {e}")))?;

        let pages = paginator
            .num_pages()
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询作业页数失败: {e}")))?;

        let homeworks = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询作业列表失败: {e}")))?;

        Ok(HomeworkListResponse {
            items: homeworks.into_iter().map(|m| m.into_homework()).collect(),
            pagination: PaginationInfo {
                page: page as i64,
                size: size as i64,
                total: total as i64,
                pages: pages as i64,
            },
        })
    }
}
