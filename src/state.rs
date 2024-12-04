use crate::config::Config;
use crate::copilot_public_keys::CopilotPublicKeys;
use anyhow::{anyhow, Context};
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use ecdsa::VerifyingKey;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use p256::NistP256;
use reqwest::header::USER_AGENT;
use std::str::FromStr;

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

async fn load_copilot_public_key(url: &str) -> anyhow::Result<VerifyingKey<NistP256>> {
    let client = reqwest::Client::new();
    let keys: CopilotPublicKeys = client
        .get(url)
        .header(USER_AGENT, "My Rust Program 1.0")
        .send()
        .await?
        .json()
        .await?;
    let optional_raw_key = keys
        .public_keys
        .into_iter()
        .find(|k| k.is_current)
        .map(|k| k.key);

    optional_raw_key.map_or_else(
        || Err(anyhow!("No current public keys in: {url}")),
        |raw_key| parse_key(&raw_key),
    )
}

fn parse_key(raw_key: &str) -> anyhow::Result<VerifyingKey<NistP256>> {
    Ok(VerifyingKey::from_str(raw_key)?)
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

#[cfg(test)]
mod tests {
    use crate::state::load_copilot_public_key;

    #[tokio::test]
    async fn load_pub_key() -> anyhow::Result<()> {
        let key =
            load_copilot_public_key("https://api.github.com/meta/public_keys/copilot_api").await?;
        println!("{key:#?}");
        Ok(())
    }
}
