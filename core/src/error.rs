#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("serialization: {0}")]
    Serialization(serde_json::Error),
}
