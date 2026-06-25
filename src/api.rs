use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::env::Config;

#[derive(Serialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

/// Call the LLM with a system prompt and user content, returning the assistant's raw text.
pub async fn ask(config: &Config, system: &str, user: &str) -> Result<String> {
    let client = Client::new();

    let url = config
        .endpoint
        .trim_end_matches('/')
        .to_string()
        + "/chat/completions";

    let request_body = ChatRequest {
        model: config.model.clone(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user.to_string(),
            },
        ],
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .with_context(|| format!("failed to send request to {}", url))?;

    let status = response.status();
    let body_text = response
        .text()
        .await
        .context("failed to read response body")?;

    if !status.is_success() {
        anyhow::bail!("API returned HTTP {}: {}", status, body_text);
    }

    let chat_response: ChatResponse = serde_json::from_str(&body_text)
        .with_context(|| format!("failed to parse API response: {}", body_text))?;

    chat_response
        .choices
        .into_iter()
        .next()
        .map(|choice| choice.message.content)
        .context("API returned no choices")
}
