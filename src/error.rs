//! Typed error types for GOAT.
//!
//! `GoatError` covers internal, user-facing failure modes.
//! Application code uses `anyhow::Result` for ergonomic error propagation;
//! these typed variants are used where callers need to match on the cause.

use thiserror::Error;

/// Top-level GOAT error type.
#[derive(Debug, Error)]
pub enum GoatError {
    // ── Path / IO ─────────────────────────────────────────────────────────────
    #[error("could not determine home directory")]
    NoHomeDir,

    #[error("could not determine XDG data directory")]
    NoDataDir,

    #[error("failed to create data directory '{path}': {source}")]
    CreateDataDir {
        path: String,
        source: std::io::Error,
    },

    #[error("failed to read file '{path}': {source}")]
    ReadFile {
        path: String,
        source: std::io::Error,
    },

    #[error("failed to write file '{path}': {source}")]
    WriteFile {
        path: String,
        source: std::io::Error,
    },

    #[error("failed to open database at '{path}': {source}")]
    OpenDatabase {
        path: String,
        source: rusqlite::Error,
    },

    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),

    // ── Config ────────────────────────────────────────────────────────────────
    #[error("failed to parse config file '{path}': {source}")]
    ParseConfig {
        path: String,
        source: toml::de::Error,
    },

    #[error("config file is world-readable (permissions {mode:o}): run `chmod 600 {path}`")]
    InsecureConfigPermissions { path: String, mode: u32 },

    // ── Provider / LLM ────────────────────────────────────────────────────────
    #[error("no LLM provider configured; add keys to goat.toml or set environment variables")]
    NoProviderConfigured,

    #[error("LLM request failed: {0}")]
    LlmRequest(String),

    // ── Session ───────────────────────────────────────────────────────────────
    #[error("session not found: {0}")]
    SessionNotFound(String),

    // ── Tool ─────────────────────────────────────────────────────────────────
    #[error("tool '{name}' execution failed: {reason}")]
    ToolExecution { name: String, reason: String },

    // ── Generic ───────────────────────────────────────────────────────────────
    #[error("internal error: {0}")]
    Internal(String),
}
