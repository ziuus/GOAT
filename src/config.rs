//! GOAT configuration loader.
//!
//! Config is loaded from `~/.config/goat/goat.toml` by default, or from a
//! path specified via the `--config` CLI flag.
//!
//! The config file is created with safe defaults if it does not exist.
//! On Unix, a warning is emitted (and stored in the startup log) if the
//! file has group- or world-readable permissions.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub keys: Keys,
    #[serde(default)]
    pub mcp_servers: HashMap<String, McpServerConfig>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Keys {
    pub openai_api_key: Option<String>,
    pub groq_api_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct McpServerConfig {
    pub command: String,
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

/// Outcome of loading the config file — carries both the loaded config and
/// any non-fatal warnings that should be shown to the user.
pub struct ConfigLoadResult {
    pub config: Config,
    /// Human-readable warnings to display at startup.
    pub warnings: Vec<String>,
}

impl Config {
    /// Load config from the default path `~/.config/goat/goat.toml`.
    ///
    /// If the file does not exist, a default config is written and returned.
    /// If the file exists but has unsafe permissions, a warning is added.
    pub fn load_from(path: &Path) -> Result<ConfigLoadResult> {
        let mut warnings = Vec::new();

        let config = if path.exists() {
            let content = fs::read_to_string(path)
                .with_context(|| format!("failed to read config file: {}", path.display()))?;
            toml::from_str::<Config>(&content)
                .with_context(|| format!("failed to parse config file: {}", path.display()))?
        } else {
            // Create the default config file.
            let default_config = Config::default();
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("failed to create config directory: {}", parent.display())
                })?;
            }
            let toml_str = toml::to_string_pretty(&default_config)
                .context("failed to serialize default config")?;
            // Write with a comment header.
            let content = format!(
                "# GOAT configuration file\n# \
                 See: https://github.com/ziuus/GOAT/blob/master/docs/GOAT_PRODUCT_SPEC.md\n\
                 #\n\
                 # SECURITY: chmod 600 {} to restrict access to this file.\n\n{}\n",
                path.display(),
                toml_str
            );
            fs::write(path, &content)
                .with_context(|| format!("failed to write default config: {}", path.display()))?;

            // Set safe permissions on newly created config.
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o600));
            }

            warnings.push(format!(
                "[CONFIG] Created default config: {} — add your API keys.",
                path.display()
            ));

            default_config
        };

        // Check permissions on existing config files.
        if path.exists() {
            if let Some(mode) = crate::paths::check_config_permissions(path) {
                warnings.push(format!(
                    "[SECURITY] Config file has unsafe permissions (mode {:o}): {}  — run: chmod 600 {}",
                    mode,
                    path.display(),
                    path.display()
                ));
            }
        }

        // Attempt to read OpenCode fallback key if no keys configured.
        let config = maybe_apply_fallback_key(config);

        Ok(ConfigLoadResult { config, warnings })
    }

    /// Convenience: load from a `PathBuf`.
    pub fn load_from_path(path: PathBuf) -> Result<ConfigLoadResult> {
        Self::load_from(&path)
    }

    /// Read the legacy OpenCode/freellmapi fallback API key if present.
    /// Returns `None` if the source file does not exist or contains no key.
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
}

/// If the config has no keys configured, try to read from the OpenCode
/// fallback source. Mutates and returns the config.
fn maybe_apply_fallback_key(mut config: Config) -> Config {
    if config.keys.openai_api_key.is_none() && config.keys.groq_api_key.is_none() {
        if let Some((key, _url)) = Config::get_fallback_api_key() {
            config.keys.openai_api_key = Some(key);
        }
    }
    config
}
