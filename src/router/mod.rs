mod hello_world;

use axum::{routing::get, Extension, Router};
use hello_world::hello_world;

use sea_orm::DatabaseConnection;

pub fn router_app(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello_world", get(hello_world))
        .layer(Extension(db))
}
