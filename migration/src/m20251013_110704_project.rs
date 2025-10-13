use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(pk_auto(Project::Id))
	                .col(
		                ColumnDef::new(Project::ProjectName)
			                .string()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(Project::IsActive)
			                .boolean()
			                .not_null(),
	                )
	                .col(
		                ColumnDef::new(Project::CreatedAt)
			                .timestamp_with_time_zone()
			                .not_null()
			                .default(Expr::current_timestamp())
	                )
	                .col(
		                ColumnDef::new(Project::UpdatedAt)
			                .timestamp_with_time_zone()
			                .not_null()
			                .default(Expr::current_timestamp())
	                )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Project {
    Table,
    Id,
    ProjectName,
    IsActive,
	CreatedAt,
	UpdatedAt,
}
