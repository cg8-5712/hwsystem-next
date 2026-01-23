use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建用户表
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::Role).string().not_null())
                    .col(ColumnDef::new(Users::Status).string().not_null())
                    .col(ColumnDef::new(Users::ProfileName).string().null())
                    .col(ColumnDef::new(Users::AvatarUrl).string().null())
                    .col(ColumnDef::new(Users::LastLogin).big_integer().null())
                    .col(ColumnDef::new(Users::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).big_integer().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建班级表
        manager
            .create_table(
                Table::create()
                    .table(Classes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Classes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Classes::TeacherId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Classes::ClassName)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Classes::Description).text().null())
                    .col(
                        ColumnDef::new(Classes::InviteCode)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Classes::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(Classes::UpdatedAt).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Classes::Table, Classes::TeacherId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建班级用户关联表
        manager
            .create_table(
                Table::create()
                    .table(ClassUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ClassUsers::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ClassUsers::ClassId).big_integer().not_null())
                    .col(ColumnDef::new(ClassUsers::UserId).big_integer().not_null())
                    .col(ColumnDef::new(ClassUsers::Role).string().not_null())
                    .col(ColumnDef::new(ClassUsers::ProfileName).string().null())
                    .col(
                        ColumnDef::new(ClassUsers::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClassUsers::JoinedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ClassUsers::Table, ClassUsers::ClassId)
                            .to(Classes::Table, Classes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ClassUsers::Table, ClassUsers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建作业表
        manager
            .create_table(
                Table::create()
                    .table(Homeworks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Homeworks::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Homeworks::ClassId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Homeworks::CreatedBy)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Homeworks::Title).string().not_null())
                    .col(ColumnDef::new(Homeworks::Content).text().null())
                    .col(ColumnDef::new(Homeworks::Attachments).text().null())
                    .col(ColumnDef::new(Homeworks::MaxScore).double().not_null())
                    .col(ColumnDef::new(Homeworks::Deadline).big_integer().null())
                    .col(
                        ColumnDef::new(Homeworks::AllowLateSubmission)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Homeworks::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Homeworks::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Homeworks::Table, Homeworks::ClassId)
                            .to(Classes::Table, Classes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Homeworks::Table, Homeworks::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建提交表
        manager
            .create_table(
                Table::create()
                    .table(Submissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Submissions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Submissions::HomeworkId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Submissions::CreatorId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Submissions::Content).text().not_null())
                    .col(ColumnDef::new(Submissions::Attachments).text().null())
                    .col(
                        ColumnDef::new(Submissions::SubmittedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Submissions::Table, Submissions::HomeworkId)
                            .to(Homeworks::Table, Homeworks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Submissions::Table, Submissions::CreatorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建评分表
        manager
            .create_table(
                Table::create()
                    .table(Grades::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Grades::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Grades::SubmissionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Grades::GraderId).big_integer().not_null())
                    .col(ColumnDef::new(Grades::Score).double().not_null())
                    .col(ColumnDef::new(Grades::Comment).text().null())
                    .col(ColumnDef::new(Grades::GradedAt).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Grades::Table, Grades::SubmissionId)
                            .to(Submissions::Table, Submissions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Grades::Table, Grades::GraderId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建文件表
        manager
            .create_table(
                Table::create()
                    .table(Files::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Files::SubmissionToken)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Files::FileName).string().not_null())
                    .col(ColumnDef::new(Files::FileSize).big_integer().not_null())
                    .col(ColumnDef::new(Files::FileType).string().not_null())
                    .col(ColumnDef::new(Files::UploadedAt).big_integer().not_null())
                    .col(ColumnDef::new(Files::CitationCount).integer().default(0))
                    .col(ColumnDef::new(Files::UserId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Files::Table, Files::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建索引
        // 用户表索引
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_username")
                    .table(Users::Table)
                    .col(Users::Username)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_users_role")
                    .table(Users::Table)
                    .col(Users::Role)
                    .to_owned(),
            )
            .await?;

        // 班级表索引
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_classes_teacher_id")
                    .table(Classes::Table)
                    .col(Classes::TeacherId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_classes_invite_code")
                    .table(Classes::Table)
                    .col(Classes::InviteCode)
                    .to_owned(),
            )
            .await?;

        // 班级用户表索引
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_class_users_class_id")
                    .table(ClassUsers::Table)
                    .col(ClassUsers::ClassId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_class_users_user_id")
                    .table(ClassUsers::Table)
                    .col(ClassUsers::UserId)
                    .to_owned(),
            )
            .await?;

        // 文件表索引
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_files_user_id")
                    .table(Files::Table)
                    .col(Files::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 按照创建的相反顺序删除
        manager
            .drop_table(Table::drop().table(Files::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Grades::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Submissions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Homeworks::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ClassUsers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Classes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    #[sea_orm(iden = "users")]
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    Role,
    Status,
    ProfileName,
    AvatarUrl,
    LastLogin,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Classes {
    #[sea_orm(iden = "classes")]
    Table,
    Id,
    TeacherId,
    ClassName,
    Description,
    InviteCode,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ClassUsers {
    #[sea_orm(iden = "class_users")]
    Table,
    Id,
    ClassId,
    UserId,
    Role,
    ProfileName,
    UpdatedAt,
    JoinedAt,
}

#[derive(DeriveIden)]
enum Homeworks {
    #[sea_orm(iden = "homeworks")]
    Table,
    Id,
    ClassId,
    CreatedBy,
    Title,
    Content,
    Attachments,
    MaxScore,
    Deadline,
    AllowLateSubmission,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Submissions {
    #[sea_orm(iden = "submissions")]
    Table,
    Id,
    HomeworkId,
    CreatorId,
    Content,
    Attachments,
    SubmittedAt,
}

#[derive(DeriveIden)]
enum Grades {
    #[sea_orm(iden = "grades")]
    Table,
    Id,
    SubmissionId,
    GraderId,
    Score,
    Comment,
    GradedAt,
}

#[derive(DeriveIden)]
enum Files {
    #[sea_orm(iden = "files")]
    Table,
    SubmissionToken,
    FileName,
    FileSize,
    FileType,
    UploadedAt,
    CitationCount,
    UserId,
}
