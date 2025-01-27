#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    PostHogCore(posthog_core::error::Error),
    #[error("send request: {0}")]
    SendRequest(reqwest::Error),
    #[error("response status: {0}")]
    ResponseStatus(reqwest::Error),
    #[error("decode response: {0}")]
    DecodeResponse(reqwest::Error),
}
