//! 评分实体

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "grades")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub submission_id: i64,
    pub grader_id: i64,
    pub score: f64,
    #[sea_orm(column_type = "Text", nullable)]
    pub comment: Option<String>,
    pub graded_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::submissions::Entity",
        from = "Column::SubmissionId",
        to = "super::submissions::Column::Id"
    )]
    Submission,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::GraderId",
        to = "super::users::Column::Id"
    )]
    Grader,
}

impl Related<super::submissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Submission.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Grader.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
