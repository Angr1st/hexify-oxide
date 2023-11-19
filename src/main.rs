use axum::{extract::Path, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn hello_name(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", name)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/:name", get(hello_name));

    Ok(router.into())
}
