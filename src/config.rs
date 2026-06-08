use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub keys: Keys,
    #[serde(default)]
    pub mcp_servers: HashMap<String, McpServerConfig>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Keys {
    pub openai_api_key: Option<String>,
    pub groq_api_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct McpServerConfig {
    pub command: String,
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl Config {
    pub fn get_fallback_api_key() -> Option<(String, String)> {
        let mut path = dirs::home_dir()?;
        path.push(".config");
        path.push("opencode");
        path.push("opencode.json");

        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(key) = json
                    .pointer("/provider/freellmapi/options/apiKey")
                    .and_then(|v| v.as_str())
                {
                    if !key.is_empty() && !key.starts_with("{env:") {
                        let url = json
                            .pointer("/provider/freellmapi/options/baseURL")
                            .and_then(|v| v.as_str())
                            .unwrap_or("http://localhost:5999/v1");
                        return Some((key.to_string(), url.to_string()));
                    }
                }
                if let Some(key) = json
                    .pointer("/provider/omnichat/options/apiKey")
                    .and_then(|v| v.as_str())
                {
                    if !key.is_empty() && key != "***" && !key.starts_with("{env:") {
                        let url = json
                            .pointer("/provider/omnichat/options/baseURL")
                            .and_then(|v| v.as_str())
                            .unwrap_or("https://api.openai.com/v1");
                        return Some((key.to_string(), url.to_string()));
                    }
                }
            }
        }
        None
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        config_path.push(".config");
        config_path.push("goat");
        config_path.push("goat.toml");

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let default_config = Config::default();

            if let Some(parent) = config_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(content) = toml::to_string_pretty(&default_config) {
                let _ = fs::write(config_path, content);
            }

            Ok(default_config)
        }
    }
}
