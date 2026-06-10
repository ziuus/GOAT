//! GOAT configuration loader.
//!
//! Config is loaded from `~/.config/goat/goat.toml` by default, or from a
//! path specified via the `--config` CLI flag.
//!
//! The config file is created with safe defaults if it does not exist.
//! On Unix, a warning is emitted (and stored in the startup log) if the
//! file has group- or world-readable permissions.
//!
//! # Config file structure
//!
//! ```toml
//! # API keys (or use environment variables)
//! [keys]
//! openai_api_key = "sk-..."
//! groq_api_key   = "gsk_..."
//!
//! # LLM request settings
//! [llm]
//! max_retries             = 2
//! timeout_secs            = 60
//! fallback_on_rate_limit  = true
//! fallback_on_network     = true
//! fallback_on_server_error = true
//!
//! # Custom provider settings (optional — env vars preferred for keys)
//! [providers.openrouter]
//! enabled  = true
//! base_url = "https://openrouter.ai/api/v1"
//! # api_key_env = "OPENROUTER_API_KEY"  # override env var name
//!
//! [providers.ollama]
//! enabled  = true
//! base_url = "http://localhost:11434"
//!
//! # Model profiles
//! [profiles]
//! default = "balanced"
//!
//! [profiles.balanced]
//! chain = ["openai:gpt-4o-mini", "groq:llama-3.3-70b-versatile"]
//! ```

use crate::models::ProfilesConfig;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// ── Top-level Config ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub keys: Keys,
    #[serde(default)]
    pub mcp_servers: HashMap<String, McpServerConfig>,
    /// LLM request settings: retries, timeouts, fallback policy.
    #[serde(default)]
    pub llm: LlmConfig,
    /// Per-provider custom settings (OpenRouter, Ollama, etc.).
    #[serde(default)]
    pub providers: HashMap<String, ProviderCustomConfig>,
    /// Model profile configuration.  Optional — built-in defaults used if absent.
    #[serde(default)]
    pub profiles: ProfilesConfig,
    /// Memory injection config.
    #[serde(default)]
    pub memory: MemoryConfig,
    /// Skills system config.
    #[serde(default)]
    pub skills: SkillsConfig,
    /// Repo map config.
    #[serde(default)]
    pub repo_map: RepoMapConfig,
    /// Tools and permissions config.
    #[serde(default)]
    pub tools: ToolsConfig,
    /// External agent adapters config.
    #[serde(default)]
    pub external_agents: ExternalAgentsConfig,
    /// Checkpoint system config.
    #[serde(default)]
    pub checkpoint: CheckpointConfig,
    /// Hooks system config.
    #[serde(default)]
    pub hooks: HooksConfig,
    /// Scheduler config.
    #[serde(default)]
    pub scheduler: SchedulerConfig,
    /// Daemon config.
    #[serde(default)]
    pub daemon: DaemonConfig,
    /// Brain learning and memory galaxy config.
    #[serde(default)]
    pub learning: LearningConfig,
    /// Skill marketplace config.
    #[serde(default)]
    pub skill_marketplace: SkillMarketplaceConfig,
    /// Recipe marketplace and automation config.
    #[serde(default)]
    pub recipe_marketplace: RecipeConfig,
    /// Brain search and semantic index config.
    #[serde(default)]
    pub brain_index: BrainIndexConfig,
}

// ── Keys ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Keys {
    pub openai_api_key: Option<String>,
    pub groq_api_key: Option<String>,
    /// OpenRouter API key (alternative: `OPENROUTER_API_KEY` env var).
    pub openrouter_api_key: Option<String>,
}

// ── LLM settings ─────────────────────────────────────────────────────────────

/// LLM request settings.  All fields have sensible defaults.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LlmConfig {
    /// Maximum retry attempts per chain entry on transient errors.
    /// Range: 0–10.  Default: 2.
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,

    /// Request timeout in seconds.  Default: 60.
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,

    /// Whether to advance the fallback chain on HTTP 429 rate-limit errors.
    #[serde(default = "default_true")]
    pub fallback_on_rate_limit: bool,

    /// Whether to retry and advance on network/timeout errors.
    #[serde(default = "default_true")]
    pub fallback_on_network: bool,

    /// Whether to retry and advance on HTTP 5xx server errors.
    #[serde(default = "default_true")]
    pub fallback_on_server_error: bool,
}

impl LlmConfig {
    /// Validate the config.  Returns a list of warnings for invalid values.
    pub fn validate(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        if self.max_retries > 10 {
            warnings.push(format!(
                "[CONFIG] llm.max_retries = {} is very high (max recommended: 10) — using 10",
                self.max_retries
            ));
        }
        if self.timeout_secs == 0 {
            warnings
                .push("[CONFIG] llm.timeout_secs = 0 is invalid — using default (60)".to_string());
        }
        if self.timeout_secs > 600 {
            warnings.push(format!(
                "[CONFIG] llm.timeout_secs = {} is very long (max recommended: 600)",
                self.timeout_secs
            ));
        }
        warnings
    }

    /// Effective max_retries, clamped to safe range.
    pub fn effective_max_retries(&self) -> u32 {
        self.max_retries.min(10)
    }

    /// Effective timeout_secs, falling back to default if 0.
    pub fn effective_timeout_secs(&self) -> u64 {
        if self.timeout_secs == 0 {
            default_timeout_secs()
        } else {
            self.timeout_secs
        }
    }
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            max_retries: default_max_retries(),
            timeout_secs: default_timeout_secs(),
            fallback_on_rate_limit: true,
            fallback_on_network: true,
            fallback_on_server_error: true,
        }
    }
}

fn default_max_retries() -> u32 {
    2
}
fn default_timeout_secs() -> u64 {
    60
}
fn default_true() -> bool {
    true
}

// ── Memory settings ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MemoryConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub inject_user: bool,
    #[serde(default = "default_true")]
    pub inject_memory: bool,
    #[serde(default = "default_true")]
    pub inject_project: bool,
    #[serde(default = "default_max_user_chars")]
    pub max_user_chars: usize,
    #[serde(default = "default_max_memory_chars")]
    pub max_memory_chars: usize,
    #[serde(default = "default_max_project_chars")]
    pub max_project_chars: usize,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            inject_user: true,
            inject_memory: true,
            inject_project: true,
            max_user_chars: 1500,
            max_memory_chars: 2500,
            max_project_chars: 1500,
        }
    }
}

fn default_max_user_chars() -> usize {
    1500
}
fn default_max_memory_chars() -> usize {
    2500
}
fn default_max_project_chars() -> usize {
    1500
}

// ── Skills settings ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkillsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub inject_index: bool,
    #[serde(default = "default_max_index_chars")]
    pub max_index_chars: usize,
    #[serde(default = "default_max_skill_chars")]
    pub max_skill_chars: usize,
}

impl Default for SkillsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            inject_index: true,
            max_index_chars: 2000,
            max_skill_chars: 4000,
        }
    }
}

fn default_max_index_chars() -> usize {
    2000
}
fn default_max_skill_chars() -> usize {
    4000
}

// ── Tools settings ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub permissions: ToolPermissionsConfig,
}

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            permissions: ToolPermissionsConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ToolPermissionsConfig {
    #[serde(default = "default_ask")]
    pub shell: String,
    #[serde(default = "default_ask")]
    pub filesystem_write: String,
    #[serde(default = "default_allow")]
    pub filesystem_read: String,
    #[serde(default = "default_ask")]
    pub network: String,
    #[serde(default = "default_ask")]
    pub git: String,
    #[serde(default = "default_allow")]
    pub memory: String,
    #[serde(default = "default_allow")]
    pub skills: String,
    #[serde(default = "default_ask")]
    pub subagent: String,
}

impl Default for ToolPermissionsConfig {
    fn default() -> Self {
        Self {
            shell: "ask".to_string(),
            filesystem_write: "ask".to_string(),
            filesystem_read: "allow".to_string(),
            network: "ask".to_string(),
            git: "ask".to_string(),
            memory: "allow".to_string(),
            skills: "allow".to_string(),
            subagent: "ask".to_string(),
        }
    }
}

fn default_ask() -> String {
    "ask".to_string()
}

fn default_allow() -> String {
    "allow".to_string()
}

// ── Repo Map settings ─────────────────────────────────────────────────────────

/// Configuration for the repo map feature.
///
/// Example TOML:
/// ```toml
/// [repo_map]
/// enabled = true
/// inject = true
/// max_chars = 4000
/// include_symbols = true
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RepoMapConfig {
    /// Enable the repo map system.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Inject repo map into the system prompt context.
    #[serde(default = "default_true")]
    pub inject: bool,
    /// Maximum characters for the injected repo map context.
    #[serde(default = "default_repo_map_max_chars")]
    pub max_chars: usize,
    /// Include symbol names in the repo map (fn/struct/class etc.)
    #[serde(default = "default_true")]
    pub include_symbols: bool,
}

impl Default for RepoMapConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            inject: true,
            max_chars: 4000,
            include_symbols: true,
        }
    }
}

fn default_repo_map_max_chars() -> usize {
    4000
}

// ── Learning settings ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LearningConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_false")]
    pub auto_extract: bool,
    #[serde(default = "default_true")]
    pub require_review: bool,
    #[serde(default = "default_max_candidates_per_session")]
    pub max_candidates_per_session: usize,
    #[serde(default = "default_true")]
    pub store_project_memories: bool,
    #[serde(default = "default_false")]
    pub store_user_memories: bool,
    #[serde(default = "default_true")]
    pub store_workflow_memories: bool,
    #[serde(default = "default_false")]
    pub allow_llm_summarization: bool,
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_extract: false,
            require_review: true,
            max_candidates_per_session: 10,
            store_project_memories: true,
            store_user_memories: false,
            store_workflow_memories: true,
            allow_llm_summarization: false,
        }
    }
}

fn default_max_candidates_per_session() -> usize {
    10
}

// ── Custom Providers ──────────────────────────────────────────────────────────

/// Custom settings for a specific provider (e.g. OpenRouter, Ollama).
///
/// Example TOML:
/// ```toml
/// [providers.openrouter]
/// enabled  = true
/// base_url = "https://openrouter.ai/api/v1"
///
/// [providers.ollama]
/// enabled  = true
/// base_url = "http://localhost:11434"
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProviderCustomConfig {
    /// Whether this provider is enabled.  Default: true.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Base URL for the provider's API.
    /// For Ollama: "http://localhost:11434"
    /// For OpenRouter: "https://openrouter.ai/api/v1"
    pub base_url: Option<String>,

    /// Environment variable name containing the API key.
    /// If not set, uses the standard name for the provider.
    pub api_key_env: Option<String>,
}

impl Default for ProviderCustomConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            base_url: None,
            api_key_env: None,
        }
    }
}

impl Config {
    /// Return the effective base URL for a provider.
    pub fn provider_base_url(&self, provider: &str) -> Option<String> {
        self.providers
            .get(provider)
            .and_then(|p| p.base_url.clone())
    }

    /// Return the effective API key for a provider.
    ///
    /// Priority: config key field → custom env var → default env var.
    /// Never returns an empty string.
    pub fn provider_api_key(&self, provider: &str) -> Option<String> {
        match provider {
            "openai" => self
                .keys
                .openai_api_key
                .clone()
                .or_else(|| std::env::var("OPENAI_API_KEY").ok()),
            "groq" => self
                .keys
                .groq_api_key
                .clone()
                .or_else(|| std::env::var("GROQ_API_KEY").ok()),
            "openrouter" => {
                // Custom env var name takes precedence.
                let env_name = self
                    .providers
                    .get("openrouter")
                    .and_then(|p| p.api_key_env.as_deref())
                    .unwrap_or("OPENROUTER_API_KEY");
                self.keys
                    .openrouter_api_key
                    .clone()
                    .or_else(|| std::env::var(env_name).ok())
            }
            "ollama" => None, // Ollama is local — no API key needed.
            _ => {
                // Generic fallback: look for a custom api_key_env.
                let env_name = self
                    .providers
                    .get(provider)
                    .and_then(|p| p.api_key_env.as_deref())?;
                std::env::var(env_name).ok()
            }
        }
        .filter(|k| !k.is_empty())
    }

    /// Whether a provider is enabled per config.
    pub fn provider_enabled(&self, provider: &str) -> bool {
        self.providers
            .get(provider)
            .map(|p| p.enabled)
            .unwrap_or(true) // default: enabled if not mentioned
    }
}

// ── MCP server config ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct McpServerConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_transport")]
    pub transport: String,
    pub command: String,
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default = "default_risk")]
    pub risk: String,
}

fn default_transport() -> String {
    "stdio".to_string()
}
fn default_risk() -> String {
    "ask".to_string()
}

// ── ConfigLoadResult ──────────────────────────────────────────────────────────

/// Outcome of loading the config file — carries both the loaded config and
/// any non-fatal warnings that should be shown to the user.
pub struct ConfigLoadResult {
    pub config: Config,
    /// Human-readable warnings to display at startup.
    pub warnings: Vec<String>,
}

// ── Config loading ────────────────────────────────────────────────────────────

impl Config {
    /// Load config from `~/.config/goat/goat.toml` (or a custom path).
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

        // Validate LLM config.
        for w in config.llm.validate() {
            warnings.push(w);
        }

        // Attempt to read OpenCode fallback key if no keys configured.
        let mut config = maybe_apply_fallback_key(config);

        // Load external mcp.json or mcp.toml if present
        if let Some(parent) = path.parent() {
            let mcp_json = parent.join("mcp.json");
            let mcp_toml = parent.join("mcp.toml");

            #[derive(Deserialize)]
            #[allow(non_snake_case)]
            struct McpFile {
                #[serde(default)]
                mcpServers: HashMap<String, McpServerConfig>,
                #[serde(default)]
                servers: HashMap<String, McpServerConfig>,
            }

            if mcp_json.exists() {
                if let Ok(content) = fs::read_to_string(&mcp_json) {
                    if let Ok(parsed) = serde_json::from_str::<McpFile>(&content) {
                        for (k, v) in parsed.mcpServers {
                            config.mcp_servers.insert(k, v);
                        }
                        for (k, v) in parsed.servers {
                            config.mcp_servers.insert(k, v);
                        }
                    } else {
                        warnings.push(format!("[CONFIG] Failed to parse {}", mcp_json.display()));
                    }
                }
            } else if mcp_toml.exists() {
                if let Ok(content) = fs::read_to_string(&mcp_toml) {
                    if let Ok(parsed) = toml::from_str::<McpFile>(&content) {
                        for (k, v) in parsed.mcpServers {
                            config.mcp_servers.insert(k, v);
                        }
                        for (k, v) in parsed.servers {
                            config.mcp_servers.insert(k, v);
                        }
                    } else {
                        warnings.push(format!("[CONFIG] Failed to parse {}", mcp_toml.display()));
                    }
                }
            }
        }

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
    let has_any_key = config.keys.openai_api_key.is_some()
        || config.keys.groq_api_key.is_some()
        || config.keys.openrouter_api_key.is_some()
        || std::env::var("OPENAI_API_KEY").is_ok()
        || std::env::var("GROQ_API_KEY").is_ok()
        || std::env::var("OPENROUTER_API_KEY").is_ok();

    if !has_any_key {
        if let Some((key, _url)) = Config::get_fallback_api_key() {
            config.keys.openai_api_key = Some(key);
        }
    }
    config
}

// ── External Agents Config ───────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ExternalWorkspaceMode {
    DetectOnly,
    Readonly,
    IsolatedCopy,
    RealWorkspace,
}

impl std::fmt::Display for ExternalWorkspaceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DetectOnly => write!(f, "detect-only"),
            Self::Readonly => write!(f, "readonly"),
            Self::IsolatedCopy => write!(f, "isolated-copy"),
            Self::RealWorkspace => write!(f, "real-workspace"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExternalAgentsConfig {
    pub enabled: bool,
    pub allow_execution: bool,
    pub default_timeout_secs: u64,
    pub workspace_mode: ExternalWorkspaceMode,
    #[serde(default)]
    pub agents: HashMap<String, ExternalAgentAdapterConfig>,
}

impl Default for ExternalAgentsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            allow_execution: false,
            default_timeout_secs: 120,
            workspace_mode: ExternalWorkspaceMode::DetectOnly,
            agents: HashMap::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExternalAgentAdapterConfig {
    pub enabled: bool,
    pub command: String,
    pub allow_execution: bool,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_config_defaults() {
        let cfg = LlmConfig::default();
        assert_eq!(cfg.max_retries, 2);
        assert_eq!(cfg.timeout_secs, 60);
        assert!(cfg.fallback_on_rate_limit);
        assert!(cfg.fallback_on_network);
        assert!(cfg.fallback_on_server_error);
    }

    #[test]
    fn test_llm_config_validate_high_retries() {
        let cfg = LlmConfig {
            max_retries: 99,
            ..LlmConfig::default()
        };
        let warnings = cfg.validate();
        assert!(!warnings.is_empty());
        assert!(warnings[0].contains("max_retries"));
        assert_eq!(cfg.effective_max_retries(), 10);
    }

    #[test]
    fn test_llm_config_validate_zero_timeout() {
        let cfg = LlmConfig {
            timeout_secs: 0,
            ..LlmConfig::default()
        };
        let warnings = cfg.validate();
        assert!(!warnings.is_empty());
        assert_eq!(cfg.effective_timeout_secs(), 60);
    }

    #[test]
    fn test_llm_config_validate_clean() {
        let cfg = LlmConfig::default();
        assert!(cfg.validate().is_empty());
    }

    #[test]
    fn test_config_provider_enabled_default_true() {
        let config = Config::default();
        assert!(config.provider_enabled("openrouter")); // not in map → default true
    }

    #[test]
    fn test_config_provider_enabled_explicit_false() {
        let mut config = Config::default();
        config.providers.insert(
            "openrouter".to_string(),
            ProviderCustomConfig {
                enabled: false,
                base_url: None,
                api_key_env: None,
            },
        );
        assert!(!config.provider_enabled("openrouter"));
    }

    #[test]
    fn test_config_provider_base_url() {
        let mut config = Config::default();
        config.providers.insert(
            "ollama".to_string(),
            ProviderCustomConfig {
                enabled: true,
                base_url: Some("http://localhost:11434".to_string()),
                api_key_env: None,
            },
        );
        assert_eq!(
            config.provider_base_url("ollama"),
            Some("http://localhost:11434".to_string())
        );
        assert_eq!(config.provider_base_url("nonexistent"), None);
    }

    #[test]
    fn test_config_ollama_no_api_key() {
        let config = Config::default();
        // Ollama should always return None for api_key (local, no auth needed).
        // Only skip this assertion if OPENROUTER_API_KEY is somehow set in env.
        assert_eq!(config.provider_api_key("ollama"), None);
    }
}

// ── Checkpoint ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CheckpointConfig {
    pub enabled: bool,
    pub auto_before_patch: bool,
    pub max_checkpoints: usize,
}

impl Default for CheckpointConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_before_patch: true,
            max_checkpoints: 20,
        }
    }
}

// ── Hooks settings ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HooksConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub require_approval: bool,
    #[serde(default)]
    pub rules: Vec<HookRuleConfig>,
}

impl Default for HooksConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            require_approval: true,
            rules: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HookRuleConfig {
    pub name: String,
    pub event: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_hook_type")]
    pub r#type: String,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default = "default_risk_ask")]
    pub risk: String,
}

fn default_hook_type() -> String {
    "command".to_string()
}

fn default_risk_ask() -> String {
    "ask".to_string()
}

// ── Scheduler settings ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SchedulerConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub require_approval_for_actions: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            require_approval_for_actions: true,
        }
    }
}

// ── Daemon Settings ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DaemonConfig {
    #[serde(default = "default_false")]
    pub enabled: bool,
    #[serde(default = "default_daemon_host")]
    pub host: String,
    #[serde(default = "default_daemon_port")]
    pub port: u16,
    #[serde(default = "default_true")]
    pub auth_required: bool,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            host: default_daemon_host(),
            port: default_daemon_port(),
            auth_required: true,
        }
    }
}

fn default_daemon_host() -> String {
    "127.0.0.1".to_string()
}

fn default_daemon_port() -> u16 {
    47647
}
fn default_false() -> bool {
    false
}

// ── Skill Marketplace ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SkillMarketplaceConfig {
    #[serde(default = "default_false")]
    pub enabled: bool,
    #[serde(default = "default_marketplace_provider")]
    pub provider: String,
    #[serde(default = "default_marketplace_base_url")]
    pub base_url: String,
    #[serde(default = "default_auth_mode")]
    pub auth_mode: String,
    #[serde(default = "default_true")]
    pub cache_enabled: bool,
    #[serde(default = "default_cache_ttl_minutes")]
    pub cache_ttl_minutes: u64,
    #[serde(default = "default_true")]
    pub require_audit_before_install: bool,
    #[serde(default = "default_true")]
    pub require_approval_before_install: bool,
}

impl Default for SkillMarketplaceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: default_marketplace_provider(),
            base_url: default_marketplace_base_url(),
            auth_mode: default_auth_mode(),
            cache_enabled: true,
            cache_ttl_minutes: default_cache_ttl_minutes(),
            require_audit_before_install: true,
            require_approval_before_install: true,
        }
    }
}

fn default_marketplace_provider() -> String {
    "skills.sh".to_string()
}
fn default_marketplace_base_url() -> String {
    "https://api.skills.sh".to_string()
}
fn default_auth_mode() -> String {
    "vercel_oidc".to_string()
}
fn default_cache_ttl_minutes() -> u64 {
    1440
}

// ── Recipes Config ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecipeConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub allow_activation: bool,
    #[serde(default = "default_true")]
    pub require_approval_for_activation: bool,
    #[serde(default = "default_recipe_max_steps")]
    pub max_steps_per_run: u32,
    #[serde(default = "default_recipe_min_interval")]
    pub min_schedule_interval_minutes: u32,
    #[serde(default = "default_true")]
    pub dry_run_before_activation: bool,
}

impl Default for RecipeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allow_activation: true,
            require_approval_for_activation: true,
            max_steps_per_run: 20,
            min_schedule_interval_minutes: 15,
            dry_run_before_activation: true,
        }
    }
}

fn default_recipe_max_steps() -> u32 {
    20
}
fn default_recipe_min_interval() -> u32 {
    15
}

// ── Brain Index Config ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BrainIndexConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub auto_index: bool,
    #[serde(default = "default_true")]
    pub index_audit_logs: bool,
    #[serde(default = "default_true")]
    pub index_approvals: bool,
    #[serde(default = "default_true")]
    pub index_jobs: bool,
    #[serde(default = "default_true")]
    pub index_skills: bool,
    #[serde(default = "default_true")]
    pub index_recipes: bool,
    #[serde(default = "default_true")]
    pub index_studio_drafts: bool,
    #[serde(default = "default_false")]
    pub allow_semantic_embeddings: bool,
    #[serde(default = "default_none_str")]
    pub embedding_provider: String,
    #[serde(default = "default_max_document_chars")]
    pub max_document_chars: usize,
}

impl Default for BrainIndexConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_index: true,
            index_audit_logs: true,
            index_approvals: true,
            index_jobs: true,
            index_skills: true,
            index_recipes: true,
            index_studio_drafts: true,
            allow_semantic_embeddings: false,
            embedding_provider: "none".to_string(),
            max_document_chars: 12000,
        }
    }
}

fn default_none_str() -> String {
    "none".to_string()
}
fn default_max_document_chars() -> usize {
    12000
}
