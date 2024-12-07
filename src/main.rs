use axum::routing::post;
use axum::{routing::get, Router};
use shuttle_runtime::SecretStore;
use toddler_copilot_extension::agent::chat_completion;
use toddler_copilot_extension::config::Config;
use toddler_copilot_extension::oauth::{post_auth, pre_auth};
use toddler_copilot_extension::state::AppState;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    toddler_copilot_extension::tracing::setup()?;
    let config = Config::try_from(secret_store)?;
    let state = AppState::new(config).await?;

    let router = Router::new()
        .route("/auth/authorization", get(pre_auth))
        .route("/auth/callback", get(post_auth))
        .route("/agent", post(chat_completion))
        .with_state(state);

    Ok(router.into())
}
