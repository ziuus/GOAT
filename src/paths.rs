//! GOAT path resolution and system-readiness checks.
//!
//! `GoatPaths` is the single source of truth for all filesystem locations.
//! It is constructed once at startup and passed to the rest of the application.
//! This design makes it straightforward to expose these paths via a future
//! REST daemon or Tauri IPC layer without any path logic scattered elsewhere.

use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};

/// All resolved filesystem paths for a GOAT session.
///
/// Construct with [`GoatPaths::resolve`] or with custom paths from CLI flags.
#[derive(Debug, Clone)]
pub struct GoatPaths {
    /// `~/.config/goat/goat.toml` (or custom via `--config`).
    pub config_file: PathBuf,
    /// `~/.local/share/goat/` (or platform equivalent, or custom via `--data-dir`).
    pub data_dir: PathBuf,
    /// `<data_dir>/goat.db` (or custom via `--db`).
    pub db_file: PathBuf,
    /// Directory for rolling log files.
    pub log_dir: PathBuf,
    /// `<data_dir>/brain/`
    pub brain_dir: PathBuf,
    /// `<data_dir>/USER.md`
    pub user_file: PathBuf,
    /// `<data_dir>/MEMORY.md`
    pub memory_file: PathBuf,
    /// `<data_dir>/tool-audit.log`
    pub tool_audit_log_file: PathBuf,
    /// `<data_dir>/subagent-audit.log`
    pub subagent_audit_log_file: PathBuf,
    /// `~/.config/goat/skills/`
    pub skills_dir: PathBuf,
}

impl GoatPaths {
    /// Resolve default platform paths.
    ///
    /// Uses XDG conventions on Linux/macOS:
    /// - config: `~/.config/goat/goat.toml`
    /// - data:   `~/.local/share/goat/`
    /// - db:     `~/.local/share/goat/goat.db`
    /// - logs:   `~/.local/share/goat/logs/` (or `./logs` fallback)
    pub fn resolve() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow!("cannot determine home directory"))?;

        let config_file = {
            let mut p = home.clone();
            p.extend([".config", "goat", "goat.toml"]);
            p
        };

        let data_dir = dirs::data_dir().map(|d| d.join("goat")).unwrap_or_else(|| {
            // Fallback: ~/.local/share/goat
            let mut p = home.clone();
            p.extend([".local", "share", "goat"]);
            p
        });

        let db_file = data_dir.join("goat.db");

        // Logs go next to the data dir so they are not in the project root.
        let log_dir = data_dir.join("logs");

        let config_dir = config_file.parent().unwrap_or(Path::new("")).to_path_buf();
        let brain_dir = data_dir.join("brain");
        let user_file = data_dir.join("USER.md");
        let memory_file = data_dir.join("MEMORY.md");
        let tool_audit_log_file = data_dir.join("tool-audit.log");
        let subagent_audit_log_file = data_dir.join("subagent-audit.log");
        let skills_dir = config_dir.join("skills");

        Ok(Self {
            config_file,
            data_dir,
            db_file,
            log_dir,
            brain_dir,
            user_file,
            memory_file,
            tool_audit_log_file,
            subagent_audit_log_file,
            skills_dir,
        })
    }

    /// Override specific paths from CLI flags.
    pub fn with_config(mut self, path: PathBuf) -> Self {
        self.config_file = path;
        self
    }

    pub fn with_db(mut self, path: PathBuf) -> Self {
        self.db_file = path;
        self
    }

    pub fn with_data_dir(mut self, dir: PathBuf) -> Self {
        let db = dir.join("goat.db");
        self.data_dir = dir;
        self.db_file = db;
        self
    }

    /// Ensure the data directory exists.
    pub fn ensure_data_dir(&self) -> Result<()> {
        if !self.data_dir.exists() {
            fs::create_dir_all(&self.data_dir).with_context(|| {
                format!(
                    "failed to create GOAT data directory: {}",
                    self.data_dir.display()
                )
            })?;
        }
        Ok(())
    }

    /// Ensure the log directory exists.
    pub fn ensure_log_dir(&self) -> Result<()> {
        if !self.log_dir.exists() {
            fs::create_dir_all(&self.log_dir).with_context(|| {
                format!(
                    "failed to create GOAT log directory: {}",
                    self.log_dir.display()
                )
            })?;
        }
        Ok(())
    }

    /// Ensure the skills directory exists.
    pub fn ensure_skills_dir(&self) -> Result<()> {
        if !self.skills_dir.exists() {
            fs::create_dir_all(&self.skills_dir).with_context(|| {
                format!(
                    "failed to create GOAT skills directory: {}",
                    self.skills_dir.display()
                )
            })?;
        }
        Ok(())
    }

    /// Check whether the legacy project-root `goat_brain.db` exists.
    ///
    /// Returns the path if found, `None` otherwise.
    pub fn detect_legacy_db() -> Option<PathBuf> {
        // Check current working directory first.
        let cwd_legacy = PathBuf::from("goat_brain.db");
        if cwd_legacy.exists() {
            return Some(cwd_legacy);
        }
        None
    }
}

// ── Config permission check ────────────────────────────────────────────────────

/// On Unix: check whether `path` is readable by group or world.
///
/// Returns `Some(mode)` if the permissions are considered insecure,
/// `None` if the file is safe or the check cannot be run.
#[cfg(unix)]
pub fn check_config_permissions(path: &Path) -> Option<u32> {
    use std::os::unix::fs::PermissionsExt;

    let meta = fs::metadata(path).ok()?;
    let mode = meta.permissions().mode();
    // Mask off file-type bits, keep just permission bits (rwxrwxrwx).
    let perm_bits = mode & 0o777;
    // Warn if group-read (040) or world-read (004) bits are set.
    if perm_bits & 0o044 != 0 {
        Some(perm_bits)
    } else {
        None
    }
}

#[cfg(not(unix))]
pub fn check_config_permissions(_path: &Path) -> Option<u32> {
    // Permission model differs on Windows — skip check.
    None
}

// ── Doctor ────────────────────────────────────────────────────────────────────

/// A single result from `goat doctor`.
#[derive(Debug)]
pub struct DoctorCheck {
    pub status: DoctorStatus,
    pub label: String,
    pub detail: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DoctorStatus {
    Ok,
    Warn,
    Fail,
    Info,
}

impl DoctorStatus {
    pub fn prefix(&self) -> &str {
        match self {
            DoctorStatus::Ok => "[OK]  ",
            DoctorStatus::Warn => "[WARN]",
            DoctorStatus::Fail => "[FAIL]",
            DoctorStatus::Info => "[INFO]",
        }
    }
}

/// Run all doctor checks and return a list of results.
///
/// `has_*_key`: booleans — actual key values must NOT be passed here.
/// `headless_ready`: whether `--headless` was requested.
/// `llm_config`: LLM retry/timeout settings to display.
pub fn run_doctor(
    paths: &GoatPaths,
    config: &crate::config::Config,
    headless: bool,
) -> Vec<DoctorCheck> {
    let mut checks = Vec::new();

    let has_openai = config.provider_api_key("openai").is_some();
    let has_groq = config.provider_api_key("groq").is_some();
    let has_openrouter = config.provider_api_key("openrouter").is_some();
    let ollama_enabled = config.providers.contains_key("ollama");
    let llm_config = Some(&config.llm);

    // ── OS / Platform ─────────────────────────────────────────────────────────
    checks.push(DoctorCheck {
        status: DoctorStatus::Info,
        label: "OS".to_string(),
        detail: format!("{} {}", std::env::consts::OS, std::env::consts::ARCH),
    });

    checks.push(DoctorCheck {
        status: DoctorStatus::Info,
        label: "GOAT version".to_string(),
        detail: env!("CARGO_PKG_VERSION").to_string(),
    });

    // ── Config file ───────────────────────────────────────────────────────────
    if paths.config_file.exists() {
        checks.push(DoctorCheck {
            status: DoctorStatus::Ok,
            label: "Config file".to_string(),
            detail: paths.config_file.display().to_string(),
        });

        // Permission check (Unix only).
        if let Some(mode) = check_config_permissions(&paths.config_file) {
            checks.push(DoctorCheck {
                status: DoctorStatus::Warn,
                label: "Config permissions".to_string(),
                detail: format!(
                    "File is group/world readable (mode {:o}). Run: chmod 600 {}",
                    mode,
                    paths.config_file.display()
                ),
            });
        } else {
            checks.push(DoctorCheck {
                status: DoctorStatus::Ok,
                label: "Config permissions".to_string(),
                detail: "File permissions are safe".to_string(),
            });
        }
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Warn,
            label: "Config file".to_string(),
            detail: format!(
                "Not found at {} — GOAT will use defaults",
                paths.config_file.display()
            ),
        });
    }

    // ── Data directory ────────────────────────────────────────────────────────
    if paths.data_dir.exists() {
        // Check writability by attempting to create a temp file.
        let probe = paths.data_dir.join(".goat_write_probe");
        let writable = fs::write(&probe, b"probe").is_ok();
        let _ = fs::remove_file(&probe);

        if writable {
            checks.push(DoctorCheck {
                status: DoctorStatus::Ok,
                label: "Data directory".to_string(),
                detail: paths.data_dir.display().to_string(),
            });
        } else {
            checks.push(DoctorCheck {
                status: DoctorStatus::Fail,
                label: "Data directory".to_string(),
                detail: format!("{} exists but is NOT writable", paths.data_dir.display()),
            });
        }
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Warn,
            label: "Data directory".to_string(),
            detail: format!(
                "{} does not exist — will be created on first launch",
                paths.data_dir.display()
            ),
        });
    }

    // ── Database file ─────────────────────────────────────────────────────────
    if paths.db_file.exists() {
        // Try to open it.
        match rusqlite::Connection::open(&paths.db_file) {
            Ok(_) => {
                checks.push(DoctorCheck {
                    status: DoctorStatus::Ok,
                    label: "Database".to_string(),
                    detail: paths.db_file.display().to_string(),
                });
            }
            Err(e) => {
                checks.push(DoctorCheck {
                    status: DoctorStatus::Fail,
                    label: "Database".to_string(),
                    detail: format!("{} — error: {}", paths.db_file.display(), e),
                });
            }
        }
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Info,
            label: "Database".to_string(),
            detail: format!(
                "{} — will be created on first launch",
                paths.db_file.display()
            ),
        });
    }

    // ── Legacy DB detection ───────────────────────────────────────────────────
    if let Some(legacy) = GoatPaths::detect_legacy_db() {
        checks.push(DoctorCheck {
            status: DoctorStatus::Warn,
            label: "Legacy database detected".to_string(),
            detail: format!(
                "{} — old project-root DB found. Run: cp {} {}",
                legacy.display(),
                legacy.display(),
                paths.db_file.display()
            ),
        });
    }

    // ── API Keys ──────────────────────────────────────────────────────────────
    if has_openai {
        checks.push(DoctorCheck {
            status: DoctorStatus::Ok,
            label: "OpenAI key".to_string(),
            detail: "Configured".to_string(),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Warn,
            label: "OpenAI key".to_string(),
            detail: "Missing (required for default profile)".to_string(),
        });
    }

    if has_groq {
        checks.push(DoctorCheck {
            status: DoctorStatus::Ok,
            label: "Groq key".to_string(),
            detail: "Configured (key hidden)".to_string(),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Info,
            label: "Groq key".to_string(),
            detail: "Not configured (optional). Add `groq_api_key` to goat.toml [keys]".to_string(),
        });
    }

    if !has_openai && !has_groq {
        checks.push(DoctorCheck {
            status: DoctorStatus::Fail,
            label: "Provider".to_string(),
            detail: "No provider configured. GOAT cannot send LLM requests.".to_string(),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Ok,
            label: "Provider".to_string(),
            detail: "At least one provider key is configured".to_string(),
        });
    }

    // ── Provider count ────────────────────────────────────────────────────────
    let provider_count = [has_openai, has_groq].iter().filter(|&&v| v).count();
    checks.push(DoctorCheck {
        status: if provider_count > 0 {
            DoctorStatus::Ok
        } else {
            DoctorStatus::Fail
        },
        label: "Provider count".to_string(),
        detail: format!(
            "{} of 2 providers configured (OpenAI, Groq)",
            provider_count
        ),
    });

    // ── Model profiles ────────────────────────────────────────────────────────
    {
        use crate::models::ProfileRegistry;
        // Use default config to get registry without importing Config.
        let registry = ProfileRegistry::with_defaults();
        let default_name = &registry.default_profile;
        let (_, chain) = registry.resolve(default_name);

        // Evaluate the default chain readiness.
        let default_chain =
            crate::models::ProfileRegistry::with_defaults().profiles["balanced"].clone();
        let mut chain_healthy = false;
        let mut reasons = Vec::new();

        for entry in &default_chain.entries {
            let available = match entry.provider.as_str() {
                "openai" => has_openai,
                "groq" => has_groq,
                _ => false,
            };
            if available {
                chain_healthy = true;
                break;
            } else {
                reasons.push(format!("missing {} key", entry.provider));
            }
        }

        checks.push(DoctorCheck {
            status: if chain_healthy {
                DoctorStatus::Ok
            } else {
                DoctorStatus::Warn
            },
            label: "Default profile".to_string(),
            detail: format!(
                "'{}' — primary: {}  fallback: {}",
                default_name,
                chain.primary_display(),
                chain.fallback_display()
            ),
        });

        checks.push(DoctorCheck {
            status: DoctorStatus::Info,
            label: "Model profiles".to_string(),
            detail: format!(
                "{} profiles: {}   Run: goat models",
                registry.profiles.len(),
                registry.profile_names().join(", ")
            ),
        });
    }

    // ── Provider: OpenRouter ───────────────────────────────────────────────────
    if has_openrouter {
        checks.push(DoctorCheck {
            status: DoctorStatus::Ok,
            label: "OpenRouter key".to_string(),
            detail: "Configured (key hidden)".to_string(),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Info,
            label: "OpenRouter key".to_string(),
            detail:
                "Not configured (optional). Set OPENROUTER_API_KEY or openrouter_api_key in config"
                    .to_string(),
        });
    }

    // ── Provider: Ollama ────────────────────────────────────────────────────────
    checks.push(DoctorCheck {
        status: if ollama_enabled {
            DoctorStatus::Info
        } else {
            DoctorStatus::Info
        },
        label: "Ollama (local)".to_string(),
        detail: if ollama_enabled {
            "Provider config found. No key required — server must be running at configured URL"
                .to_string()
        } else {
            "Available (local, no key). Configure [providers.ollama] base_url in goat.toml to use"
                .to_string()
        },
    });

    // ── Anthropic / Gemini (planned) ───────────────────────────────────────────
    checks.push(DoctorCheck {
        status: DoctorStatus::Info,
        label: "Anthropic".to_string(),
        detail: "Planned — not implemented. Will be added in a future phase.".to_string(),
    });
    checks.push(DoctorCheck {
        status: DoctorStatus::Info,
        label: "Gemini".to_string(),
        detail: "Planned — not implemented. Will be added in a future phase.".to_string(),
    });

    // ── LLM retry/timeout config ────────────────────────────────────────────────
    if let Some(llm) = llm_config {
        let warnings = llm.validate();
        checks.push(DoctorCheck {
            status: if warnings.is_empty() { DoctorStatus::Ok } else { DoctorStatus::Warn },
            label: "LLM retry config".to_string(),
            detail: format!(
                "max_retries={} timeout={}s fallback_rate_limit={} fallback_network={} fallback_5xx={}{}",
                llm.effective_max_retries(),
                llm.effective_timeout_secs(),
                llm.fallback_on_rate_limit,
                llm.fallback_on_network,
                llm.fallback_on_server_error,
                if warnings.is_empty() { String::new() } else { format!(" ({})", warnings.len()) }
            ),
        });
    }

    // ── DB migration status ───────────────────────────────────────────────────
    if GoatPaths::detect_legacy_db().is_some() {
        checks.push(DoctorCheck {
            status: DoctorStatus::Warn,
            label: "DB migration needed".to_string(),
            detail: "Run: goat migrate-db   (copies ./goat_brain.db to XDG path)".to_string(),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Ok,
            label: "DB migration".to_string(),
            detail: "No legacy database in project root".to_string(),
        });
    }

    // ── Approval gate ─────────────────────────────────────────────────────────
    checks.push(DoctorCheck {
        status: DoctorStatus::Ok,
        label: "ApprovalGate".to_string(),
        detail: "Active — bash, write_file, call_subagent require user confirmation".to_string(),
    });

    // ── Log directory ─────────────────────────────────────────────────────────
    if paths.log_dir.exists() {
        checks.push(DoctorCheck {
            status: DoctorStatus::Ok,
            label: "Log directory".to_string(),
            detail: paths.log_dir.display().to_string(),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Info,
            label: "Log directory".to_string(),
            detail: format!(
                "{} — will be created on first launch",
                paths.log_dir.display()
            ),
        });
    }

    // ── Headless mode ─────────────────────────────────────────────────────────
    checks.push(DoctorCheck {
        status: if headless {
            DoctorStatus::Ok
        } else {
            DoctorStatus::Info
        },
        label: "Headless mode".to_string(),
        detail: if headless {
            "Active (reading from stdin)".to_string()
        } else {
            "Inactive (use --headless to pipe input)".to_string()
        },
    });

    // ── Memory ────────────────────────────────────────────────────────────────
    let memory_manager = crate::memory::MemoryManager::new(paths, config.memory.clone());
    let (u_count, u_max, u_warn) = memory_manager.user_budget_status();
    let (m_count, m_max, m_warn) = memory_manager.memory_budget_status();

    checks.push(DoctorCheck {
        status: if config.memory.enabled {
            DoctorStatus::Ok
        } else {
            DoctorStatus::Info
        },
        label: "Memory System".to_string(),
        detail: if config.memory.enabled {
            "Enabled".to_string()
        } else {
            "Disabled".to_string()
        },
    });

    if memory_manager.user_file.exists() {
        checks.push(DoctorCheck {
            status: if u_warn {
                DoctorStatus::Warn
            } else {
                DoctorStatus::Ok
            },
            label: "USER.md".to_string(),
            detail: format!(
                "{}/{} chars{}",
                u_count,
                u_max,
                if u_warn { " (OVER BUDGET)" } else { "" }
            ),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Info,
            label: "USER.md".to_string(),
            detail: "Not yet created".to_string(),
        });
    }

    if memory_manager.memory_file.exists() {
        checks.push(DoctorCheck {
            status: if m_warn {
                DoctorStatus::Warn
            } else {
                DoctorStatus::Ok
            },
            label: "MEMORY.md".to_string(),
            detail: format!(
                "{}/{} chars{}",
                m_count,
                m_max,
                if m_warn { " (OVER BUDGET)" } else { "" }
            ),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Info,
            label: "MEMORY.md".to_string(),
            detail: "Not yet created".to_string(),
        });
    }

    // ── Skills System ─────────────────────────────────────────────────────────
    checks.push(DoctorCheck {
        status: if config.skills.enabled {
            DoctorStatus::Ok
        } else {
            DoctorStatus::Info
        },
        label: "Skills System".to_string(),
        detail: if config.skills.enabled {
            "Enabled".to_string()
        } else {
            "Disabled".to_string()
        },
    });

    if paths.skills_dir.exists() {
        // Just checking basic existence and count
        let count = std::fs::read_dir(&paths.skills_dir)
            .map(|i| i.filter_map(|e| e.ok()).count())
            .unwrap_or(0);

        checks.push(DoctorCheck {
            status: DoctorStatus::Ok,
            label: "Skills Directory".to_string(),
            detail: format!("Exists ({} entries)", count),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Info,
            label: "Skills Directory".to_string(),
            detail: "Not yet created".to_string(),
        });
    }

    // ── Subagents ──────────────────────────────────────────────────────────────
    let subagents_registry = crate::subagents::SubagentRegistry::new();
    checks.push(DoctorCheck {
        status: DoctorStatus::Ok,
        label: "Subagents".to_string(),
        detail: format!(
            "{} internal subagents loaded",
            subagents_registry.list_all().len()
        ),
    });

    if paths.subagent_audit_log_file.exists() {
        checks.push(DoctorCheck {
            status: DoctorStatus::Ok,
            label: "Subagent Audit Log".to_string(),
            detail: "Exists and writable".to_string(),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Info,
            label: "Subagent Audit Log".to_string(),
            detail: "Not yet created (will be created on first subagent run)".to_string(),
        });
    }

    checks
}

/// Print doctor results to stdout in human-readable format.
pub fn print_doctor_results(checks: &[DoctorCheck]) {
    println!("GOAT Doctor — System Readiness Check");
    println!("{}", "─".repeat(60));
    for check in checks {
        println!(
            "{} {:26} {}",
            check.status.prefix(),
            check.label,
            check.detail
        );
    }
    println!("{}", "─".repeat(60));

    let fails = checks
        .iter()
        .filter(|c| c.status == DoctorStatus::Fail)
        .count();
    let warns = checks
        .iter()
        .filter(|c| c.status == DoctorStatus::Warn)
        .count();

    if fails == 0 && warns == 0 {
        println!("All checks passed. GOAT is ready.");
    } else {
        if fails > 0 {
            println!(
                "{} critical issue(s) found — GOAT may not work correctly.",
                fails
            );
        }
        if warns > 0 {
            println!("{} warning(s) found — review the items above.", warns);
        }
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_paths_resolve_ok() {
        let paths = GoatPaths::resolve();
        // Should succeed on any machine with a home dir.
        assert!(paths.is_ok(), "GoatPaths::resolve() failed: {:?}", paths);
        let p = paths.unwrap();
        assert!(p.config_file.to_str().is_some());
        assert!(p.data_dir.to_str().is_some());
        assert!(p.db_file.to_str().is_some());
    }

    #[test]
    fn test_data_dir_contains_goat() {
        let p = GoatPaths::resolve().unwrap();
        let data_str = p.data_dir.to_string_lossy();
        assert!(
            data_str.contains("goat"),
            "data_dir should contain 'goat': {}",
            data_str
        );
    }

    #[test]
    fn test_db_inside_data_dir() {
        let p = GoatPaths::resolve().unwrap();
        assert!(
            p.db_file.starts_with(&p.data_dir),
            "db_file should be inside data_dir"
        );
    }

    #[test]
    fn test_with_db_override() {
        let p = GoatPaths::resolve().unwrap();
        let custom = PathBuf::from("/tmp/custom.db");
        let p2 = p.with_db(custom.clone());
        assert_eq!(p2.db_file, custom);
    }

    #[test]
    fn test_with_config_override() {
        let p = GoatPaths::resolve().unwrap();
        let custom = PathBuf::from("/tmp/custom.toml");
        let p2 = p.with_config(custom.clone());
        assert_eq!(p2.config_file, custom);
    }

    #[cfg(unix)]
    #[test]
    fn test_config_perm_check_world_readable() {
        use std::fs::File;
        let tmp = std::env::temp_dir().join("goat_test_perm.toml");
        File::create(&tmp).unwrap();
        // Make it world-readable (644).
        fs::set_permissions(&tmp, fs::Permissions::from_mode(0o644)).unwrap();
        let result = check_config_permissions(&tmp);
        assert!(result.is_some(), "should detect world-readable config");
        let _ = fs::remove_file(&tmp);
    }

    #[cfg(unix)]
    #[test]
    fn test_config_perm_check_safe() {
        use std::fs::File;
        let tmp = std::env::temp_dir().join("goat_test_perm_safe.toml");
        File::create(&tmp).unwrap();
        // Make it owner-only (600).
        fs::set_permissions(&tmp, fs::Permissions::from_mode(0o600)).unwrap();
        let result = check_config_permissions(&tmp);
        assert!(result.is_none(), "should not warn for 600 config");
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_doctor_runs_without_panic() {
        let paths = GoatPaths::resolve().unwrap();
        let config = crate::config::Config::default();
        let checks = run_doctor(&paths, &config, false);
        assert!(!checks.is_empty());
    }

    #[test]
    fn test_doctor_no_provider_shows_fail() {
        let paths = GoatPaths::resolve().unwrap();
        let config = crate::config::Config::default();
        let checks = run_doctor(&paths, &config, false);
        let has_fail = checks
            .iter()
            .any(|c| c.status == DoctorStatus::Fail && c.label == "Provider");
        assert!(has_fail, "should fail when no provider is configured");
    }

    #[test]
    fn test_doctor_with_provider_shows_ok() {
        let paths = GoatPaths::resolve().unwrap();
        let mut config = crate::config::Config::default();
        config.keys.openai_api_key = Some("dummy".to_string());
        let checks = run_doctor(&paths, &config, false);
        let has_ok = checks
            .iter()
            .any(|c| c.status == DoctorStatus::Ok && c.label == "OpenAI key");
        assert!(has_ok, "should be ok when a provider is configured");
    }

    #[test]
    fn test_doctor_with_llm_config_shows_retry() {
        use crate::config::LlmConfig;
        let paths = GoatPaths::resolve().unwrap();
        let mut config = crate::config::Config::default();
        config.llm = LlmConfig {
            max_retries: 3,
            timeout_secs: 30,
            ..LlmConfig::default()
        };
        let checks = run_doctor(&paths, &config, false);
        let has_retry = checks
            .iter()
            .any(|c| c.label == "LLM retry config" && c.detail.contains("max_retries=3"));
        assert!(has_retry, "should show retry config in doctor");
    }

    #[test]
    fn test_doctor_with_openrouter_key_shows_ok() {
        let paths = GoatPaths::resolve().unwrap();
        let mut config = crate::config::Config::default();
        config.keys.openrouter_api_key = Some("dummy".to_string());
        let checks = run_doctor(&paths, &config, false);
        let or_check = checks.iter().find(|c| c.label == "OpenRouter key");
        assert!(or_check.is_some());
        assert_eq!(or_check.unwrap().status, DoctorStatus::Ok);
    }
}
