use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::Duration;

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

pub async fn github_callback(
    State(state): State<AppState>,
    callback_code: Query<GithubToken>,
    jar: PrivateCookieJar,
) -> impl IntoResponse {
    let code = Callback {
        code: callback_code.code.clone(),
    };

    let meme = Client::new();

    let hehe = meme
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

    let json_response = hehe.json::<GithubCallback>().await.unwrap();

    println!("{}", json_response.access_token);

    let cookie = Cookie::build("foo", json_response.access_token)
        .secure(true)
        .max_age(Duration::DAY)
        .finish();

    (jar.add(cookie.clone()), Redirect::permanent("/"))
}
