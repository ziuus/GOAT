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
