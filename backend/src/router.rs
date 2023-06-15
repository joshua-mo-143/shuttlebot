use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Router, Json};
use tera::Tera;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Meme {
    discord_thread_id: i32,
    severity: i8,
    response_received: bool,
    github_link: String,
    response_time: String,
}

pub fn init_router(public: PathBuf) -> Router {
    let mut tera = Tera::new(
            &(public.to_str()
            .expect("failed to get static folder")
            .to_string() 
            + "/**/*"),
    ).expect("Parsing error while loading template folder");
        tera.autoescape_on(vec!["j2"]);
    
    Router::new()
        .route("/health", get(health))
        .layer(Extension(std::sync::Arc::new(tera)))
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "It works!".to_string())
}

async fn get_memes(State(state): State<AppState>) -> Result<impl IntoResponse, impl IntoResponse> {
    let memes = sqlx::query_as::<_, Meme>("SELECT 
        DiscordThreadId as discord_thread_id, 
        SevCat as severity, 
        case (when FirstResponseUser is not null then TRUE else FALSE end) as response_received 
        from issues
        ")
        .fetch_all(state.pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(memes))
}