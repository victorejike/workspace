use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::error::AiError;
use crate::provider::AiProvider;

/// Deterministic offline provider for local-first testing and zero-cloud environments.
#[derive(Clone)]
pub struct OfflineHeuristicProvider;

impl OfflineHeuristicProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OfflineHeuristicProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AiProvider for OfflineHeuristicProvider {
    async fn complete_text(
        &self,
        _system_instruction: &str,
        user_prompt: &str,
    ) -> Result<String, AiError> {
        let topic = if user_prompt.to_lowercase().contains("rust") {
            "Rust memory safety, ownership, and concurrency"
        } else if user_prompt.to_lowercase().contains("python") {
            "Python data structures, decorators, and generators"
        } else {
            "engineering fundamentals"
        };

        Ok(format!(
            "[WORKSPACE Offline Mentor]\n\nRegarding your inquiry on {}:\n\nIn accordance with your learning context, remember that true understanding comes from writing practical code, not merely reading explanations.\n\nTake this step next: Build a small, isolated example and verify its behavior before continuing.",
            topic
        ))
    }

    async fn generate_structured<T: DeserializeOwned + Send>(
        &self,
        _system_instruction: &str,
        user_prompt: &str,
    ) -> Result<T, AiError> {
        // If requesting curriculum decomposition:
        let dummy_json = if user_prompt.to_lowercase().contains("ai engineer") {
            r#"{
                "title": "AI Engineering Roadmap",
                "modules": [
                    {
                        "title": "Foundational Python & Mathematics",
                        "topics": [
                            {"title": "Linear Algebra & Matrices", "estimated_minutes": 120},
                            {"title": "NumPy & Tensor Operations", "estimated_minutes": 90}
                        ]
                    },
                    {
                        "title": "Machine Learning Core",
                        "topics": [
                            {"title": "Gradient Descent & Loss Functions", "estimated_minutes": 120},
                            {"title": "Model Evaluation & Metrics", "estimated_minutes": 90}
                        ]
                    }
                ]
            }"#
        } else {
            r#"{
                "title": "Custom Engineering Track",
                "modules": [
                    {
                        "title": "Core Foundations",
                        "topics": [
                            {"title": "Architecture & Data Flow", "estimated_minutes": 60},
                            {"title": "Testing & Verification", "estimated_minutes": 60}
                        ]
                    }
                ]
            }"#
        };

        serde_json::from_str::<T>(dummy_json).map_err(|e| {
            AiError::Validation(format!(
                "Failed to deserialize offline structured response: {}",
                e
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct CurriculumResponse {
        title: String,
    }

    #[tokio::test]
    async fn test_offline_provider_completion() {
        let provider = OfflineHeuristicProvider::new();
        let res = provider
            .complete_text("You are an AI mentor", "Explain Rust borrowing")
            .await
            .expect("Failed completion");
        assert!(res.contains("Rust memory safety"));
    }

    #[tokio::test]
    async fn test_offline_structured_output() {
        let provider = OfflineHeuristicProvider::new();
        let curr: CurriculumResponse = provider
            .generate_structured("Curriculum prompt", "I want to become an AI Engineer")
            .await
            .expect("Failed structured output");
        assert_eq!(curr.title, "AI Engineering Roadmap");
    }
}
