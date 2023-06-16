use axum::{http::StatusCode, response::IntoResponse, routing::get, Router, Json, extract::State};
use std::path::PathBuf;
use serde::Serialize;
use sqlx::PgPool;
use tower_http::services::{ServeFile, ServeDir};
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize, sqlx::FromRow)]
pub struct Meme {
    #[serde(rename(serialize = "originalPoster"))]
    original_poster: String,
    #[serde(rename(serialize = "discordThreadLink"))]
    discord_thread_link: String,
    severity: i16,
    #[serde(rename(serialize = "firstResponder"))]
    first_responder: Option<String>,
    #[serde(rename(serialize = "githubLink"))]
    github_link: Option<String>,
    #[serde(rename(serialize = "resolvedBy"))]
    resolved_by: Option<String>,
    #[serde(rename(serialize = "creationDate"))]
    creation_date: String
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

pub fn init_router(public: PathBuf, pool: PgPool) -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);
                    
    let state = AppState {pool};
    
    Router::new()
        .route("/health", get(health))
        .route("/api/issues", get(get_memes))
        .with_state(state)
        .nest_service(
            "/",
            ServeDir::new(&public).not_found_service(ServeFile::new(public.join("/index.html"))),
        ).layer(cors)
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "It works!".to_string())
}

async fn get_memes(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<Meme>>), impl IntoResponse> {
    match sqlx::query_as::<_, Meme>("SELECT 
        OriginalPoster as original_poster,
        DiscordThreadLink as discord_thread_link,
        SevCat as severity, 
        FirstResponseUser as first_responder,
        GithubLink as github_link,
        ResolverUser as resolved_by,
        CAST(DATE(created) as varchar) as creation_date
        from issues
        ")
        .fetch_all(&state.pool)
        .await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    }
}