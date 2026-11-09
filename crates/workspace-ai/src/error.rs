use thiserror::Error;

#[derive(Error, Debug)]
pub enum AiError {
    #[error("API request failed: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Structured output schema validation failed: {0}")]
    Validation(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Missing API credentials for provider: {0}")]
    MissingCredentials(String),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
