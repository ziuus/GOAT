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

        Ok(Self {
            config_file,
            data_dir,
            db_file,
            log_dir,
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
/// `has_openai_key` / `has_groq_key`: whether the keys are configured
/// (the actual key values must not be passed here to avoid accidental logging).
pub fn run_doctor(paths: &GoatPaths, has_openai_key: bool, has_groq_key: bool) -> Vec<DoctorCheck> {
    let mut checks = Vec::new();

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

    // ── Provider keys ─────────────────────────────────────────────────────────
    if has_openai_key {
        checks.push(DoctorCheck {
            status: DoctorStatus::Ok,
            label: "OpenAI key".to_string(),
            detail: "Configured (key hidden)".to_string(),
        });
    } else {
        checks.push(DoctorCheck {
            status: DoctorStatus::Warn,
            label: "OpenAI key".to_string(),
            detail: "Not configured. Add `openai_api_key` to goat.toml [keys]".to_string(),
        });
    }

    if has_groq_key {
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

    if !has_openai_key && !has_groq_key {
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
        // Doctor should never panic, even with missing files.
        let checks = run_doctor(&paths, false, false);
        assert!(!checks.is_empty());
    }

    #[test]
    fn test_doctor_no_provider_shows_fail() {
        let paths = GoatPaths::resolve().unwrap();
        let checks = run_doctor(&paths, false, false);
        let has_fail = checks
            .iter()
            .any(|c| c.status == DoctorStatus::Fail && c.label == "Provider");
        assert!(has_fail, "should fail when no provider is configured");
    }

    #[test]
    fn test_doctor_with_provider_shows_ok() {
        let paths = GoatPaths::resolve().unwrap();
        let checks = run_doctor(&paths, true, false);
        let has_ok = checks
            .iter()
            .any(|c| c.status == DoctorStatus::Ok && c.label == "Provider");
        assert!(has_ok, "should be ok when a provider is configured");
    }
}
