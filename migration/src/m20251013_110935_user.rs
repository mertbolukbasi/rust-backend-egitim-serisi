use sea_orm_migration::{prelude::*, schema::*};
use crate::m20251013_110704_project::Project;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
	                .col(
		                ColumnDef::new(User::FirstName)
			                .string()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(User::LastName)
			                .string()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(User::Email)
			                .string()
			                .unique_key()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(User::Password)
			                .string()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(User::IsActive)
			                .boolean()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(User::ProjectId)
			                .integer()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(User::CreatedAt)
			                .timestamp_with_time_zone()
			                .not_null()
			                .default(Expr::current_timestamp())
	                )
	                .col(
		                ColumnDef::new(User::UpdatedAt)
			                .timestamp_with_time_zone()
			                .not_null()
			                .default(Expr::current_timestamp())
	                )
	                .foreign_key(
		                ForeignKey::create()
			                .from(User::Table, User::ProjectId)
			                .to(Project::Table, Project::Id)
			                .on_delete(ForeignKeyAction::Cascade)
			                .on_update(ForeignKeyAction::Cascade),
	                )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    FirstName,
    LastName,
	Email,
	Password,
	IsActive,
	ProjectId,
	CreatedAt,
	UpdatedAt
}
