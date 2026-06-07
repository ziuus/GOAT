use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

pub struct LlmRouter {
    client: Client,
    openai_key: Option<String>,
    groq_key: Option<String>,
}

impl LlmRouter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            openai_key: std::env::var("OPENAI_API_KEY").ok(),
            groq_key: std::env::var("GROQ_API_KEY").ok(),
        }
    }

    pub async fn completion(
        &self,
        provider: &str,
        model: &str,
        messages: Vec<Message>,
    ) -> Result<String, Box<dyn Error>> {
        match provider {
            "openai" => self.openai_completion(model, messages).await,
            "groq" => self.groq_completion(model, messages).await,
            _ => Err(format!("Unsupported provider: {}", provider).into()),
        }
    }

    async fn openai_completion(&self, model: &str, messages: Vec<Message>) -> Result<String, Box<dyn Error>> {
        let key = self.openai_key.as_ref().ok_or("OPENAI_API_KEY not set")?;
        self.call_openai_compatible("https://api.openai.com/v1/chat/completions", key, model, messages).await
    }

    async fn groq_completion(&self, model: &str, messages: Vec<Message>) -> Result<String, Box<dyn Error>> {
        let key = self.groq_key.as_ref().ok_or("GROQ_API_KEY not set")?;
        self.call_openai_compatible("https://api.groq.com/openai/v1/chat/completions", key, model, messages).await
    }

    async fn call_openai_compatible(
        &self,
        url: &str,
        key: &str,
        model: &str,
        messages: Vec<Message>,
    ) -> Result<String, Box<dyn Error>> {
        let req_body = OpenAiRequest {
            model: model.to_string(),
            messages,
        };

        let response = self
            .client
            .post(url)
            .bearer_auth(key)
            .json(&req_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()).into());
        }

        let res_body: OpenAiResponse = response.json().await?;
        if let Some(choice) = res_body.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No choices returned".into())
        }
    }
}
