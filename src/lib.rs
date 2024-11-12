mod router;

use migration::{Migrator, MigratorTrait};
use router::router_app;
use sea_orm::Database;

pub async fn run_app(db_url: &str) {
    let db = Database::connect(db_url).await.unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    Migrator::up(&db, None).await.unwrap();

    axum::serve(listener, router_app(db)).await.unwrap();
}
