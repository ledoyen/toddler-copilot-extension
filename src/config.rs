use shuttle_runtime::SecretStore;
use std::collections::HashMap;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub base_url: String,
    pub github_app_client_id: String,
    pub github_app_client_secret: String,
}

impl TryFrom<SecretStore> for Config {
    type Error = anyhow::Error;

    fn try_from(value: SecretStore) -> Result<Self, Self::Error> {
        let config = envy::from_iter::<_, Self>(value)?;
        Ok(config)
    }
}

impl TryFrom<HashMap<String, String>> for Config {
    type Error = anyhow::Error;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        let config = envy::from_iter::<_, Self>(value)?;
        Ok(config)
    }
}
