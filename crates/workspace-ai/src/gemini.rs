use async_trait::async_trait;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::json;

use crate::error::AiError;
use crate::provider::AiProvider;

#[derive(Clone)]
pub struct GeminiProvider {
    api_key: String,
    model: String,
    client: Client,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            api_key,
            model: model.unwrap_or_else(|| "gemini-2.5-flash".to_string()),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
    async fn complete_text(
        &self,
        system_instruction: &str,
        user_prompt: &str,
    ) -> Result<String, AiError> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let body = json!({
            "system_instruction": {
                "parts": [{ "text": system_instruction }]
            },
            "contents": [{
                "parts": [{ "text": user_prompt }]
            }]
        });

        let resp = self.client.post(&url).json(&body).send().await?;
        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(AiError::Provider(format!("Gemini API error: {}", err_text)));
        }

        let resp_json: serde_json::Value = resp.json().await?;
        let text = resp_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        Ok(text)
    }

    async fn generate_structured<T: DeserializeOwned + Send>(
        &self,
        system_instruction: &str,
        user_prompt: &str,
    ) -> Result<T, AiError> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let body = json!({
            "system_instruction": {
                "parts": [{ "text": system_instruction }]
            },
            "contents": [{
                "parts": [{ "text": user_prompt }]
            }],
            "generationConfig": {
                "response_mime_type": "application/json"
            }
        });

        let resp = self.client.post(&url).json(&body).send().await?;
        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(AiError::Provider(format!("Gemini API error: {}", err_text)));
        }

        let resp_json: serde_json::Value = resp.json().await?;
        let text = resp_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or_default();

        let parsed: T = serde_json::from_str(text)?;
        Ok(parsed)
    }
}
