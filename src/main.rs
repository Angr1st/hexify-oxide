use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

// Make our own error
enum AppError {
    ParseIntError(core::num::ParseIntError),
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            format!(
                "Something went wrong: {}",
                match self {
                    Self::ParseIntError(err) => err,
                }
            ),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<core::num::ParseIntError>,
{
    fn from(err: E) -> Self {
        Self::ParseIntError(err.into())
    }
}

#[derive(Serialize)]
struct JsonError {
    error: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

async fn hello_world() -> impl IntoResponse {
    IndexTemplate {
        name: String::from("world"),
    }
}

async fn hello_name(Path(name): Path<String>) -> impl IntoResponse {
    IndexTemplate { name }
}

#[derive(Template)]
#[template(path = "hex-result.html")]
struct HexResult {
    value: String,
}

async fn hex_result(Json(hex_req): Json<Hexify>) -> Result<HexResult, AppError> {
    let hex_str = decimal_to_hex(&hex_req.dec_value)?;
    Ok(HexResult { value: hex_str })
}

#[derive(Deserialize)]
struct Hexify {
    dec_value: String,
}

async fn dec_to_hex(Json(hex_req): Json<Hexify>) -> Result<String, AppError> {
    let hex_str = decimal_to_hex(&hex_req.dec_value)?;
    Ok(hex_str)
}

#[derive(Deserialize)]
struct Decify {
    hex_value: String,
}

async fn hex_to_dec(Json(dec_rec): Json<Decify>) -> Result<String, AppError> {
    let dec_str = hex_to_decimal(&dec_rec.hex_value)?;
    Ok(dec_str)
}

fn decimal_to_hex(value: &str) -> Result<String, AppError> {
    let value = value.parse::<i64>()?;
    let value = format!("{:X}", value);
    Ok(value)
}

fn hex_to_decimal(value: &str) -> Result<String, AppError> {
    let value = i64::from_str_radix(value, 16)?;
    Ok(value.to_string())
}

async fn api_fallback() -> (StatusCode, Json<JsonError>) {
    (
        StatusCode::NOT_FOUND,
        Json(JsonError {
            error: String::from("notFound"),
        }),
    )
}

fn api_router() -> Router {
    Router::new()
        .route("/hexify", post(dec_to_hex))
        .route("/decify", post(hex_to_dec))
        .fallback(api_fallback)
}

fn html_router() -> Router {
    Router::new().route("hexify", post(hex_result))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .nest("/api", api_router())
        .nest("/html/", html_router())
        .route("/", get(hello_world))
        .route("/index.html", get(hello_world))
        .route("/:name", get(hello_name))
        .nest_service("/static", ServeDir::new("static/"));

    Ok(router.into())
}
