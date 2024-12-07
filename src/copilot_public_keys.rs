use anyhow::anyhow;
use axum::http::header::USER_AGENT;
use ecdsa::VerifyingKey;
use p256::NistP256;
use std::str::FromStr;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CopilotPublicKeys {
    pub public_keys: Vec<CopilotPublicKey>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CopilotPublicKey {
    pub key_identifier: String,
    pub key: String,
    pub is_current: bool,
}

pub async fn load_copilot_public_key(url: &str) -> anyhow::Result<VerifyingKey<NistP256>> {
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

#[cfg(test)]
mod tests {
    use crate::copilot_public_keys::load_copilot_public_key;

    #[tokio::test]
    async fn load_pub_key() -> anyhow::Result<()> {
        let key =
            load_copilot_public_key("https://api.github.com/meta/public_keys/copilot_api").await?;
        println!("{key:#?}");
        Ok(())
    }
}
