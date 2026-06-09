//! Repo map — lightweight, safe repository awareness for GOAT.
//!
//! Inspired by Aider's repo map, but GOAT-native: no full AST parsing,
//! no auto-inject of full source files, fully security-conscious.
//!
//! # Design
//!
//! The repo map scans the current project directory and produces:
//! - Project root and git status
//! - Detected language/framework stack
//! - Source/test directories
//! - Key files (Cargo.toml, package.json, etc.)
//! - Lightweight file metadata (path, extension, line count)
//! - Top-level symbol names (fn/struct/class/def) via simple regex — **not** full AST
//! - Git branch and dirty-tree status (if .git exists)
//!
//! # Security
//!
//! - Never reads `.env`, credential files, or secrets
//! - Skips `node_modules`, `target`, `dist`, `.git` and other build/cache dirs
//! - Symbol extraction is regex-only; no code execution
//! - Context injection is budget-capped (default 4000 chars)
//! - Secret patterns in file names trigger a warning, not a read

use crate::approval::redact_secrets;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

// ── Ignore rules ──────────────────────────────────────────────────────────────

/// Directories that are always skipped during repo map scan.
pub const IGNORED_DIRS: &[&str] = &[
    "node_modules",
    "target",
    "dist",
    "build",
    ".next",
    ".turbo",
    ".cache",
    "venv",
    ".venv",
    "__pycache__",
    ".git",
    ".svn",
    ".hg",
    "vendor",
    ".cargo",
    ".rustup",
    "coverage",
    ".nyc_output",
    "tmp",
    ".tmp",
    "out",
    ".out",
    ".build",
    "pkg",
    "bin",
    ".bin",
];

/// File name patterns that look like secrets — never read, just noted.
const SECRET_FILE_PATTERNS: &[&str] = &[
    ".env",
    ".env.local",
    ".env.production",
    ".env.development",
    "id_rsa",
    "id_ed25519",
    "id_ecdsa",
    "credentials",
    ".aws/credentials",
    ".npmrc",
    "*.pem",
    "*.key",
    "*.pfx",
    "*.p12",
];

fn is_ignored_dir(name: &str) -> bool {
    IGNORED_DIRS.iter().any(|d| d.eq_ignore_ascii_case(name))
}

fn looks_like_secret_file(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_lowercase();
    SECRET_FILE_PATTERNS.iter().any(|p| {
        if p.starts_with('*') {
            name.ends_with(&p[1..])
        } else {
            name == *p || name.ends_with(p)
        }
    })
}

// ── File info ────────────────────────────────────────────────────────────────

/// Lightweight metadata about a single source file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub extension: String,
    pub line_count: usize,
    /// Top-level symbols (fn/struct/class/def/etc.) extracted by simple regex.
    pub symbols: Vec<String>,
}

// ── Git status ────────────────────────────────────────────────────────────────

/// Lightweight git awareness — no git library, just safe process calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub branch: String,
    pub is_dirty: bool,
    pub changed_files_count: usize,
}

impl GitStatus {
    /// Try to read git status from the given directory.
    /// Returns None if git is not available or the directory is not a repo.
    pub fn read(root: &Path) -> Option<Self> {
        // Get branch name
        let branch_out = Command::new("git")
            .args([
                "-C",
                &root.to_string_lossy(),
                "rev-parse",
                "--abbrev-ref",
                "HEAD",
            ])
            .output()
            .ok()?;

        if !branch_out.status.success() {
            return None;
        }
        let branch = String::from_utf8_lossy(&branch_out.stdout)
            .trim()
            .to_string();

        // Get changed files
        let status_out = Command::new("git")
            .args(["-C", &root.to_string_lossy(), "status", "--porcelain"])
            .output()
            .ok()?;

        let status_text = String::from_utf8_lossy(&status_out.stdout).to_string();
        let changed: Vec<&str> = status_text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .collect();

        Some(GitStatus {
            branch,
            is_dirty: !changed.is_empty(),
            changed_files_count: changed.len(),
        })
    }

    pub fn summary(&self) -> String {
        if self.is_dirty {
            format!(
                "branch: {} | {} change(s) pending",
                self.branch, self.changed_files_count
            )
        } else {
            format!("branch: {} | clean", self.branch)
        }
    }
}

// ── Repo map ─────────────────────────────────────────────────────────────────

/// A complete repo map of the current project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoMap {
    pub root: String,
    pub is_git_repo: bool,
    pub git_status: Option<GitStatus>,
    pub stack: Vec<String>,
    pub source_dirs: Vec<String>,
    pub key_files: Vec<String>,
    pub files: Vec<FileInfo>,
    pub total_files_scanned: usize,
    pub ignored_dirs_skipped: usize,
    pub scanned_at: SystemTime,
}

impl RepoMap {
    /// Build a compact text representation suitable for LLM injection.
    pub fn to_compact_string(&self, max_chars: usize, include_symbols: bool) -> String {
        let mut out = String::new();

        out.push_str(&format!("Project: {}\n", self.root));

        if !self.stack.is_empty() {
            out.push_str(&format!("Stack: {}\n", self.stack.join(", ")));
        }

        if let Some(ref git) = self.git_status {
            out.push_str(&format!("Git: {}\n", git.summary()));
        } else if self.is_git_repo {
            out.push_str("Git: git repo (status unavailable)\n");
        }

        if !self.source_dirs.is_empty() {
            out.push_str(&format!("Dirs: {}\n", self.source_dirs.join(", ")));
        }

        if !self.key_files.is_empty() {
            out.push_str(&format!("Key files: {}\n", self.key_files.join(", ")));
        }

        out.push_str(&format!(
            "Files: {} scanned ({} dirs ignored)\n",
            self.total_files_scanned, self.ignored_dirs_skipped
        ));

        if !self.files.is_empty() {
            out.push_str("\n--- Source Files ---\n");
            for f in &self.files {
                let sym_line = if include_symbols && !f.symbols.is_empty() {
                    format!(" [{}]", f.symbols.join(", "))
                } else {
                    String::new()
                };
                let entry = format!("{} (~{} lines){}\n", f.path, f.line_count, sym_line);
                if out.len() + entry.len() > max_chars {
                    out.push_str("... (truncated — repo map budget reached)\n");
                    break;
                }
                out.push_str(&entry);
            }
        }

        out
    }
}

// ── Symbol extraction ────────────────────────────────────────────────────────

/// Extract top-level symbols from source code using simple regex (no AST).
fn extract_symbols(content: &str, extension: &str) -> Vec<String> {
    let mut symbols = Vec::new();

    match extension {
        "rs" => {
            // Rust: fn, struct, enum, trait, mod, impl (capture the name)
            let patterns = [
                r"(?m)^(?:pub(?:\s+\w+)?\s+)?fn\s+(\w+)",
                r"(?m)^(?:pub(?:\s+\w+)?\s+)?struct\s+(\w+)",
                r"(?m)^(?:pub(?:\s+\w+)?\s+)?enum\s+(\w+)",
                r"(?m)^(?:pub(?:\s+\w+)?\s+)?trait\s+(\w+)",
                r"(?m)^(?:pub(?:\s+\w+)?\s+)?mod\s+(\w+)",
            ];
            for pat in &patterns {
                if let Ok(re) = Regex::new(pat) {
                    for cap in re.captures_iter(content) {
                        if let Some(m) = cap.get(1) {
                            let name = m.as_str().to_string();
                            if !symbols.contains(&name) {
                                symbols.push(name);
                            }
                        }
                    }
                }
            }
        }
        "js" | "ts" | "jsx" | "tsx" => {
            // JS/TS: function, class, export const/let/function
            let patterns = [
                r"(?m)^(?:export\s+)?(?:async\s+)?function\s+(\w+)",
                r"(?m)^(?:export\s+)?class\s+(\w+)",
                r"(?m)^export\s+(?:const|let|var)\s+(\w+)",
            ];
            for pat in &patterns {
                if let Ok(re) = Regex::new(pat) {
                    for cap in re.captures_iter(content) {
                        if let Some(m) = cap.get(1) {
                            let name = m.as_str().to_string();
                            if !symbols.contains(&name) {
                                symbols.push(name);
                            }
                        }
                    }
                }
            }
        }
        "py" => {
            // Python: def, class
            let patterns = [r"(?m)^(?:async\s+)?def\s+(\w+)", r"(?m)^class\s+(\w+)"];
            for pat in &patterns {
                if let Ok(re) = Regex::new(pat) {
                    for cap in re.captures_iter(content) {
                        if let Some(m) = cap.get(1) {
                            let name = m.as_str().to_string();
                            if !symbols.contains(&name) {
                                symbols.push(name);
                            }
                        }
                    }
                }
            }
        }
        "go" => {
            let patterns = [
                r"(?m)^func\s+\(?(?:\w+\s+)?\*?(?:\w+\s+)?\)?(\w+)\s*\(",
                r"(?m)^type\s+(\w+)\s+struct",
                r"(?m)^type\s+(\w+)\s+interface",
            ];
            for pat in &patterns {
                if let Ok(re) = Regex::new(pat) {
                    for cap in re.captures_iter(content) {
                        if let Some(m) = cap.get(1) {
                            let name = m.as_str().to_string();
                            if !symbols.contains(&name) {
                                symbols.push(name);
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }

    // Cap to 20 symbols per file to keep things concise
    symbols.truncate(20);
    symbols
}

// ── Source file extensions ────────────────────────────────────────────────────

const SOURCE_EXTENSIONS: &[&str] = &[
    "rs", "go", "py", "js", "ts", "jsx", "tsx", "java", "kt", "cpp", "c", "h", "hpp", "cs", "rb",
    "swift", "php", "lua", "zig",
];

fn is_source_file(ext: &str) -> bool {
    SOURCE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
}

// ── Scanner ───────────────────────────────────────────────────────────────────

pub struct RepoMapScanner {
    root: PathBuf,
    include_symbols: bool,
    max_file_bytes: u64,
}

impl RepoMapScanner {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            include_symbols: true,
            max_file_bytes: 500_000, // 500KB per file max
        }
    }

    pub fn with_no_symbols(mut self) -> Self {
        self.include_symbols = false;
        self
    }

    pub fn scan(&self) -> Result<RepoMap> {
        let is_git_repo = self.root.join(".git").exists();
        let git_status = if is_git_repo {
            GitStatus::read(&self.root)
        } else {
            None
        };

        // Detect stack from key files
        let known_packages: &[(&str, &str)] = &[
            ("Cargo.toml", "Rust"),
            ("package.json", "Node/JS"),
            ("pyproject.toml", "Python"),
            ("requirements.txt", "Python"),
            ("go.mod", "Go"),
            ("pom.xml", "Java/Maven"),
            ("build.gradle", "Java/Gradle"),
            ("Dockerfile", "Docker"),
            ("docker-compose.yml", "Docker"),
            ("Makefile", "Make"),
        ];

        let mut stack = Vec::new();
        let mut key_files = Vec::new();

        for (fname, tech) in known_packages {
            if self.root.join(fname).exists() {
                key_files.push(fname.to_string());
                if !stack.contains(&tech.to_string()) {
                    stack.push(tech.to_string());
                }
            }
        }

        // Source directories
        let known_src_dirs = ["src", "app", "pages", "components", "lib", "tests", "docs"];
        let source_dirs: Vec<String> = known_src_dirs
            .iter()
            .filter(|d| self.root.join(d).is_dir())
            .map(|d| d.to_string())
            .collect();

        // Walk files
        let mut files = Vec::new();
        let mut total_files_scanned = 0usize;
        let mut ignored_dirs_skipped = 0usize;

        self.walk_dir(
            &self.root,
            &mut files,
            &mut total_files_scanned,
            &mut ignored_dirs_skipped,
            0,
        );

        Ok(RepoMap {
            root: self.root.display().to_string(),
            is_git_repo,
            git_status,
            stack,
            source_dirs,
            key_files,
            files,
            total_files_scanned,
            ignored_dirs_skipped,
            scanned_at: SystemTime::now(),
        })
    }

    fn walk_dir(
        &self,
        dir: &Path,
        files: &mut Vec<FileInfo>,
        total: &mut usize,
        ignored: &mut usize,
        depth: usize,
    ) {
        // Cap recursion depth at 8 to prevent runaway scans
        if depth > 8 {
            return;
        }

        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if path.is_dir() {
                if is_ignored_dir(&name_str) {
                    *ignored += 1;
                } else {
                    self.walk_dir(&path, files, total, ignored, depth + 1);
                }
                continue;
            }

            if path.is_file() {
                *total += 1;

                // Skip secret files
                if looks_like_secret_file(&path) {
                    continue;
                }

                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                if !is_source_file(&ext) {
                    continue;
                }

                // Skip very large files
                let file_size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                if file_size > self.max_file_bytes {
                    continue;
                }

                let rel_path = path
                    .strip_prefix(&self.root)
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| path.display().to_string());

                let content = fs::read_to_string(&path).unwrap_or_default();
                let line_count = content.lines().count();

                let symbols = if self.include_symbols {
                    extract_symbols(&content, &ext)
                } else {
                    Vec::new()
                };

                files.push(FileInfo {
                    path: rel_path,
                    extension: ext,
                    line_count,
                    symbols,
                });
            }
        }
    }
}

// ── Diff generation ──────────────────────────────────────────────────────────

/// A preview of a proposed file write, shown before ApprovalGate.
#[derive(Debug, Clone)]
pub struct DiffPreview {
    pub path: String,
    pub is_new_file: bool,
    pub added_lines: usize,
    pub removed_lines: usize,
    pub diff_text: String,
    pub has_secret_warning: bool,
}

/// Generate a unified diff preview for a proposed write to `path` with `new_content`.
///
/// If the file does not exist yet, returns a "new file" summary.
/// The diff is capped to 80 lines for display purposes.
pub fn generate_diff_preview(path: &str, new_content: &str) -> DiffPreview {
    let has_secret_warning = detect_secret_in_content(new_content);
    let redacted_new = if has_secret_warning {
        redact_secrets(new_content)
    } else {
        new_content.to_string()
    };

    let file_path = Path::new(path);

    if !file_path.exists() {
        // New file
        let added = new_content.lines().count();
        let preview_lines: Vec<String> = redacted_new
            .lines()
            .take(40)
            .map(|l| format!("+ {}", l))
            .collect();
        let truncated = added > 40;
        let diff_text = if truncated {
            format!(
                "--- /dev/null\n+++ {}\n{}\n... ({} more lines)\n",
                path,
                preview_lines.join("\n"),
                added - 40
            )
        } else {
            format!(
                "--- /dev/null\n+++ {}\n{}\n",
                path,
                preview_lines.join("\n")
            )
        };

        return DiffPreview {
            path: path.to_string(),
            is_new_file: true,
            added_lines: added,
            removed_lines: 0,
            diff_text,
            has_secret_warning,
        };
    }

    // Existing file — generate unified diff
    let old_content = fs::read_to_string(file_path).unwrap_or_default();

    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = redacted_new.lines().collect();

    let mut diff_lines: Vec<String> = Vec::new();
    let mut added = 0usize;
    let mut removed = 0usize;

    // Simple linear diff: compare line by line (good enough for UX preview)
    // For real diffs we use a patience-diff-like approach via the `similar` crate
    // but here we keep it dependency-free with a simple Myers diff approximation.
    let hunks = simple_diff(&old_lines, &new_lines);

    diff_lines.push(format!("--- a/{}", path));
    diff_lines.push(format!("+++ b/{}", path));

    for hunk in &hunks {
        diff_lines.push(hunk.header.clone());
        for line in &hunk.lines {
            match line.kind {
                DiffLineKind::Add => {
                    diff_lines.push(format!("+ {}", line.content));
                    added += 1;
                }
                DiffLineKind::Remove => {
                    diff_lines.push(format!("- {}", line.content));
                    removed += 1;
                }
                DiffLineKind::Context => {
                    diff_lines.push(format!("  {}", line.content));
                }
            }
        }
    }

    let total_diff_lines = diff_lines.len();
    let truncated = total_diff_lines > 80;
    let display_lines: Vec<String> = diff_lines.into_iter().take(80).collect();

    let diff_text = if truncated {
        format!(
            "{}\n... ({} more diff lines not shown)\n",
            display_lines.join("\n"),
            total_diff_lines - 80
        )
    } else {
        display_lines.join("\n")
    };

    DiffPreview {
        path: path.to_string(),
        is_new_file: false,
        added_lines: added,
        removed_lines: removed,
        diff_text,
        has_secret_warning,
    }
}

/// Detect secret-like content in a proposed file write.
fn detect_secret_in_content(content: &str) -> bool {
    let lower = content.to_lowercase();
    let patterns = [
        "sk-",
        "gsk_",
        "ghp_",
        "xoxb-",
        "ey", // JWT prefix
        "api_key",
        "apikey",
        "api-key",
        "secret",
        "password",
        "passwd",
        "-----begin rsa",
        "-----begin openssh",
        "-----begin ec",
    ];
    patterns.iter().any(|p| lower.contains(p))
}

// ── Minimal diff engine (no external dependency) ──────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiffLineKind {
    Add,
    Remove,
    Context,
}

struct DiffLine<'a> {
    kind: DiffLineKind,
    content: &'a str,
}

struct DiffHunk<'a> {
    header: String,
    lines: Vec<DiffLine<'a>>,
}

/// Simple LCS-based diff for small files.  For very large files (>2000 lines)
/// we fall back to a "too large for preview" message.
fn simple_diff<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<DiffHunk<'a>> {
    if old.len() > 2000 || new.len() > 2000 {
        return vec![DiffHunk {
            header: "@@ (file too large for inline preview) @@".to_string(),
            lines: Vec::new(),
        }];
    }

    // Build LCS table
    let m = old.len();
    let n = new.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[i][j] = if old[i] == new[j] {
                dp[i + 1][j + 1] + 1
            } else {
                dp[i + 1][j].max(dp[i][j + 1])
            };
        }
    }

    // Trace back
    let mut edits: Vec<(DiffLineKind, usize, &str)> = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < m || j < n {
        if i < m && j < n && old[i] == new[j] {
            edits.push((DiffLineKind::Context, i, old[i]));
            i += 1;
            j += 1;
        } else if j < n && (i >= m || dp[i][j + 1] >= dp[i + 1][j]) {
            edits.push((DiffLineKind::Add, j, new[j]));
            j += 1;
        } else {
            edits.push((DiffLineKind::Remove, i, old[i]));
            i += 1;
        }
    }

    // Group into a single hunk (simplified — no multi-hunk support)
    let context_lines = 3;
    let has_changes = edits.iter().any(|(k, _, _)| *k != DiffLineKind::Context);

    if !has_changes {
        return Vec::new();
    }

    let header = format!("@@ -{},{} +{},{} @@", 1, m, 1, n);
    let mut hunk_lines = Vec::new();

    for (kind, _, content) in &edits {
        // Only show context_lines around changes
        // For simplicity in this implementation, include all lines
        // (the display cap of 80 handles truncation)
        let _ = context_lines;
        hunk_lines.push(DiffLine {
            kind: *kind,
            content,
        });
    }

    vec![DiffHunk {
        header,
        lines: hunk_lines,
    }]
}

/// Format a DiffPreview for display in TUI/headless.
pub fn format_diff_preview(preview: &DiffPreview) -> Vec<String> {
    let mut lines = Vec::new();

    if preview.is_new_file {
        lines.push(format!("┌─ NEW FILE: {} ─", preview.path));
        lines.push(format!("│  +{} lines to be written", preview.added_lines));
    } else {
        lines.push(format!("┌─ DIFF: {} ─", preview.path));
        lines.push(format!(
            "│  +{} added  -{} removed",
            preview.added_lines, preview.removed_lines
        ));
    }

    if preview.has_secret_warning {
        lines.push(
            "│  ⚠ WARNING: Secret-like content detected — values redacted in preview!".to_string(),
        );
    }

    lines.push("│".to_string());

    for diff_line in preview.diff_text.lines() {
        lines.push(format!("│  {}", diff_line));
    }

    lines.push("└────────────────────────────────────────────────".to_string());
    lines
}

// ── Command detection ─────────────────────────────────────────────────────────

/// Detected dev commands for a project.
#[derive(Debug, Clone)]
pub struct ProjectCommands {
    pub check: Option<String>,
    pub test: Option<String>,
    pub lint: Option<String>,
    pub format: Option<String>,
    pub build: Option<String>,
}

impl ProjectCommands {
    /// Auto-detect commands from a project root directory.
    pub fn detect(root: &Path) -> Self {
        let mut cmds = ProjectCommands {
            check: None,
            test: None,
            lint: None,
            format: None,
            build: None,
        };

        // Rust
        if root.join("Cargo.toml").exists() {
            cmds.check = Some("cargo check".to_string());
            cmds.test = Some("cargo test".to_string());
            cmds.lint = Some("cargo clippy".to_string());
            cmds.format = Some("cargo fmt".to_string());
            cmds.build = Some("cargo build".to_string());
            return cmds;
        }

        // Node.js — parse package.json scripts
        if root.join("package.json").exists() {
            if let Ok(content) = fs::read_to_string(root.join("package.json")) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    let scripts = json
                        .get("scripts")
                        .and_then(|s| s.as_object())
                        .cloned()
                        .unwrap_or_default();

                    let runner = if root.join("pnpm-lock.yaml").exists() {
                        "pnpm"
                    } else if root.join("yarn.lock").exists() {
                        "yarn"
                    } else {
                        "npm"
                    };

                    if scripts.contains_key("typecheck") {
                        cmds.check = Some(format!("{} run typecheck", runner));
                    } else if scripts.contains_key("build") {
                        cmds.check = Some(format!("{} run build", runner));
                    }
                    if scripts.contains_key("test") {
                        cmds.test = Some(format!("{} run test", runner));
                    }
                    if scripts.contains_key("lint") {
                        cmds.lint = Some(format!("{} run lint", runner));
                    }
                    if scripts.contains_key("format") || scripts.contains_key("fmt") {
                        let key = if scripts.contains_key("format") {
                            "format"
                        } else {
                            "fmt"
                        };
                        cmds.format = Some(format!("{} run {}", runner, key));
                    }
                    if scripts.contains_key("build") {
                        cmds.build = Some(format!("{} run build", runner));
                    }
                }
            }
            return cmds;
        }

        // Python
        if root.join("pyproject.toml").exists() || root.join("requirements.txt").exists() {
            // ruff
            cmds.lint = Some("ruff check .".to_string());
            cmds.format = Some("ruff format .".to_string());
            // pytest
            cmds.test = Some("pytest".to_string());
            return cmds;
        }

        // Go
        if root.join("go.mod").exists() {
            cmds.check = Some("go build ./...".to_string());
            cmds.test = Some("go test ./...".to_string());
            cmds.lint = Some("golint ./...".to_string());
            cmds.format = Some("gofmt -w .".to_string());
            return cmds;
        }

        // Makefile fallback
        if root.join("Makefile").exists() {
            cmds.build = Some("make".to_string());
            cmds.test = Some("make test".to_string());
        }

        cmds
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn make_temp_project(files: &[(&str, &str)]) -> TempDir {
        let dir = TempDir::new().expect("temp dir");
        for (name, content) in files {
            let path = dir.path().join(name);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            let mut f = std::fs::File::create(&path).expect("create file");
            f.write_all(content.as_bytes()).ok();
        }
        dir
    }

    #[test]
    fn test_is_ignored_dir() {
        assert!(is_ignored_dir("node_modules"));
        assert!(is_ignored_dir("target"));
        assert!(is_ignored_dir(".git"));
        assert!(!is_ignored_dir("src"));
        assert!(!is_ignored_dir("app"));
    }

    #[test]
    fn test_secret_file_detection() {
        assert!(looks_like_secret_file(Path::new(".env")));
        assert!(looks_like_secret_file(Path::new("id_rsa")));
        assert!(looks_like_secret_file(Path::new("server.key")));
        assert!(!looks_like_secret_file(Path::new("main.rs")));
        assert!(!looks_like_secret_file(Path::new("config.toml")));
    }

    #[test]
    fn test_symbol_extraction_rust() {
        let content = r#"
pub fn hello_world() {}
pub struct MyStruct {}
pub enum Status { Ok, Err }
pub trait Readable {}
fn private_fn() {}
"#;
        let symbols = extract_symbols(content, "rs");
        assert!(symbols.contains(&"hello_world".to_string()));
        assert!(symbols.contains(&"MyStruct".to_string()));
        assert!(symbols.contains(&"Status".to_string()));
        assert!(symbols.contains(&"Readable".to_string()));
        assert!(symbols.contains(&"private_fn".to_string()));
    }

    #[test]
    fn test_symbol_extraction_python() {
        let content = r#"
def my_function():
    pass

class MyClass:
    def method(self):
        pass

async def async_fn():
    pass
"#;
        let symbols = extract_symbols(content, "py");
        assert!(symbols.contains(&"my_function".to_string()));
        assert!(symbols.contains(&"MyClass".to_string()));
        assert!(symbols.contains(&"async_fn".to_string()));
    }

    #[test]
    fn test_symbol_extraction_js() {
        let content = r#"
export function myFunc() {}
export class MyComponent {}
export const myConst = 42;
function localFn() {}
"#;
        let symbols = extract_symbols(content, "js");
        assert!(symbols.contains(&"myFunc".to_string()));
        assert!(symbols.contains(&"MyComponent".to_string()));
    }

    #[test]
    fn test_repo_map_scan_ignores_target() {
        let dir = make_temp_project(&[
            ("Cargo.toml", "[package]\nname = \"test\""),
            ("src/main.rs", "fn main() {}"),
        ]);
        std::fs::create_dir_all(dir.path().join("target/debug")).ok();
        std::fs::write(dir.path().join("target/debug/foo.rs"), "fn junk() {}").ok();

        let scanner = RepoMapScanner::new(dir.path().to_path_buf());
        let map = scanner.scan().expect("scan failed");

        // target/ should be in ignored dirs
        assert!(map.ignored_dirs_skipped > 0);
        // No files from target/ should appear
        assert!(!map.files.iter().any(|f| f.path.contains("target")));
    }

    #[test]
    fn test_repo_map_scan_finds_rust_files() {
        let dir = make_temp_project(&[
            ("Cargo.toml", "[package]\nname = \"test\""),
            ("src/lib.rs", "pub fn add(a: i32, b: i32) -> i32 { a + b }"),
        ]);

        let scanner = RepoMapScanner::new(dir.path().to_path_buf());
        let map = scanner.scan().expect("scan failed");

        assert!(map.stack.contains(&"Rust".to_string()));
        assert!(!map.files.is_empty());
        let lib = map.files.iter().find(|f| f.path.ends_with("lib.rs"));
        assert!(lib.is_some());
    }

    #[test]
    fn test_diff_preview_new_file() {
        let preview = generate_diff_preview("/tmp/new_file.rs", "fn main() {}\n");
        assert!(preview.is_new_file);
        assert_eq!(preview.added_lines, 1);
        assert_eq!(preview.removed_lines, 0);
    }

    #[test]
    fn test_diff_preview_secret_detection() {
        // Use key=value format so redact_secrets can locate and redact the value.
        let content = "API_KEY=sk-abc123456789\n";
        let preview = generate_diff_preview("/tmp/test.rs", content);
        // Secret pattern should be detected
        assert!(preview.has_secret_warning);
        // The actual secret value should be redacted in the diff text
        assert!(
            !preview.diff_text.contains("sk-abc123456789"),
            "Secret was not redacted in diff: {}",
            preview.diff_text
        );
    }

    #[test]
    fn test_repo_map_to_compact_string_budget() {
        let map = RepoMap {
            root: "/home/user/project".to_string(),
            is_git_repo: false,
            git_status: None,
            stack: vec!["Rust".to_string()],
            source_dirs: vec!["src".to_string()],
            key_files: vec!["Cargo.toml".to_string()],
            files: vec![FileInfo {
                path: "src/main.rs".to_string(),
                extension: "rs".to_string(),
                line_count: 100,
                symbols: vec!["main".to_string()],
            }],
            total_files_scanned: 1,
            ignored_dirs_skipped: 1,
            scanned_at: SystemTime::now(),
        };

        let compact = map.to_compact_string(4000, true);
        assert!(compact.len() <= 4000);
        assert!(compact.contains("Rust"));
        assert!(compact.contains("main.rs"));
    }

    #[test]
    fn test_command_detection_rust() {
        let dir = make_temp_project(&[("Cargo.toml", "[package]\nname = \"test\"")]);
        let cmds = ProjectCommands::detect(dir.path());
        assert_eq!(cmds.check, Some("cargo check".to_string()));
        assert_eq!(cmds.test, Some("cargo test".to_string()));
        assert_eq!(cmds.lint, Some("cargo clippy".to_string()));
        assert_eq!(cmds.format, Some("cargo fmt".to_string()));
    }

    #[test]
    fn test_command_detection_python() {
        let dir = make_temp_project(&[("pyproject.toml", "[tool.pytest]")]);
        let cmds = ProjectCommands::detect(dir.path());
        assert_eq!(cmds.test, Some("pytest".to_string()));
    }

    #[test]
    fn test_secret_content_detection() {
        assert!(detect_secret_in_content("api_key = \"sk-1234567890\""));
        assert!(detect_secret_in_content("password = \"hunter2\""));
        assert!(!detect_secret_in_content("let x = 42;"));
        assert!(!detect_secret_in_content("fn main() {}"));
    }
}
