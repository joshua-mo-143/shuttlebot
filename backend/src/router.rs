use axum::{
    extract::{FromRef, State},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::cookie::{Key, PrivateCookieJar};
use shuttle_persist::PersistInstance;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

use crate::database::{DBQueries, DashboardData};
use crate::oauth::github_callback;
use crate::Persist;
use octocrab::Octocrab;

#[derive(Clone)]
pub struct AppState {
    pub oauth_id: String,
    pub oauth_secret: String,
    pub key: Key,
    pub persist: PersistInstance,
    pub db: DBQueries,
}

// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

pub fn init_router(
    public: PathBuf,
    db: DBQueries,
    oauth_id: String,
    oauth_secret: String,
    persist: PersistInstance,
    crab: Octocrab,
) -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let state = AppState {
        db,
        oauth_id,
        oauth_secret,
        key: Key::generate(),
        persist,
    };

    let api_router = Router::new()
        .route("/issues", get(get_issues))
        .route("/dashboard", get(dashboard))
        .layer(middleware::from_fn_with_state(state.clone(), check_authed));

    Router::new()
        .nest("/api", api_router)
        .route("/health", get(health))
        .route("/github/callback", get(github_callback))
        .with_state(state)
        .nest_service(
            "/",
            ServeDir::new(&public).not_found_service(ServeFile::new(public.join("/index.html"))),
        )
        .layer(cors)
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "It works!".to_string())
}

async fn get_issues(State(state): State<AppState>) -> Result<impl IntoResponse, impl IntoResponse> {
    match state.db.clone().get_all_issues().await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

async fn dashboard(State(state): State<AppState>) -> Result<impl IntoResponse, impl IntoResponse> {
    let last_four_weeks_stats = match state.db.clone().get_last_four_weeks_stats().await {
        Ok(res) => res,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    };

    let issues_awaiting_response = match state.db.clone().get_issues_awaiting_response().await {
        Ok(res) => res,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    };

    let issues_opened_last_week = match state.db.get_issues_opened_last_7_days().await {
        Ok(res) => res,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    };

    let dashboard_data = DashboardData {
        last_four_weeks_stats,
        issues_awaiting_response,
        issues_opened_last_week,
    };

    Ok((StatusCode::OK, Json(dashboard_data)))
}

async fn check_authed<B>(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let Some(cookie) = jar.get("session_id").map(|cookie| cookie.value().to_owned()) else {
        return Err(StatusCode::FORBIDDEN)
    };

    if Persist::check_record_exists(state.persist, cookie).unwrap() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
