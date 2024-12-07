use anyhow::{anyhow, Context};
use axum::http::header::USER_AGENT;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use ecdsa::der::Signature;
use ecdsa::VerifyingKey;
use p256::NistP256;
use signature::Verifier;
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

pub trait VerifyFromStr {
    fn verify_from_str(&self, sig: &str, content: &str) -> anyhow::Result<()>;
}

impl VerifyFromStr for VerifyingKey<NistP256> {
    fn verify_from_str(&self, sig: &str, content: &str) -> anyhow::Result<()> {
        let decoded_signature = BASE64_STANDARD
            .decode(sig)
            .context("Error while b64 decoding signature")?;
        let signature = Signature::<NistP256>::from_bytes(&decoded_signature)
            .context("Error while parsing signature")?;
        self.verify(content.as_bytes(), &signature)
            .context("Error while verifying signature of body")?;
        Ok(())
    }
}

fn parse_key(raw_key: &str) -> anyhow::Result<VerifyingKey<NistP256>> {
    Ok(VerifyingKey::from_str(raw_key)?)
}

#[cfg(test)]
mod tests {
    use crate::copilot_public_keys::{load_copilot_public_key, VerifyFromStr};

    #[tokio::test]
    async fn load_pub_key() -> anyhow::Result<()> {
        let key =
            load_copilot_public_key("https://api.github.com/meta/public_keys/copilot_api").await?;
        println!("{key:#?}");
        Ok(())
    }

    #[tokio::test]
    async fn verify_from_str() -> anyhow::Result<()> {
        let key =
            load_copilot_public_key("https://api.github.com/meta/public_keys/copilot_api").await?;
        let sig = "MEYCIQDR6a+9PhILipMzOT7h4dpDUzKcq1mKeh7/Gp+hT8M3nwIhAKzZRofIDKjuI/wF+Kk1/rQ5hSy8kDOBufXPgiHvBL5l";
        let body = "";
        key.verify_from_str(sig, body)?;
        Ok(())
    }
}
