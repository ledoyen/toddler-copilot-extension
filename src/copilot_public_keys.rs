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
        let sig = "MEYCIQCmf3PGvVxh4bRDuozwzXo2QS5+x1vXP3nWcnO+gr6BEQIhAKzXkLncbxXkn49JCzZJ0YkMPrExEChupbMK7QYrcykA";
        let body = r#"{"copilot_thread_id":"f61a1e05-67f5-4627-abdf-208dee860660","messages":[{"role":"user","content":"@mais-arreeeeeeeeteuuuu coucou","copilot_references":[{"type":"github.repository","data":{"type":"repository","id":898581080,"name":"toddler-copilot-extension","ownerLogin":"ledoyen","ownerType":"User","readmePath":"README.md","description":"","commitOID":"5a7d8142530f7ee820cb47d265f9f62db896e9bc","ref":"refs/heads/main","refInfo":{"name":"main","type":"branch"},"visibility":"public","languages":[{"name":"Rust","percent":94.5},{"name":"Just","percent":5.5}]},"id":"ledoyen/toddler-copilot-extension","is_implicit":false,"metadata":{"display_name":"ledoyen/toddler-copilot-extension","display_icon":"","display_url":""}}],"copilot_confirmations":null},{"role":"user","content":"Current Date and Time (UTC): 2024-12-09 18:48:35\nCurrent User's Login: ledoyen\n","name":"_session","copilot_references":[{"type":"github.current-url","data":{"url":"https://github.com/ledoyen/toddler-copilot-extension/actions"},"id":"https://github.com/ledoyen/toddler-copilot-extension/actions","is_implicit":true,"metadata":{"display_name":"https://github.com/ledoyen/toddler-copilot-extension/actions","display_icon":"","display_url":""}}],"copilot_confirmations":null},{"role":"user","content":"","copilot_references":[{"type":"github.repository","data":{"type":"repository","id":898581080,"name":"toddler-copilot-extension","ownerLogin":"ledoyen","ownerType":"User","readmePath":"README.md","description":"","commitOID":"2645908ddf744f57a8bff15f8b7dc40d28edc15f","ref":"refs/heads/main","refInfo":{"name":"main","type":"branch"},"visibility":"public","languages":[{"name":"Rust","percent":94.8},{"name":"Just","percent":5.2}]},"id":"ledoyen/toddler-copilot-extension","is_implicit":false,"metadata":{"display_name":"ledoyen/toddler-copilot-extension","display_icon":"","display_url":""}}],"copilot_confirmations":null},{"role":"user","content":"tuttut","copilot_references":[],"copilot_confirmations":[]}],"stop":null,"top_p":0,"temperature":0,"max_tokens":0,"presence_penalty":0,"frequency_penalty":0,"response_format":null,"copilot_skills":null,"agent":"mais-arreeeeeeeeteuuuu","tools":null,"functions":null,"model":""}"#;
        key.verify_from_str(sig, body)?;
        Ok(())
    }
}
