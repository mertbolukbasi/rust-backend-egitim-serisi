use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(pk_auto(Role::Id))
	                .col(
		                ColumnDef::new(Role::RoleName)
			                .string()
			                .unique_key()
			                .not_null()
	                )
	                .col(
		                ColumnDef::new(Role::CreatedAt)
			                .timestamp_with_time_zone()
			                .not_null()
			                .default(Expr::current_timestamp())
	                )
	                .col(
		                ColumnDef::new(Role::UpdatedAt)
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
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Id,
    RoleName,
    CreatedAt,
	UpdatedAt,
}
