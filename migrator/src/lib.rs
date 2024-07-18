pub use sea_orm_migration::prelude::*;

mod user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(user::m20240717_203614_users_and_session::Migration)
        ]
    }
}
