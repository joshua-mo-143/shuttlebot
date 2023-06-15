use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router};
use tera::Tera;

pub fn init_router() -> Router {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    Router::new()
        .route("/health", get(health))
        .layer(Extension(std::sync::Arc::new(tera)))
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "It works!".to_string())
}
