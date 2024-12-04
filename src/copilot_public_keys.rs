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
