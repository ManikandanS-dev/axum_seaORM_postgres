mod cake;
mod cake_filling;
mod filling;
mod fruit;
mod hello_world;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use cake::cake_save;
use cake_filling::cake_filling_save;
use filling::filling_save;
use fruit::fruit_save;
use hello_world::hello_world;

use sea_orm::DatabaseConnection;

pub fn router_app(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello_world", get(hello_world))
        .route("/cake_save", post(cake_save))
        .route("/fruit_save", post(fruit_save))
        .route("/filling_save", post(filling_save))
        .route("/cake_filling_save", post(cake_filling_save))
        .layer(Extension(db))
}
