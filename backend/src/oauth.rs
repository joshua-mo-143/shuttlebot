use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Json,
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use chrono::{DateTime, Days, NaiveDateTime, Utc};
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::persist::{Persist, UserSession};
use crate::router::AppState;

#[derive(Deserialize, Debug)]
pub struct GithubToken {
    code: String,
}

#[derive(Serialize, Deserialize)]
pub struct Callback {
    code: String,
}

#[derive(Deserialize, Serialize)]
pub struct GithubCallback {
    token_type: String,
    scope: String,
    access_token: String,
}

#[axum_macros::debug_handler]
pub async fn github_callback(
    State(state): State<AppState>,
    callback_code: Query<GithubToken>,
    jar: PrivateCookieJar,
) -> impl IntoResponse {
    let code = Callback {
        code: callback_code.code.clone(),
    };

    let ctx = Client::new();

    // paste callback code from JSON and send to access token URL to generate an access token
    let post = ctx
        .post("https://github.com/login/oauth/access_token")
        .basic_auth(&state.oauth_id, Some(&state.oauth_secret))
        .json(&code)
        .header(
            HeaderName::from_lowercase(b"accept").unwrap(),
            HeaderValue::from_bytes(b"application/json").unwrap(),
        )
        .send()
        .await
        .expect("Failed Github fetch request");

    let json_response = post.json::<GithubCallback>().await.unwrap();

    println!("{}", json_response.access_token);

    let post = ctx
        .post("https://api.github.com/user")
        .bearer_auth(json_response.access_token.to_string())
        .header(
            HeaderName::from_lowercase(b"accept").unwrap(),
            HeaderValue::from_bytes(b"application/json").unwrap(),
        )
        .send()
        .await
        .expect("Failed Github fetch request");

    let github_user = post.json::<GithubUser>().await.unwrap();

    let cookie = Cookie::build("foo", json_response.access_token.to_string())
        .secure(true)
        .max_age(Duration::DAY)
        .finish();

    let user_session = UserSession {
        name: github_user.name.to_string(),
        session_id: json_response.access_token,
        expires_at: DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
            Utc,
        ) + Days::new(1),
    };

    Persist::add_record(state.persist, user_session).unwrap();

    let user = GithubUserResponse {
        name: github_user.name,
    };

    (
        jar.add(cookie.clone()),
        Redirect::permanent("/"),
        Json(user),
    )
}

#[derive(Serialize)]
pub struct GithubUserResponse {
    name: String,
}

#[derive(Debug, Clone)]
pub struct Redirect {
    status_code: StatusCode,
    location: HeaderValue,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
struct GithubUser {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    site_admin: String,
    name: String,
    company: String,
    blog: String,
    location: String,
    email: String,
    hireable: bool,
    bio: String,
    twitter_username: String,
    public_repos: i32,
    public_gists: i32,
    followers: i32,
    following: i32,
    created_at: String,
    updated_at: String,
}
