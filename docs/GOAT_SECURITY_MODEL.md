# GOAT — Security Model

**Version:** 0.2 (Phase 1.1 — ApprovalGate implemented)  
**Last Updated:** 2026-06-08  
**Current Security Status:** IMPROVED — approval gates implemented for bash, write_file, call_subagent

> This document describes the intended security model. Items marked MISSING are not yet implemented.

---

## 1. Threat Model

GOAT is a local AI agent that can execute shell commands, read/write files, spawn subprocesses, and make network requests. The primary threats are:

| Threat | Description | Severity |
|--------|-------------|---------|
| Prompt injection | Malicious content in LLM responses causes dangerous tool execution | HIGH |
| Unrestricted shell execution | Agent runs destructive commands without user approval | HIGH |
| Unrestricted file writes | Agent overwrites critical system files | HIGH |
| Secret leakage | API keys appear in logs, prompts, or error messages | HIGH |
| Rogue external agent | External subagent makes unauthorized changes | HIGH |
| Unauthorized memory scanning | `learn_about_me` reads sensitive files without consent | MEDIUM |
| Data exfiltration | Agent reads sensitive files and sends contents to LLM | MEDIUM |
| Supply chain risk | Malicious MCP server gains system access | MEDIUM |
| Denial of service | Runaway agent loop exhausts resources or API budget | LOW |

---

## 2. Permission System

### 2.1 Operation Categories (Current Design)

Every tool call must declare its operation category:

| Category | Examples | Default Behavior |
|----------|---------|-----------------|
| `ReadFile` | read_file, list_dir | Approve once per session |
| `WriteFile` | write_file, create_dir | **Always require approval** |
| `Shell` | bash, exec | **Always require approval** |
| `Network` | HTTP calls to arbitrary URLs | Approve once per session |
| `SubagentSpawn` | call_subagent, MCP spawn | **Always require approval** |
| `MemoryRead` | brain queries | Auto-approved |
| `MemoryWrite` | brain inserts | Auto-approved |
| `MemoryScan` | index_paths | Require approval per directory |

### 2.2 Approval Gate (✅ IMPLEMENTED — Phase 1.1)

Before any `Shell`, `WriteFile`, or `SubagentSpawn` operation, GOAT:

1. Checks the [`ApprovalGate`] session policy (is this tool always-allowed/always-denied?).
2. If no policy, surfaces an approval prompt — both in the TUI log panel and as a centred overlay box.
3. Suspends the agent loop and intercepts the next keypress from the TUI event loop.
4. Resolves via user input (see below) and logs the decision via `tracing`.
5. If denied, pushes a tool-result message back to the LLM so it can adapt its plan.

**Implementation:** `src/approval.rs` — `ApprovalGate`, `ApprovalRequest`, `ApprovalDecision`, `SessionPolicy`

**Key inputs:**

| Key | Outcome |
|-----|---------|
| `y` | Approve once |
| `n` | Deny once |
| `a` | Approve + set session policy to always-allow for this tool |
| `d` | Deny + set session policy to always-deny for this tool |
| any other key | Denied (safe default) |

```
╔══════════════ APPROVAL REQUIRED ══════════════╗
  Tool   : bash
  Action : rm -rf /tmp/test_dir
  Risk   : HIGH
  Note   : This command matches patterns associated with destructive operations.
  Cwd    : /home/user/project
  [y] Approve once  [n] Deny  [a] Always allow (session)  [d] Always deny (session)
╚════════════════════════════════════════════════╝
```

### 2.3 Allowlist and Blocklist (MISSING — Phase 2)

Config-driven lists to always allow or always deny:

```toml
[security]
# Commands that are always blocked without approval, even with 'approve all'
command_blocklist = [
    "rm -rf /",
    "dd if=",
    "mkfs",
    "chmod 777 /",
    ":(){ :|:& };:"  # fork bomb
]

# Path prefixes that are always read-only (block writes)
path_write_blocklist = [
    "/etc/",
    "/usr/",
    "/bin/",
    "/sbin/",
    "~/.ssh/",
    "~/.gnupg/",
    "~/.aws/credentials"
]

# Path prefixes the agent is never allowed to read
path_read_blocklist = [
    "~/.ssh/id_rsa",
    "~/.gnupg/",
    "~/.aws/credentials"
]
```

---

## 3. Secret Protection

### 3.1 API Key Storage (Current — Insufficient)

Currently: API keys are stored in plaintext in `~/.config/goat/goat.toml`.

Planned improvements:
- Phase 1: Warn user if config file permissions allow world-read (`chmod 600 goat.toml`)
- Phase 3: Support system keyring via `secret-service` or `keychain` crate
- Phase 3: Support env-var references in config (`key = "${OPENAI_API_KEY}"`)

### 3.2 Secret Detection (✅ IMPLEMENTED FOR MEMORY — Phase 2.0)

Before any of the following, GOAT will scan for patterns resembling API keys, passwords, or tokens:
- **Memory writes:** `USER.md` and `MEMORY.md` additions are scanned and rejected if secrets are detected.
- Tool arguments sent to LLM (Planned)
- Tool results sent to LLM (Planned)
- Memory indexed from files (Planned)

Patterns detected (Memory Manager):
- `sk-` prefixed strings (OpenAI)
- `gsk_` prefixed strings (Groq)
- `AKIA` prefixed strings (AWS)
- `-----BEGIN` blocks (RSA/SSH keys)
- `password=`, `api_key=`, `secret=` patterns

### 3.3 Log Redaction (MISSING — Phase 2)

All tracing log output and audit log output must pass through a redaction filter before writing:
- Redact detected secrets
- Replace with `[REDACTED:openai_key]` or similar

---

## 4. Sandboxing

### 4.1 Current State

No sandboxing exists. All tools run with full user permissions.

### 4.2 Planned Sandbox Mode (Phase 6)

`--sandbox` CLI flag enables:
- Filesystem: read-only for all paths outside workspace
- Network: block all outbound except LLM provider endpoints
- Subprocess: no subprocess spawning
- Memory writes: allowed but scoped to session

Implementation options:
- Linux: seccomp-bpf syscall filtering
- Simpler: path allowlist enforcement in `goat-security` before any fs operation
- MVP: path prefix validation (no write outside workspace directory)

---

## 5. Audit Log

### 5.1 Current State

No persistent audit log. Tool executions are logged to tracing (file log) but in unstructured format.

### 5.2 Planned Audit Log (Phase 2)

Location: `~/.local/share/goat/audit.log`

Format (JSONL):
```json
{
  "timestamp": "2026-06-08T14:00:00Z",
  "session_id": "uuid",
  "tool": "bash",
  "args": {"command": "ls -la"},
  "approved_by": "user",
  "approval_mode": "interactive",
  "result_status": "success",
  "exit_code": 0
}
```

Properties:
- Append-only (no overwriting)
- Never deleted by GOAT automatically (user controls rotation)
- API keys never appear in audit log (redacted before write)

---

## 6. External Agent Security

### 6.1 Current State

`call_subagent` tool in `tools.rs` runs any CLI command with any prompt arg. No controls.

### 6.2 Planned Controls (Phase 5)

Before spawning any external agent:
1. Show approval prompt with agent name and prompt
2. Run in isolated working directory (not the user's project)
3. Set environment: only pass explicitly whitelisted env vars
4. Set timeout: kill if exceeds budget
5. Capture and display all output before committing any changes
6. No direct filesystem changes from external agent without GOAT review

---

## 7. Skills System Security (Phase 2.1)

Skills are treated as **untrusted markdown files** that provide instructions to the LLM. 

Security rules:
1. **No automatic execution**: Skills *never* bypass the `ApprovalGate`. If a skill instructs the agent to run a shell command or write a file, the user must still approve it.
2. **Progressive Disclosure**: Only a short index is injected by default. The full skill content is loaded only when requested.
3. **Validation & Suspicious Content**: Skills are scanned during load (and via `goat skills validate`) for patterns like `rm -rf`, `curl | sh`, `sk-`, `sudo`, and password fragments. If detected, the skill is loaded with a `[SUSPICIOUS]` warning tag.
4. **Context Budget Enforcement**: Skills are truncated if they exceed the configured injection budget, preventing prompt overflow attacks.

---

## 8. MCP Server Security

### 7.1 Current State

MCP servers are spawned with full user permissions. No validation of server identity.

### 7.2 Planned Controls (Phase 6)

- MCP server binary must be listed in config (no dynamic server loading from prompts)
- Approval required before connecting to a new MCP server
- Tool call arguments validated against declared schema before dispatch
- MCP server output size limits
- Rate limiting per MCP server per session

---

## 8. Memory Scanning Security

### 8.1 Current State

`learn_about_me` scans `~/Projects`, `~/PAI`, `~/Documents`, `~/.config/goat` without any user approval.

### 8.2 Planned Controls (Phase 1/4)

- Phase 1: Surface a confirmation dialog before any file indexing starts
- Phase 4: Show which directories will be scanned before starting
- Phase 4: Allow users to configure exactly which paths to include/exclude
- Never index: `.env`, `.env.*`, `credentials`, `secrets`, `.ssh/`, `.gnupg/`, `*.pem`, `*.key`, `*.p12`
- Check for secret patterns before indexing file content into brain DB

---

## 9. Network Security

### 9.1 Current State

No controls. `reqwest` makes HTTP requests to any URL the provider config points to.

### 9.2 Planned Controls

- Phase 3: Validate provider base URLs against allowlist of known-good domains
- Phase 6: In sandbox mode, block all network except provider endpoints
- Always use HTTPS for provider API calls (enforced by reqwest)
- Provider API calls use `Bearer` auth only (no cookies, no redirects to third parties)

---

## 10. Input Validation

### 10.1 Current State

No input validation on tool arguments.

### 10.2 Planned Controls (Phase 2)

- JSON schema validation for tool arguments (LLM-provided args validated against declared schema)
- Path traversal prevention: reject `../` sequences in file paths
- Command injection prevention: reject shell metacharacters in non-bash tools
- Size limits on all tool arguments

---

## 11. Security Checklist by Phase

### Phase 1 (Must-have before calling Phase 1 complete):
- [x] Approval gate for bash tool ✅
- [x] Approval gate for write_file tool ✅
- [x] Approval gate for call_subagent tool ✅
- [ ] Warn if config file is world-readable
- [ ] Confirm scanning before learn_about_me

### Phase 2:
- [ ] Command blocklist
- [ ] Path write blocklist
- [ ] Path read blocklist
- [ ] Audit log (JSONL)
- [ ] Secret redaction in logs
- [ ] Input size limits

### Phase 3:
- [ ] API key env-var reference support
- [ ] Provider URL validation

### Phase 4:
- [ ] Memory scan approval per directory
- [ ] Never index secrets from files

### Phase 5:
- [ ] External agent approval + isolation
- [ ] External agent timeout enforcement

### Phase 6:
- [ ] MCP server identity controls
- [ ] Sandbox mode (path-based)

### Phase 7+:
- [ ] Full seccomp sandboxing (Linux)
- [ ] Keyring API key storage
