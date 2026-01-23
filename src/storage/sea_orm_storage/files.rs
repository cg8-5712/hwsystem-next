//! 文件存储操作

use super::SeaOrmStorage;
use crate::entity::files::{ActiveModel, Column, Entity as Files};
use crate::errors::{HWSystemError, Result};
use crate::models::files::entities::File;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

impl SeaOrmStorage {
    /// 上传文件（创建文件记录）
    pub async fn upload_file_impl(
        &self,
        submission_token: &str,
        file_name: &str,
        file_size: &i64,
        file_type: &str,
        user_id: i64,
    ) -> Result<File> {
        let now = chrono::Utc::now().timestamp();

        let model = ActiveModel {
            submission_token: Set(submission_token.to_string()),
            file_name: Set(file_name.to_string()),
            file_size: Set(*file_size),
            file_type: Set(file_type.to_string()),
            uploaded_at: Set(now),
            citation_count: Set(Some(0)),
            user_id: Set(user_id),
        };

        let result = model
            .insert(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("上传文件记录失败: {e}")))?;

        Ok(result.into_file())
    }

    /// 通过 token 获取文件
    pub async fn get_file_by_token_impl(&self, token: &str) -> Result<Option<File>> {
        let result = Files::find()
            .filter(Column::SubmissionToken.eq(token))
            .one(&self.db)
            .await
            .map_err(|e| HWSystemError::database_operation(format!("查询文件失败: {e}")))?;

        Ok(result.map(|m| m.into_file()))
    }
}
