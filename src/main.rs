use std::env;

use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use oxid_gateway::{
    models::{NewTarget, Target},
    schema::target,
};
use deadpool_diesel::postgres::{Manager, Pool};
use diesel::{ RunQueryDsl, SelectableHelper};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Clone)]
struct AppState {
    pool: Pool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = get_postgres_pool().await.unwrap();

    let connection = pool.get().await.unwrap();

    migrate(connection).await;

    let app = Router::new()
        .route("/targets", post(create_target))
        .with_state(AppState { pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_postgres_pool() -> Result<Pool, deadpool_diesel::postgres::BuildError> {
    let postgres_url = get_database().await;
    let manager = Manager::new(postgres_url, deadpool_diesel::Runtime::Tokio1);
    return Pool::builder(manager).build();
}

async fn migrate(connection: deadpool_diesel::postgres::Object) {
    connection
        .interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}

async fn create_target(
    State(app_state): State<AppState>,
    Json(body): Json<NewTarget>,
) -> Result<Json<Target>, (StatusCode, String)> {
    let manager = app_state.pool.get().await.expect("Failed to get pool");

    let res = manager
        .interact(|conn| {
            diesel::insert_into(target::table)
                .values(body)
                .returning(Target::as_returning())
                .get_result(conn)
        })
        .await
        .unwrap()
        .unwrap();

    Ok(Json(res))
}

async fn get_database() -> String {
    env::var("DATABASE_URL")
        .expect("DATABASE_URL not found")
        .to_string()
}
