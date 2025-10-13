use sea_orm_migration::{prelude::*, schema::*};
use crate::m20251013_110704_project::Project;
use crate::m20251013_110935_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_auto(Task::Id))
	                .col(
		                ColumnDef::new(Task::TaskName)
			                .string()
			                .unique_key()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(Task::IsActive)
			                .boolean()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(Task::ProjectId)
			                .integer()
		                    .not_null(),
	                )
	                .col(
		                ColumnDef::new(Task::UserId)
			                .integer()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(Task::CreatedAt)
			                .timestamp_with_time_zone()
			                .not_null()
			                .default(Expr::current_timestamp())
	                )
	                .col(
		                ColumnDef::new(Task::UpdatedAt)
			                .timestamp_with_time_zone()
			                .not_null()
			                .default(Expr::current_timestamp())
	                )
	                .foreign_key(
		                ForeignKey::create()
			                .from(Task::Table, Task::ProjectId)
			                .to(Project::Table, Project::Id)
			                .on_delete(ForeignKeyAction::Cascade)
			                .on_update(ForeignKeyAction::Cascade),
	                )
	                .foreign_key(
		                ForeignKey::create()
			                .from(Task::Table, Task::UserId)
			                .to(User::Table, User::Id)
			                .on_delete(ForeignKeyAction::Cascade)
			                .on_update(ForeignKeyAction::Cascade),
	                )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    TaskName,
    IsActive,
	ProjectId,
	UserId,
	CreatedAt,
	UpdatedAt,
}
