//! 提交存储操作

use std::collections::HashMap;

use super::SeaOrmStorage;
use crate::entity::grades::{Column as GradeColumn, Entity as Grades};
use crate::entity::submission_files::{
    ActiveModel as SubmissionFileActiveModel, Column as SubmissionFileColumn,
    Entity as SubmissionFiles,
};
use crate::entity::submissions::{ActiveModel, Column, Entity as Submissions};
use crate::entity::users::{Column as UserColumn, Entity as Users};
use crate::errors::{HWSystemError, Result};
use crate::models::{
    PaginationInfo,
    submissions::{
        entities::{Submission, SubmissionStatus},
        requests::{CreateSubmissionRequest, SubmissionListQuery},
        responses::{
            LatestSubmissionInfo, SubmissionCreator, SubmissionGradeInfo, SubmissionListItem,
            SubmissionListResponse, SubmissionSummaryItem, SubmissionSummaryResponse,
        },
    },
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};

impl SeaOrmStorage {
    /// 创建提交（自动计算版本号）
    pub async fn create_submission_impl(
        &self,
        creator_id: i64,
        req: CreateSubmissionRequest,
    ) -> Result<Submission> {
        let now = chrono::Utc::now().timestamp();

        // 查询当前最大版本号
        let max_version = Submissions::find()
            .filter(Column::HomeworkId.eq(req.homework_id))
            .filter(Column::CreatorId.eq(creator_id))
            .select_only()
            .column_as(Column::Version.max(), "max_version")
            .into_tuple::<Option<i32>>()
            .one(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询最大版本号失败: {e}")))?
            .flatten()
            .unwrap_or(0);

        let version = max_version + 1;

        // 检查是否迟交
        let homework = self.get_homework_by_id_impl(req.homework_id).await?;
        let is_late = if let Some(hw) = homework {
            if let Some(deadline) = hw.deadline {
                chrono::Utc::now() > deadline
            } else {
                false
            }
        } else {
            false
        };

        let status = if is_late {
            SubmissionStatus::Late.to_string()
        } else {
            SubmissionStatus::Pending.to_string()
        };

        let model = ActiveModel {
            homework_id: Set(req.homework_id),
            creator_id: Set(creator_id),
            version: Set(version),
            content: Set(Some(req.content)),
            status: Set(status),
            is_late: Set(is_late),
            submitted_at: Set(now),
            ..Default::default()
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("创建提交失败: {e}")))?;

        // 处理附件
        if let Some(attachments) = req.attachments {
            self.set_submission_files_impl(result.id, attachments, creator_id)
                .await?;
        }

        Ok(result.into_submission())
    }

    /// 通过 ID 获取提交
    pub async fn get_submission_by_id_impl(
        &self,
        submission_id: i64,
    ) -> Result<Option<Submission>> {
        let result = Submissions::find_by_id(submission_id)
            .one(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询提交失败: {e}")))?;

        Ok(result.map(|m| m.into_submission()))
    }

    /// 获取学生某作业的最新提交
    pub async fn get_latest_submission_impl(
        &self,
        homework_id: i64,
        creator_id: i64,
    ) -> Result<Option<Submission>> {
        let result = Submissions::find()
            .filter(Column::HomeworkId.eq(homework_id))
            .filter(Column::CreatorId.eq(creator_id))
            .order_by_desc(Column::Version)
            .one(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询最新提交失败: {e}")))?;

        Ok(result.map(|m| m.into_submission()))
    }

    /// 获取学生某作业的提交历史
    pub async fn list_user_submissions_impl(
        &self,
        homework_id: i64,
        creator_id: i64,
    ) -> Result<Vec<Submission>> {
        let results = Submissions::find()
            .filter(Column::HomeworkId.eq(homework_id))
            .filter(Column::CreatorId.eq(creator_id))
            .order_by_desc(Column::Version)
            .all(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询提交历史失败: {e}")))?;

        Ok(results.into_iter().map(|m| m.into_submission()).collect())
    }

    /// 列出提交（分页）
    pub async fn list_submissions_with_pagination_impl(
        &self,
        query: SubmissionListQuery,
    ) -> Result<SubmissionListResponse> {
        let page = query.page.unwrap_or(1).max(1) as u64;
        let size = query.size.unwrap_or(10).clamp(1, 100) as u64;

        let mut select = Submissions::find();

        // 作业筛选
        if let Some(homework_id) = query.homework_id {
            select = select.filter(Column::HomeworkId.eq(homework_id));
        }

        // 提交者筛选
        if let Some(creator_id) = query.creator_id {
            select = select.filter(Column::CreatorId.eq(creator_id));
        }

        // 状态筛选
        if let Some(ref status) = query.status {
            select = select.filter(Column::Status.eq(status));
        }

        // 排序
        select = select.order_by_desc(Column::SubmittedAt);

        // 分页查询
        let paginator = select.paginate(&self.db, size);
        let total = paginator
            .num_items()
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询提交总数失败: {e}")))?;

        let pages = paginator
            .num_pages()
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询提交页数失败: {e}")))?;

        let submissions = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询提交列表失败: {e}")))?;

        // 批量查询用户信息
        let creator_ids: Vec<i64> = submissions
            .iter()
            .map(|s| s.creator_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let users = Users::find()
            .filter(UserColumn::Id.is_in(creator_ids))
            .all(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询用户信息失败: {e}")))?;

        let user_map: HashMap<i64, _> = users.into_iter().map(|u| (u.id, u)).collect();

        // 组装 SubmissionListItem
        let items = submissions
            .into_iter()
            .map(|s| {
                let creator = user_map.get(&s.creator_id);
                SubmissionListItem {
                    id: s.id,
                    homework_id: s.homework_id,
                    creator_id: s.creator_id,
                    creator: SubmissionCreator {
                        id: creator.map(|u| u.id).unwrap_or(s.creator_id),
                        username: creator
                            .map(|u| u.username.clone())
                            .unwrap_or_else(|| "未知用户".to_string()),
                        display_name: creator.and_then(|u| u.display_name.clone()),
                    },
                    version: s.version,
                    content: s.content,
                    status: s.status,
                    is_late: s.is_late,
                    submitted_at: chrono::DateTime::from_timestamp(s.submitted_at, 0)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default(),
                }
            })
            .collect();

        Ok(SubmissionListResponse {
            items,
            pagination: PaginationInfo {
                page: page as i64,
                page_size: size as i64,
                total: total as i64,
                total_pages: pages as i64,
            },
        })
    }

    /// 删除提交（撤回）
    pub async fn delete_submission_impl(&self, submission_id: i64) -> Result<bool> {
        // 先删除附件关联
        SubmissionFiles::delete_many()
            .filter(SubmissionFileColumn::SubmissionId.eq(submission_id))
            .exec(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("删除提交附件关联失败: {e}")))?;

        let result = Submissions::delete_by_id(submission_id)
            .exec(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("删除提交失败: {e}")))?;

        Ok(result.rows_affected > 0)
    }

    /// 更新提交状态
    pub async fn update_submission_status_impl(
        &self,
        submission_id: i64,
        status: &str,
    ) -> Result<bool> {
        let result = Submissions::update_many()
            .col_expr(
                Column::Status,
                sea_orm::sea_query::Expr::value(status.to_string()),
            )
            .filter(Column::Id.eq(submission_id))
            .exec(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("更新提交状态失败: {e}")))?;

        Ok(result.rows_affected > 0)
    }

    /// 获取提交附件 ID 列表
    pub async fn get_submission_file_ids_impl(&self, submission_id: i64) -> Result<Vec<i64>> {
        let results = SubmissionFiles::find()
            .filter(SubmissionFileColumn::SubmissionId.eq(submission_id))
            .all(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询提交附件失败: {e}")))?;

        Ok(results.into_iter().map(|m| m.file_id).collect())
    }

    /// 设置提交附件（通过 download_token，带所有权校验）
    pub async fn set_submission_files_impl(
        &self,
        submission_id: i64,
        tokens: Vec<String>,
        user_id: i64,
    ) -> Result<()> {
        // 先删除旧的关联
        SubmissionFiles::delete_many()
            .filter(SubmissionFileColumn::SubmissionId.eq(submission_id))
            .exec(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("删除旧附件关联失败: {e}")))?;

        // 通过 token 查找文件并校验所有权
        for token in tokens {
            let file = self
                .get_file_by_token_impl(&token)
                .await?
                .ok_or_else(|| HWSystemError::not_found(format!("文件不存在: {token}")))?;

            // 校验文件所有权
            if file.user_id != Some(user_id) {
                return Err(HWSystemError::authorization(format!(
                    "无权使用此文件: {token}"
                )));
            }

            let model = SubmissionFileActiveModel {
                submission_id: Set(submission_id),
                file_id: Set(file.id),
            };

            model
                .insert(&self.db)
                .await
                .map_err(|e| HWSystemError::database_operation(format!("创建附件关联失败: {e}")))?;

            // 增加文件引用计数
            self.increment_file_citation_impl(file.id).await?;
        }

        Ok(())
    }

    /// 获取作业提交概览（按学生聚合）
    pub async fn get_submission_summary_impl(
        &self,
        homework_id: i64,
        page: i64,
        size: i64,
    ) -> Result<SubmissionSummaryResponse> {
        let page = page.max(1) as u64;
        let size = size.clamp(1, 100) as u64;

        // 1. 查询该作业所有提交（按 creator_id 和 version 倒序）
        let all_submissions = Submissions::find()
            .filter(Column::HomeworkId.eq(homework_id))
            .order_by_desc(Column::Version)
            .all(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询提交列表失败: {e}")))?;

        if all_submissions.is_empty() {
            return Ok(SubmissionSummaryResponse {
                items: vec![],
                pagination: PaginationInfo {
                    page: page as i64,
                    page_size: size as i64,
                    total: 0,
                    total_pages: 0,
                },
            });
        }

        // 2. 按 creator_id 聚合，取每个用户的最新提交和版本数
        let mut user_latest: HashMap<i64, (&crate::entity::submissions::Model, i32)> =
            HashMap::new();
        for sub in &all_submissions {
            user_latest
                .entry(sub.creator_id)
                .and_modify(|(_, count)| *count += 1)
                .or_insert((sub, 1));
        }

        // 3. 分页
        let total = user_latest.len() as u64;
        let pages = total.div_ceil(size);
        let skip = ((page - 1) * size) as usize;

        let mut user_data: Vec<_> = user_latest.into_iter().collect();
        // 按提交时间倒序排序
        user_data.sort_by(|a, b| b.1.0.submitted_at.cmp(&a.1.0.submitted_at));

        let paged_data: Vec<_> = user_data
            .into_iter()
            .skip(skip)
            .take(size as usize)
            .collect();

        // 4. 批量查询用户信息
        let creator_ids: Vec<i64> = paged_data.iter().map(|(id, _)| *id).collect();
        let users = Users::find()
            .filter(UserColumn::Id.is_in(creator_ids.clone()))
            .all(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询用户信息失败: {e}")))?;
        let user_map: HashMap<i64, _> = users.into_iter().map(|u| (u.id, u)).collect();

        // 5. 批量查询评分信息（根据最新提交 ID）
        let submission_ids: Vec<i64> = paged_data.iter().map(|(_, (sub, _))| sub.id).collect();
        let grades = Grades::find()
            .filter(GradeColumn::SubmissionId.is_in(submission_ids))
            .all(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询评分信息失败: {e}")))?;
        let grade_map: HashMap<i64, _> = grades.into_iter().map(|g| (g.submission_id, g)).collect();

        // 6. 组装结果
        let items = paged_data
            .into_iter()
            .map(|(creator_id, (sub, version_count))| {
                let user = user_map.get(&creator_id);
                let grade = grade_map.get(&sub.id);

                SubmissionSummaryItem {
                    creator: SubmissionCreator {
                        id: creator_id,
                        username: user
                            .map(|u| u.username.clone())
                            .unwrap_or_else(|| "未知用户".to_string()),
                        display_name: user.and_then(|u| u.display_name.clone()),
                    },
                    latest_submission: LatestSubmissionInfo {
                        id: sub.id,
                        version: sub.version,
                        status: sub.status.clone(),
                        is_late: sub.is_late,
                        submitted_at: chrono::DateTime::from_timestamp(sub.submitted_at, 0)
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_default(),
                    },
                    grade: grade.map(|g| SubmissionGradeInfo {
                        score: g.score,
                        comment: g.comment.clone(),
                        graded_at: chrono::DateTime::from_timestamp(g.graded_at, 0)
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_default(),
                    }),
                    total_versions: version_count,
                }
            })
            .collect();

        Ok(SubmissionSummaryResponse {
            items,
            pagination: PaginationInfo {
                page: page as i64,
                page_size: size as i64,
                total: total as i64,
                total_pages: pages as i64,
            },
        })
    }

    /// 获取某学生某作业的所有提交版本（教师视角）
    pub async fn list_user_submissions_for_teacher_impl(
        &self,
        homework_id: i64,
        user_id: i64,
    ) -> Result<Vec<Submission>> {
        // 复用现有的 list_user_submissions_impl，它已经实现了按版本倒序
        self.list_user_submissions_impl(homework_id, user_id).await
    }
}
