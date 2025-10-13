use sea_orm_migration::{prelude::*, schema::*};
use crate::m20251013_110140_role::Role;
use crate::m20251013_110935_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
	                .col(
		                ColumnDef::new(UserRole::RoleId)
			                .integer()
			                .not_null()
	                )
	                .col(
		                ColumnDef::new(UserRole::UserId)
			                .integer()
			                .not_null(),
	                )
	                .primary_key(
		                Index::create()
			                .col(UserRole::UserId)
			                .col(UserRole::RoleId)
	                )
	                .foreign_key(
		                ForeignKey::create()
			                .from(UserRole::Table, UserRole::UserId)
			                .to(User::Table, User::Id)
			                .on_delete(ForeignKeyAction::Cascade)
			                .on_update(ForeignKeyAction::Cascade)
	                )
	                .foreign_key(
		                ForeignKey::create()
			                .from(UserRole::Table, UserRole::RoleId)
			                .to(Role::Table, Role::Id)
			                .on_delete(ForeignKeyAction::Cascade)
			                .on_update(ForeignKeyAction::Cascade)
	                )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserRole {
    Table,
	RoleId,
	UserId
}
