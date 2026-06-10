//! Model profiles, chains, and registry for GOAT's model routing system.
//!
//! A **model profile** is a named configuration (e.g. `"balanced"`, `"coding"`)
//! that specifies an ordered list of `provider:model` entries to try.  GOAT
//! works through the list (the **fallback chain**) until one succeeds.
//!
//! # Config file example
//!
//! ```toml
//! [profiles]
//! default = "balanced"
//!
//! [profiles.balanced]
//! chain = ["openai:gpt-4o-mini", "groq:llama-3.3-70b-versatile"]
//!
//! [profiles.coding]
//! chain = ["openai:gpt-4o", "groq:qwen-coder"]
//!
//! [profiles.cheap]
//! chain = ["groq:llama-3.1-8b-instant", "openai:gpt-4o-mini"]
//! ```
//!
//! If the config file has no `[profiles]` section, GOAT uses built-in defaults.
//!
//! # Built-in default profiles
//!
//! | Profile    | Primary              | Fallback                       |
//! |------------|----------------------|--------------------------------|
//! | balanced   | openai:gpt-4o-mini   | groq:llama-3.3-70b-versatile   |
//! | cheap      | groq:llama-3.1-8b-instant | openai:gpt-4o-mini        |
//! | powerful   | openai:gpt-4o        | groq:llama-3.3-70b-versatile   |
//! | coding     | openai:gpt-4o        | groq:qwen-qw3-32b              |
//! | reasoning  | openai:o1-mini       | groq:llama-3.3-70b-versatile   |
//! | local      | ollama:llama3        | (none — planned, not working)  |

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Model entry ───────────────────────────────────────────────────────────────

/// A single `provider:model` entry in a fallback chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelEntry {
    pub provider: String,
    pub model: String,
}

impl ModelEntry {
    /// Parse `"openai:gpt-4o-mini"` → `ModelEntry`.
    pub fn parse(s: &str) -> Option<Self> {
        let (provider, model) = s.split_once(':')?;
        let provider = provider.trim().to_string();
        let model = model.trim().to_string();
        if provider.is_empty() || model.is_empty() {
            return None;
        }
        Some(ModelEntry { provider, model })
    }

    /// Format as `"provider:model"` string.
    pub fn display(&self) -> String {
        format!("{}:{}", self.provider, self.model)
    }
}

// ── Model chain ───────────────────────────────────────────────────────────────

/// An ordered fallback chain of `provider:model` entries.
#[derive(Debug, Clone)]
pub struct ModelChain {
    pub entries: Vec<ModelEntry>,
}

impl ModelChain {
    pub fn from_strings(strings: &[String]) -> Self {
        let entries = strings
            .iter()
            .filter_map(|s| ModelEntry::parse(s))
            .collect();
        ModelChain { entries }
    }

    /// Returns the display string for the primary entry, or "none" if empty.
    pub fn primary_display(&self) -> String {
        self.entries
            .first()
            .map(|e| e.display())
            .unwrap_or_else(|| "none".to_string())
    }

    /// Returns the display string for the fallback entries (all after first).
    pub fn fallback_display(&self) -> String {
        if self.entries.len() <= 1 {
            return "none".to_string();
        }
        self.entries[1..]
            .iter()
            .map(|e| e.display())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

// ── Profile config (from TOML) ────────────────────────────────────────────────

/// A model profile as read from the TOML config file.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProfilesConfig {
    /// Name of the default profile to use (e.g. `"balanced"`).
    #[serde(default = "default_profile_name")]
    pub default: String,

    #[serde(default = "default_mode_name")]
    pub default_mode: String,

    #[serde(default = "default_mode_name")]
    pub last_mode: String,

    #[serde(default = "default_true")]
    pub auto_suggest_profile: bool,

    #[serde(default = "default_true")]
    pub project_profile_enabled: bool,

    #[serde(default)]
    pub require_approval_for_profile_changes: bool,

    /// Per-profile configuration keyed by profile name.
    #[serde(flatten)]
    pub profiles: HashMap<String, ProfileEntry>,
}

fn default_profile_name() -> String {
    "balanced".to_string()
}

fn default_mode_name() -> String {
    "coding-assistant".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for ProfilesConfig {
    fn default() -> Self {
        Self {
            default: default_profile_name(),
            default_mode: default_mode_name(),
            last_mode: default_mode_name(),
            auto_suggest_profile: true,
            project_profile_enabled: true,
            require_approval_for_profile_changes: false,
            profiles: HashMap::new(),
        }
    }
}

/// Config entry for a single named profile.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProfileEntry {
    /// Ordered list of `"provider:model"` strings to try.
    #[serde(default)]
    pub chain: Vec<String>,
}

// ── Profile registry ──────────────────────────────────────────────────────────

/// Runtime model profile registry.
///
/// Merges built-in defaults with user config.  User config takes precedence.
pub struct ProfileRegistry {
    pub default_profile: String,
    pub profiles: HashMap<String, ModelChain>,
}

impl ProfileRegistry {
    /// Build from user config.  Missing profiles fall back to built-in defaults.
    pub fn from_config(config: &ProfilesConfig) -> Self {
        let mut profiles = built_in_defaults();

        // User config overrides built-in defaults.
        for (name, entry) in &config.profiles {
            profiles.insert(name.clone(), ModelChain::from_strings(&entry.chain));
        }

        ProfileRegistry {
            default_profile: config.default.clone(),
            profiles,
        }
    }

    /// Build with only built-in defaults (no user config).
    pub fn with_defaults() -> Self {
        ProfileRegistry {
            default_profile: "balanced".to_string(),
            profiles: built_in_defaults(),
        }
    }

    /// Resolve the active profile by name.  Falls back to "balanced" if not found.
    pub fn resolve<'a>(&'a self, name: &'a str) -> (&'a str, &'a ModelChain) {
        if let Some(chain) = self.profiles.get(name) {
            return (name, chain);
        }
        // Fall back to default profile.
        if let Some(chain) = self.profiles.get(&self.default_profile) {
            return (self.default_profile.as_str(), chain);
        }
        // Fall back to balanced (should always exist).
        (
            "balanced",
            self.profiles
                .get("balanced")
                .expect("balanced profile must always exist"),
        )
    }

    /// Get the default profile chain.
    pub fn default_chain(&self) -> &ModelChain {
        let name = self.default_profile.as_str();
        self.resolve(name).1
    }

    /// All profile names sorted.
    pub fn profile_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.profiles.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }
}

// ── Built-in default profiles ─────────────────────────────────────────────────

/// Returns the built-in default profile map.
///
/// These are used when no `[profiles]` section exists in the config file.
/// Users can override any of these in their config.
fn built_in_defaults() -> HashMap<String, ModelChain> {
    let mut m = HashMap::new();

    // balanced — primary choice for most tasks
    m.insert(
        "balanced".to_string(),
        ModelChain::from_strings(&[
            "openai:gpt-4o-mini".to_string(),
            "groq:llama-3.3-70b-versatile".to_string(),
        ]),
    );

    // cheap — fastest/lowest cost
    m.insert(
        "cheap".to_string(),
        ModelChain::from_strings(&[
            "groq:llama-3.1-8b-instant".to_string(),
            "openai:gpt-4o-mini".to_string(),
        ]),
    );

    // powerful — highest quality output
    m.insert(
        "powerful".to_string(),
        ModelChain::from_strings(&[
            "openai:gpt-4o".to_string(),
            "groq:llama-3.3-70b-versatile".to_string(),
        ]),
    );

    // coding — best for code generation/refactoring
    m.insert(
        "coding".to_string(),
        ModelChain::from_strings(&["openai:gpt-4o".to_string(), "groq:qwen-qwq-32b".to_string()]),
    );

    // reasoning — best for multi-step logic
    m.insert(
        "reasoning".to_string(),
        ModelChain::from_strings(&[
            "openai:o1-mini".to_string(),
            "groq:llama-3.3-70b-versatile".to_string(),
        ]),
    );

    // local — planned, Ollama-backed (not yet working)
    m.insert(
        "local".to_string(),
        ModelChain::from_strings(&["ollama:llama3".to_string()]),
    );

    m
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_entry_parse_valid() {
        let e = ModelEntry::parse("openai:gpt-4o-mini").unwrap();
        assert_eq!(e.provider, "openai");
        assert_eq!(e.model, "gpt-4o-mini");
    }

    #[test]
    fn test_model_entry_parse_no_colon() {
        assert!(ModelEntry::parse("openai").is_none());
    }

    #[test]
    fn test_model_entry_parse_empty_model() {
        assert!(ModelEntry::parse("openai:").is_none());
    }

    #[test]
    fn test_model_entry_display() {
        let e = ModelEntry {
            provider: "groq".into(),
            model: "llama3".into(),
        };
        assert_eq!(e.display(), "groq:llama3");
    }

    #[test]
    fn test_chain_from_strings() {
        let chain = ModelChain::from_strings(&[
            "openai:gpt-4o-mini".to_string(),
            "groq:llama3".to_string(),
        ]);
        assert_eq!(chain.len(), 2);
        assert_eq!(chain.primary_display(), "openai:gpt-4o-mini");
        assert_eq!(chain.fallback_display(), "groq:llama3");
    }

    #[test]
    fn test_chain_single_entry_no_fallback() {
        let chain = ModelChain::from_strings(&["openai:gpt-4o".to_string()]);
        assert_eq!(chain.fallback_display(), "none");
    }

    #[test]
    fn test_chain_invalid_entries_skipped() {
        let chain =
            ModelChain::from_strings(&["badentry".to_string(), "openai:gpt-4o-mini".to_string()]);
        assert_eq!(chain.len(), 1);
        assert_eq!(chain.entries[0].provider, "openai");
    }

    #[test]
    fn test_registry_built_in_profiles_exist() {
        let registry = ProfileRegistry::with_defaults();
        for name in &[
            "balanced",
            "cheap",
            "powerful",
            "coding",
            "reasoning",
            "local",
        ] {
            assert!(
                registry.profiles.contains_key(*name),
                "missing built-in profile: {name}"
            );
        }
    }

    #[test]
    fn test_registry_default_is_balanced() {
        let registry = ProfileRegistry::with_defaults();
        assert_eq!(registry.default_profile, "balanced");
    }

    #[test]
    fn test_registry_resolve_missing_falls_back_to_default() {
        let registry = ProfileRegistry::with_defaults();
        let (name, _chain) = registry.resolve("nonexistent");
        assert_eq!(name, "balanced");
    }

    #[test]
    fn test_registry_resolve_known_profile() {
        let registry = ProfileRegistry::with_defaults();
        let (name, chain) = registry.resolve("coding");
        assert_eq!(name, "coding");
        assert!(!chain.is_empty());
    }

    #[test]
    fn test_registry_user_config_overrides_default() {
        let mut config = ProfilesConfig {
            default: "myprofile".to_string(),
            ..Default::default()
        };
        config.profiles.insert(
            "myprofile".to_string(),
            ProfileEntry {
                chain: vec!["groq:llama3".to_string()],
            },
        );
        let registry = ProfileRegistry::from_config(&config);
        let (name, chain) = registry.resolve("myprofile");
        assert_eq!(name, "myprofile");
        assert_eq!(chain.entries[0].provider, "groq");
    }
}
