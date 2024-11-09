#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241109_171251_events;
mod m20241109_180343_user;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // inject-below (do not remove this comment)
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20241109_171251_events::Migration),
            Box::new(m20241109_180343_user::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
