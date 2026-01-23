//! 文件实体

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub submission_token: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_type: String,
    pub uploaded_at: i64,
    pub citation_count: Option<i32>,
    pub user_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// 从数据库模型转换为业务模型
impl Model {
    pub fn into_file(self) -> crate::models::files::entities::File {
        use crate::models::files::entities::File;
        use chrono::{DateTime, Utc};

        File {
            submission_token: self.submission_token,
            file_name: self.file_name,
            file_size: self.file_size,
            file_type: self.file_type,
            uploaded_at: DateTime::<Utc>::from_timestamp(self.uploaded_at, 0).unwrap_or_default(),
            user_id: self.user_id,
        }
    }
}
