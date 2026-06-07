use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String, // typically "function"
    pub function: FunctionCall,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String, // JSON string of arguments
}

#[derive(Serialize, Clone)]
pub struct Tool {
    pub r#type: String, // typically "function"
    pub function: FunctionDeclaration,
}

#[derive(Serialize, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Serialize, Clone, Debug)]
pub struct Message {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>, // used when role is "tool"
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: MessageContent,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MessageContent {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub role: Option<String>,
}

pub struct LlmRouter {
    client: Client,
    openai_key: Option<String>,
    groq_key: Option<String>,
}

impl LlmRouter {
    pub fn new(openai_key: Option<String>, groq_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            openai_key: openai_key.or_else(|| std::env::var("OPENAI_API_KEY").ok()),
            groq_key: groq_key.or_else(|| std::env::var("GROQ_API_KEY").ok()),
        }
    }

    pub async fn completion(
        &self,
        provider: &str,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, Box<dyn Error>> {
        match provider {
            "openai" => self.openai_completion(model, messages, tools).await,
            "groq" => self.groq_completion(model, messages, tools).await,
            _ => Err(format!("Unsupported provider: {}", provider).into()),
        }
    }

    async fn openai_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, Box<dyn Error>> {
        let key = self.openai_key.as_ref().ok_or("OPENAI_API_KEY not set")?;
        self.call_openai_compatible("https://api.openai.com/v1/chat/completions", key, model, messages, tools).await
    }

    async fn groq_completion(
        &self,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, Box<dyn Error>> {
        let key = self.groq_key.as_ref().ok_or("GROQ_API_KEY not set")?;
        self.call_openai_compatible("https://api.groq.com/openai/v1/chat/completions", key, model, messages, tools).await
    }

    async fn call_openai_compatible(
        &self,
        url: &str,
        key: &str,
        model: &str,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
    ) -> Result<MessageContent, Box<dyn Error>> {
        let req_body = OpenAiRequest {
            model: model.to_string(),
            messages,
            tools,
        };

        let response = self
            .client
            .post(url)
            .bearer_auth(key)
            .json(&req_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let err_text = response.text().await?;
            return Err(format!("API error: {}", err_text).into());
        }

        let res_body: OpenAiResponse = response.json().await?;
        if let Some(choice) = res_body.choices.into_iter().next() {
            Ok(choice.message)
        } else {
            Err("No choices returned".into())
        }
    }
}
