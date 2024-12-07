use crate::state::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use base64::prelude::*;
use p256::ecdsa::Signature;
use signature::Verifier;
use tracing::{error, info, warn};

pub async fn chat_completion(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> Result<String, StatusCode> {
    let (_github_token, _integration_id) =
        extract_header_and_verify_signature(&state, &headers, &body)?;
    info!("{body:#?}");
    Ok("toto".to_string())
}

fn extract_header_and_verify_signature(
    state: &AppState,
    headers: &HeaderMap,
    body: &str,
) -> Result<(String, Option<String>), StatusCode> {
    if let Some(signature) = headers.get("github-public-key-signature") {
        let raw_sig = signature.to_str().map_err(|err| {
            error!(error = ?err, "[http] chat_completion: Unable to read header 'github-public-key-signature'");
            StatusCode::BAD_REQUEST
        })?;
        verify_signature(raw_sig, body, state).map_err(|err| {
            error!(error = ?err, "[http] chat_completion: Invalid signature: {err:#?}");
            StatusCode::BAD_REQUEST
        })?;
        if let Some(github_token_header) = headers.get("x-github-token") {
            let github_token = github_token_header.to_str().map_err(|err| {
                error!(error = ?err, "[http] chat_completion: Unable to read header 'x-github-token'");
                StatusCode::BAD_REQUEST
            })?;
            let integration_id = match headers.get("copilot-integration-id") {
                Some(v) => {
                    Some(v.to_str().map_err(|err| {
                        error!(error = ?err, "[http] chat_completion: Unable to read header 'copilot-integration-id'");
                        StatusCode::BAD_REQUEST
                    })?.to_string())
                }
                None => None
            };

            Ok((github_token.to_string(), integration_id))
        } else {
            warn!("[http] chat_completion: Missing header 'github-public-key-signature'");
            Err(StatusCode::BAD_REQUEST)
        }
    } else {
        warn!("[http] chat_completion: Missing header 'github-public-key-signature'");
        Err(StatusCode::BAD_REQUEST)
    }
}

fn verify_signature(sig: &str, body: &str, state: &AppState) -> anyhow::Result<()> {
    let decoded_signature = BASE64_STANDARD.decode(sig)?;
    let signature = Signature::from_slice(&decoded_signature)?;
    state
        .copilot_public_key
        .verify(body.as_bytes(), &signature)?;
    Ok(())
}
