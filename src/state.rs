use crate::config::Config;
use crate::copilot_public_keys::load_copilot_public_key;
use anyhow::Context;
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use ecdsa::VerifyingKey;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use p256::NistP256;

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct AppState {
    pub config: Config,
    pub copilot_public_key: VerifyingKey<NistP256>,
    pub oauth_gh_client: BasicClient,
    pub cookie_key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let copilot_public_key =
            load_copilot_public_key("https://api.github.com/meta/public_keys/copilot_api").await?;
        let oauth_gh_client = create_oauth_gh_client(
            &config.github_app_client_id,
            &config.github_app_client_secret,
            &config.base_url,
        )?;
        let cookie_key = Key::generate();
        Ok(Self {
            config,
            copilot_public_key,
            oauth_gh_client,
            cookie_key,
        })
    }
}

fn create_oauth_gh_client<T: Into<String>>(
    client_id: T,
    client_secret: T,
    base_url: &str,
) -> anyhow::Result<BasicClient> {
    let github_client_id = ClientId::new(client_id.into());
    let github_client_secret = ClientSecret::new(client_secret.into());
    let gh_auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .context("[config] Invalid authorization endpoint URL")?;
    let gh_token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .context("[config] Invalid token endpoint URL")?;

    let redirect_url = RedirectUrl::new(format!("{base_url}/auth/gh/authorized"))
        .with_context(|| format!("[config] Unparseable GH redirect URL: {base_url}"))?;

    Ok(BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        gh_auth_url,
        Some(gh_token_url),
    )
    .set_redirect_uri(redirect_url))
}
