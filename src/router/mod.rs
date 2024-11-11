mod hello_world;

use axum::{routing::get, Router};
use hello_world::hello_world;

pub fn router_app() -> Router {
    Router::new().route("/hello_world", get(hello_world))
}
