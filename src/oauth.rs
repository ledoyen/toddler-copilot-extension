use crate::state::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::PrivateCookieJar;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, Scope};
use time::Duration;
use tracing::error;

const GH_STATE_COOKIE: &str = "gh_state";
const GH_STATE_COOKIE_DURATION: Duration = Duration::minutes(10);

#[allow(clippy::unused_async)]
pub async fn pre_auth(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
) -> Result<(PrivateCookieJar, Redirect), StatusCode> {
    let (authorize_url, csrf_state) = &state
        .oauth_gh_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    Ok((
        jar.add(
            Cookie::build((GH_STATE_COOKIE, csrf_state.secret().clone()))
                .path("/")
                .max_age(GH_STATE_COOKIE_DURATION)
                .same_site(SameSite::Lax),
        ),
        Redirect::to(authorize_url.as_ref()),
    ))
}

#[derive(Debug, serde::Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn post_auth(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState>,
    mut jar: PrivateCookieJar,
) -> (StatusCode, PrivateCookieJar, String) {
    let state_check = check_state(&query, jar);
    jar = state_check.0;
    if state_check.1.is_err() {
        return (
            StatusCode::BAD_REQUEST,
            jar,
            "state cookie missing or invalid".to_string(),
        );
    }

    let token_res = state
        .oauth_gh_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await;

    if let Err(err) = token_res {
        error!(error = ?err, "[http] post_auth");
        (
            StatusCode::BAD_REQUEST,
            jar,
            format!("error exchanging code for token {err:#?}"),
        )
    } else {
        (
            StatusCode::OK,
            jar,
            "All done! Please return to the app".to_string(),
        )
    }
}

fn check_state(query: &AuthRequest, jar: PrivateCookieJar) -> (PrivateCookieJar, Result<(), ()>) {
    let state_token = CsrfToken::new(query.state.clone());
    let stored_secret: Option<String> = jar
        .get(GH_STATE_COOKIE)
        .map(|cookie| cookie.value().to_owned());

    let jar = jar.remove(Cookie::from(GH_STATE_COOKIE));

    if stored_secret
        .as_ref()
        .is_some_and(|ss| ss.ne(state_token.secret()))
    {
        tracing::warn!(
            "Invalid state, expected:{:?}, got:{}",
            stored_secret,
            state_token.secret()
        );
        (jar, Err(()))
    } else {
        if stored_secret.is_none() {
            tracing::warn!(
                "Missing state from cookies, not able to confirm the one sent by GitHub"
            );
        }
        (jar, Ok(()))
    }
}
