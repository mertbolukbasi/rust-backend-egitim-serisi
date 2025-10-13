pub use sea_orm_migration::prelude::*;

mod m20251013_110140_role;
mod m20251013_110704_project;
mod m20251013_110935_user;
mod m20251013_112146_task;
mod m20251013_113439_user_role;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251013_110140_role::Migration),
            Box::new(m20251013_110704_project::Migration),
            Box::new(m20251013_110935_user::Migration),
            Box::new(m20251013_112146_task::Migration),
            Box::new(m20251013_113439_user_role::Migration),
        ]
    }
}
