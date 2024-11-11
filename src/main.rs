use axum_seaorm_postgres::run_app;

use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL");

    run_app(&database_url).await;
}
