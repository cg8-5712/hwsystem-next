//! 提交实体

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "submissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub homework_id: i64,
    pub creator_id: i64,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub attachments: Option<String>,
    pub submitted_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::homeworks::Entity",
        from = "Column::HomeworkId",
        to = "super::homeworks::Column::Id"
    )]
    Homework,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::CreatorId",
        to = "super::users::Column::Id"
    )]
    Creator,
    #[sea_orm(has_many = "super::grades::Entity")]
    Grades,
}

impl Related<super::homeworks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Homework.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Creator.def()
    }
}

impl Related<super::grades::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grades.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
