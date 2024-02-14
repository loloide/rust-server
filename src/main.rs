use axum::{Router};
use tower_http::services::ServeDir;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .nest_service("/public", ServeDir::new("public"))
        .nest_service("/asd", ServeDir::new("asd"));

    Ok(router.into())
}