use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::error::AiError;
use crate::gemini::GeminiProvider;
use crate::offline::OfflineHeuristicProvider;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningContext {
    pub career_goal: String,
    pub experience_level: String,
    pub active_curriculum: Option<String>,
    pub current_topic: Option<String>,
    pub known_knowledge_gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: String, // "system", "user", "assistant"
    pub content: String,
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Generates a plain text completion (for tutoring and explanations).
    async fn complete_text(
        &self,
        system_instruction: &str,
        user_prompt: &str,
    ) -> Result<String, AiError>;

    /// Generates a validated structured output matching the target JSON schema.
    async fn generate_structured<T: DeserializeOwned + Send>(
        &self,
        system_instruction: &str,
        user_prompt: &str,
    ) -> Result<T, AiError>;
}

/// Unified AI client enum supporting offline heuristics and external cloud providers.
#[derive(Clone)]
pub enum AiClient {
    Offline(OfflineHeuristicProvider),
    Gemini(GeminiProvider),
}

impl AiClient {
    pub fn new_offline() -> Self {
        Self::Offline(OfflineHeuristicProvider::new())
    }

    pub fn new_gemini(api_key: String, model: Option<String>) -> Self {
        Self::Gemini(GeminiProvider::new(api_key, model))
    }

    pub async fn complete_text(
        &self,
        system_instruction: &str,
        user_prompt: &str,
    ) -> Result<String, AiError> {
        match self {
            Self::Offline(p) => p.complete_text(system_instruction, user_prompt).await,
            Self::Gemini(p) => p.complete_text(system_instruction, user_prompt).await,
        }
    }

    pub async fn generate_structured<T: DeserializeOwned + Send>(
        &self,
        system_instruction: &str,
        user_prompt: &str,
    ) -> Result<T, AiError> {
        match self {
            Self::Offline(p) => p.generate_structured(system_instruction, user_prompt).await,
            Self::Gemini(p) => p.generate_structured(system_instruction, user_prompt).await,
        }
    }
}

impl Default for AiClient {
    fn default() -> Self {
        Self::new_offline()
    }
}
