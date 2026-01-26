//! 系统设置实体

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "system_settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub key: String,
    pub value: String,
    pub value_type: String,
    pub description: Option<String>,
    pub updated_at: i64,
    pub updated_by: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// 从数据库模型转换为业务模型
impl Model {
    pub fn into_setting(self) -> crate::models::system::entities::SystemSetting {
        use crate::models::system::entities::{SettingValueType, SystemSetting};
        use chrono::{DateTime, Utc};

        SystemSetting {
            key: self.key,
            value: self.value,
            value_type: self
                .value_type
                .parse::<SettingValueType>()
                .unwrap_or(SettingValueType::String),
            description: self.description,
            updated_at: DateTime::<Utc>::from_timestamp(self.updated_at, 0).unwrap_or_default(),
            updated_by: self.updated_by,
        }
    }
}
