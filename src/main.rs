use std::error::Error;

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn hello_name(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", name)
}

#[derive(Deserialize)]
struct Hexify {
    dec_value: String,
}

async fn dec_to_hex(Json(hex_req): Json<Hexify>) -> Result<String, Box<dyn Error>> {
    let hex_str = decimal_to_hex(&hex_req.dec_value)?;
    Ok(hex_str)
}

fn decimal_to_hex(value: &str) -> Result<String, Box<dyn Error>> {
    let value = value.parse::<i64>()?;
    let value = format!("{:X}", value);
    Ok(value)
}

fn hex_to_decimal(value: &str) -> Result<String, Box<dyn Error>> {
    let value = i64::from_str_radix(value, 16)?;
    Ok(value.to_string())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/:name", get(hello_name))
        .route("/hexify", post(dec_to_hex));

    Ok(router.into())
}
