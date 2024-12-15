#[derive(serde::Deserialize, serde::Serialize, Eq, PartialEq, Debug)]
pub struct ChatRequest {
    pub copilot_thread_id: String,
    pub messages: Vec<ChatMessage>,
}

impl ChatRequest {
    pub fn parse(content: &str) -> anyhow::Result<ChatRequest> {
        let result = &mut serde_json::Deserializer::from_str(content);
        let result: Result<ChatRequest, _> = serde_path_to_error::deserialize(result);
        Ok(result?)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Eq, PartialEq, Debug)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    pub copilot_references: Vec<CopilotReference>,
}

#[derive(serde::Deserialize, serde::Serialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(serde::Serialize, Eq, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum CopilotReference {
    #[serde(rename = "github.repository")]
    GithubRepository(CopilotReferenceData<GithubRepository>),
    #[serde(rename = "client.file")]
    ClientFile(CopilotReferenceData<ClientFile>),
    #[serde(untagged)]
    Unknown(String),
}

impl<'de> serde::Deserialize<'de> for CopilotReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
        let _type = value.get("type").unwrap();
        match _type.as_str().unwrap() {
            "github.repository" => {
                return if let Ok(reference) = serde_json::from_value::<CopilotReferenceData<GithubRepository>>(value.clone()) {
                    Ok(CopilotReference::GithubRepository(reference))
                } else {
                    Ok(CopilotReference::Unknown(value.to_string()))
                }
            }
            "client.file" => {
                return if let Ok(reference) = serde_json::from_value::<CopilotReferenceData<ClientFile>>(value.clone()) {
                    Ok(CopilotReference::ClientFile(reference))
                } else {
                    Ok(CopilotReference::Unknown(value.to_string()))
                }
            }
            _ => Ok(CopilotReference::Unknown(value.to_string())),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Eq, PartialEq, Debug, Default)]
pub struct CopilotReferenceData<T> {
    //#[serde(rename = "type")]
    //_type: String,
    data: T,
    id: String,
    is_implicit: bool,
    metadata: CopilotReferenceMetadata,
}

#[derive(serde::Deserialize, serde::Serialize, Eq, PartialEq, Debug, Default)]
pub struct CopilotReferenceMetadata {
    pub display_name: String,
    pub display_icon: String,
    pub display_url: String,
}

#[derive(serde::Deserialize, serde::Serialize, Eq, PartialEq, Debug, Default)]
pub struct GithubRepository {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: u64,
    pub name: String,
    #[serde(rename = "ownerLogin")]
    pub owner_login: String,
    #[serde(rename = "ownerType")]
    pub owner_type: String,
    #[serde(rename = "readmePath")]
    pub readme_path: String,
    pub description: String,
    #[serde(rename = "commitOID")]
    pub commit_oid: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    #[serde(rename = "refInfo")]
    pub ref_info: GithubRefInfo,
    pub visibility: String,
    #[serde(default)]
    pub languages: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Eq, PartialEq, Debug, Default)]
pub struct GithubRefInfo {
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Eq, PartialEq, Debug, Default)]
pub struct ClientFile {
    pub content: String,
    pub language: String,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::messages::{ChatMessage, ChatRequest, ClientFile, CopilotReference, CopilotReferenceData, GithubRepository, Role};

    #[test]
    fn parse_chat_request_from_vs_code() -> anyhow::Result<()> {
        let payload = fs::read_to_string("samples/chat_request_from_vs_code.json")?;
        let req = ChatRequest::parse(&payload)?;
        assert_eq!(
            req,
            ChatRequest {
                copilot_thread_id: "ca746c32-a78c-4d06-92b1-af5c31dadfde".to_string(),
                messages: vec![
                    ChatMessage {
                        role: Role::User,
                        content: "help with this file".to_string(),
                        copilot_references: vec![
                            CopilotReference::GithubRepository(CopilotReferenceData {
                                data: GithubRepository {
                                    _type: "repository".to_string(),
                                    id: 675110970,
                                    name: "korekto-frontend".to_string(),
                                    owner_login: "korekto".to_string(),
                                    ..GithubRepository::default()
                                },
                                id: "korekto/korekto-frontend".to_string(),
                                ..CopilotReferenceData::default()
                            })
                        ],
                    }
                ],
            }
        );
        Ok(())
    }

    #[test]
    fn parse_chat_request_from_ij() -> anyhow::Result<()> {
        let payload = fs::read_to_string("samples/chat_request_from_ij.json")?;
        let req = ChatRequest::parse(&payload)?;
        assert_eq!(
            req,
            ChatRequest {
                copilot_thread_id: "".to_string(),
                messages: vec![
                    ChatMessage {
                        role: Role::User,
                        content: "coucou".to_string(),
                        copilot_references: vec![
                            CopilotReference::GithubRepository(CopilotReferenceData {
                                data: GithubRepository {
                                    _type: "repository".to_string(),
                                    id: 898581080,
                                    name: "toddler-copilot-extension".to_string(),
                                    owner_login: "ledoyen".to_string(),
                                    ..GithubRepository::default()
                                },
                                id: "ledoyen/toddler-copilot-extension".to_string(),
                                ..CopilotReferenceData::default()
                            }),
                            CopilotReference::ClientFile(CopilotReferenceData {
                                data: ClientFile {
                                    content: "#[derive(serde::Deserialize, Eq, PartialEq, Debug)]\npub struct [...truncated]".to_string(),
                                    language: "rust".to_string(),
                                },
                                id: "file:///c%3A/workspace/toddler-copilot-extension/src/messages.rs".to_string(),
                                ..CopilotReferenceData::default()
                            }),
                        ],
                    }
                ],
            }
        );
        Ok(())
    }
}
