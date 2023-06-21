use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::cookie::Key;
use serde::Serialize;
use shuttle_persist::PersistInstance;
use sqlx::PgPool;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

use crate::oauth::github_callback;

#[derive(Serialize, sqlx::FromRow)]
pub struct Issue {
    #[serde(rename(serialize = "originalPoster"))]
    original_poster: Option<String>,
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
    creation_date: String,
}

#[derive(Serialize)]
pub struct DashboardData {
    #[serde(rename(serialize = "lastFourWeeksStats"))]
    last_four_weeks_stats: Vec<LastFourWeeksStats>,
    #[serde(rename(serialize = "issuesAwaitingResponse"))]
    issues_awaiting_response: IssuesAwaitingResponse,
    #[serde(rename(serialize = "issuesOpenedLastWeek"))]
    issues_opened_last_week: Vec<IssuesOpenedLastWeek>,
}

#[derive(Serialize, sqlx::FromRow)]
struct LastFourWeeksStats {
    #[serde(rename(serialize = "dateRange"))]
    date_range: String,
    #[serde(rename(serialize = "totalIssues"))]
    total_issues: i64,
    #[serde(rename(serialize = "totalElevatedIssues"))]
    total_elevated_issues: i64,
    #[serde(rename(serialize = "totalResolvedIssues"))]
    total_resolved_issues: i64,
    #[serde(rename(serialize = "averageResponseTime"))]
    average_response_time: String,
    #[serde(rename(serialize = "bestSolver"))]
    best_solver: Option<String>,
    #[serde(rename(serialize = "bestFirstResponder"))]
    best_first_responder: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
struct IssuesOpenedLastWeek {
    day: String,
    #[serde(rename(serialize = "totalIssuesPerDay"))]
    total_issues_per_day: i64,
}

#[derive(Serialize, sqlx::FromRow)]
struct IssuesAwaitingResponse {
    #[serde(rename(serialize = "unansweredThreads"))]
    unanswered_threads: i64,
    #[serde(rename(serialize = "unresolvedIssues"))]
    unresolved_issues: i64,
    #[serde(rename(serialize = "unresolvedGithubIssues"))]
    unresolved_github_issues: i64,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub oauth_id: String,
    pub oauth_secret: String,
    pub key: Key,
    pub persist: PersistInstance,
}

// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

pub fn init_router(
    public: PathBuf,
    pool: PgPool,
    oauth_id: String,
    oauth_secret: String,
    persist: PersistInstance,
) -> Router {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let state = AppState {
        pool,
        oauth_id,
        oauth_secret,
        key: Key::generate(),
        persist,
    };

    Router::new()
        .route("/health", get(health))
        .route("/api/issues", get(get_issues))
        .route("/api/dashboard", get(dashboard))
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

async fn get_issues(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Issue>>), impl IntoResponse> {
    match sqlx::query_as::<_, Issue>(
        "SELECT 
        OriginalPoster as original_poster,
        DiscordThreadLink as discord_thread_link,
        SevCat as severity, 
        FirstResponseUser as first_responder,
        GithubLink as github_link,
        ResolverUser as resolved_by,
        CAST(DATE(created) as varchar) as creation_date
        from issues
        ",
    )
    .fetch_all(&state.pool)
    .await
    {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn dashboard(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<DashboardData>), impl IntoResponse> {
    // Data we want:
    // Count of how many tickets were opened DONE
    // Count of how many tickets were elevated to Github Issues DONE
    // Count of how many tickets were resolved DONE
    // Average first response time DONE
    // Who resolved the most tickets DONE
    // Who responded first to most tickets DONE
    // On a weekly basis

    let Ok(last_four_weeks_stats) = sqlx::query_as::<_, LastFourWeeksStats>("SELECT
        CONCAT(to_date(concat(DATE_PART('year', date(created)), DATE_PART('week', date(created))), 'iyyyiw'),' - ',to_date(concat('2023', DATE_PART('week', date(created))), 'yyyyww') + 6) AS date_range,
        COUNT(*) as total_issues,
        (SELECT COUNT(*) FROM issues WHERE githubLink IS NOT NULL) as total_elevated_issues,
        (SELECT COUNT(*) FROM issues WHERE resolved = TRUE) as total_resolved_issues,
        CAST((SELECT date_trunc('second', AVG(firstresponsetimedate - created)) FROM issues) as varchar) as average_response_time,
        (SELECT COUNT(ResolverUser) FROM issues WHERE resolved = True group by ResolverUser order by ResolverUser desc limit 1) as best_solver,
        (SELECT COUNT(FirstResponseUser) FROM issues WHERE resolved = True group by FirstResponseUser order by FirstResponseUser desc limit 1) as best_first_responder
        FROM issues
        GROUP BY date_range 
        ORDER BY date_range DESC
        LIMIT 4
        ")
        .fetch_all(&state.pool)
        .await else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while getting the last 4 weeks' stats.".to_string()))
    };

    let Ok(issues_awaiting_response) = sqlx::query_as::<_, IssuesAwaitingResponse>("SELECT
        (SELECT COUNT(*) FROM issues WHERE FirstResponseUser IS NULL) as unanswered_threads,
        (SELECT COUNT(*) FROM issues WHERE Resolved = FALSE) as unresolved_issues,
        (SELECT COUNT(*) FROM issues WHERE GithubLink IS NOT NULL and Resolved = FALSE) as unresolved_github_issues
        FROM issues
        ")
        .fetch_one(&state.pool)
        .await else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "An error occurred while getting the number of issues awaiting response.".to_string()))
    };

    let Ok(issues_opened_last_week) = sqlx::query_as::<_, IssuesOpenedLastWeek>("with days as (
        select generate_series(
        date(current_timestamp) - 6,
        date(current_timestamp),
        '1 day'::interval
            ) as day
        )

        select
        CAST(date(days.day) as varchar) as day,
        count(issues.id) as total_issues_per_day
        from days
        left join issues on date(created) = days.day
        group by 1
        order by day desc")
        .fetch_all(&state.pool)
        .await else {return Err((StatusCode::INTERNAL_SERVER_ERROR, "An error occurred while getting the number of issues opened last week".to_string()))};

    let dashboard_data = DashboardData {
        last_four_weeks_stats,
        issues_awaiting_response,
        issues_opened_last_week,
    };

    Ok((StatusCode::OK, Json(dashboard_data)))
}
