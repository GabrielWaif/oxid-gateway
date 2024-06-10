pub mod entities;
pub mod repositories;
pub mod errors;

use std::env;

use deadpool_diesel::postgres::{Manager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub async fn get_postgres_pool() -> Result<Pool, deadpool_diesel::postgres::BuildError> {
    let postgres_url = get_database().await;
    let manager = Manager::new(postgres_url, deadpool_diesel::Runtime::Tokio1);
    return Pool::builder(manager).build();
}

pub async fn migrate(connection: deadpool_diesel::postgres::Object) {
    connection
        .interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}

pub async fn get_pool_connection(pool: &Pool) -> deadpool_diesel::postgres::Object {
    pool.get().await.expect("Failed to get pool")
}

pub async fn get_database() -> String {
    env::var("DATABASE_URL")
        .expect("DATABASE_URL not found")
        .to_string()
}
