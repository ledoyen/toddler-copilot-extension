use axum::http::HeaderMap;
use tracing::info;

pub async fn chat_completion(headers: HeaderMap, body: String) -> () {
    info!("{headers:#?}");
    info!("{body:#?}");
    ()
}
