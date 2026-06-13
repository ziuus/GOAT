use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectIntelligence {
    pub project_id: String,
    pub name: String,
    pub root_path: PathBuf,
    pub detected_stack: Vec<String>,
    pub frameworks: Vec<String>,
    pub package_managers: Vec<String>,
    pub languages: Vec<String>,
    pub important_files: Vec<String>,
    pub available_commands: Vec<String>,
    pub test_commands: Vec<String>,
    pub build_commands: Vec<String>,
    pub dev_commands: Vec<String>,
    pub lint_commands: Vec<String>,
    pub deploy_commands: Vec<String>,
    pub readme_summary: String,
    pub architecture_summary: String,
    pub last_scanned_at: u64,
    pub scan_status: String,
    pub risk_notes: Vec<String>,
    pub ignored_paths: Vec<String>,
    pub linked_missions: Vec<String>,
}

pub struct ProjectIntelligenceManager {
    storage_dir: PathBuf,
}

impl ProjectIntelligenceManager {
    pub fn new() -> Self {
        let storage_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("goat")
            .join("projects");
        fs::create_dir_all(&storage_dir).ok();
        Self { storage_dir }
    }

    pub fn get_projects(&self) -> Vec<ProjectIntelligence> {
        let mut projects = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.storage_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |ext| ext == "json") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(proj) = serde_json::from_str::<ProjectIntelligence>(&content) {
                            projects.push(proj);
                        }
                    }
                }
            }
        }
        projects.sort_by_key(|p| std::cmp::Reverse(p.last_scanned_at));
        projects
    }

    pub fn get_project(&self, id: &str) -> Option<ProjectIntelligence> {
        let path = self.storage_dir.join(format!("{}.json", id));
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(proj) = serde_json::from_str::<ProjectIntelligence>(&content) {
                    return Some(proj);
                }
            }
        }
        None
    }

    pub fn get_project_by_path(&self, root_path: &Path) -> Option<ProjectIntelligence> {
        let canonical = root_path
            .canonicalize()
            .unwrap_or_else(|_| root_path.to_path_buf());
        self.get_projects()
            .into_iter()
            .find(|p| p.root_path == canonical)
    }

    pub fn save_project(&self, proj: &ProjectIntelligence) -> Result<()> {
        let path = self.storage_dir.join(format!("{}.json", proj.project_id));
        let content = serde_json::to_string_pretty(proj)?;
        fs::write(path, content)?;
        Ok(())
    }
}

pub struct DeepProjectScanner {
    root: PathBuf,
}

impl DeepProjectScanner {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn scan(&self) -> Result<ProjectIntelligence> {
        let canonical_root = self
            .root
            .canonicalize()
            .unwrap_or_else(|_| self.root.clone());
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(canonical_root.to_string_lossy().as_bytes());
        let project_id = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()[..12]
            .to_string();
        let name = canonical_root
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut pi = ProjectIntelligence {
            project_id,
            name,
            root_path: canonical_root.clone(),
            detected_stack: Vec::new(),
            frameworks: Vec::new(),
            package_managers: Vec::new(),
            languages: Vec::new(),
            important_files: Vec::new(),
            available_commands: Vec::new(),
            test_commands: Vec::new(),
            build_commands: Vec::new(),
            dev_commands: Vec::new(),
            lint_commands: Vec::new(),
            deploy_commands: Vec::new(),
            readme_summary: "No README detected.".to_string(),
            architecture_summary: "Unknown architecture.".to_string(),
            last_scanned_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            scan_status: "success".to_string(),
            risk_notes: Vec::new(),
            ignored_paths: Vec::new(),
            linked_missions: Vec::new(),
        };

        // Ignore limits
        let known_ignore_dirs = [
            ".git",
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
            "coverage",
            "bin",
            "obj",
        ];

        let sensitive_files = [
            ".env",
            ".env.local",
            ".env.development",
            ".env.production",
            "id_rsa",
            "id_ed25519",
            "secrets.json",
            "credentials.json",
            "key.pem",
        ];

        let mut has_rust = false;
        let mut has_node = false;
        let mut has_python = false;
        let mut has_docker = false;
        let mut has_tauri = false;
        let mut has_github_actions = false;

        // Traverse root depth 1-2 to find important files and paths safely
        if let Ok(entries) = fs::read_dir(&canonical_root) {
            for entry in entries.flatten() {
                let fname = entry.file_name().to_string_lossy().to_string();
                let path = entry.path();

                if path.is_dir() {
                    if known_ignore_dirs.contains(&fname.as_str()) {
                        pi.ignored_paths.push(fname.clone());
                        continue;
                    }
                    if fname == ".github" {
                        if path.join("workflows").exists() {
                            has_github_actions = true;
                        }
                    }
                    if fname == "src-tauri" {
                        has_tauri = true;
                    }
                } else if path.is_file() {
                    // Sensitive filtering
                    if sensitive_files.iter().any(|s| fname.starts_with(s))
                        || fname.ends_with(".pem")
                        || fname.ends_with(".key")
                    {
                        pi.ignored_paths.push(fname.clone());
                        pi.risk_notes
                            .push(format!("Ignored sensitive file: {}", fname));
                        continue;
                    }

                    let file_size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                    if file_size > 1024 * 1024 {
                        // 1MB limit for safety
                        pi.ignored_paths.push(fname.clone());
                        continue;
                    }

                    match fname.as_str() {
                        "Cargo.toml" => {
                            has_rust = true;
                            pi.package_managers.push("Cargo".to_string());
                            pi.important_files.push(fname.clone());
                        }
                        "package.json" => {
                            has_node = true;
                            pi.package_managers.push("npm/yarn/pnpm".to_string());
                            pi.important_files.push(fname.clone());

                            // Parse scripts deterministically
                            if let Ok(content) = fs::read_to_string(&path) {
                                if let Ok(json) =
                                    serde_json::from_str::<serde_json::Value>(&content)
                                {
                                    if let Some(scripts) =
                                        json.get("scripts").and_then(|s| s.as_object())
                                    {
                                        for k in scripts.keys() {
                                            pi.available_commands.push(format!("npm run {}", k));
                                            if k.contains("test") {
                                                pi.test_commands.push(format!("npm run {}", k));
                                            }
                                            if k.contains("build") {
                                                pi.build_commands.push(format!("npm run {}", k));
                                            }
                                            if k.contains("dev") || k.contains("start") {
                                                pi.dev_commands.push(format!("npm run {}", k));
                                            }
                                            if k.contains("lint") {
                                                pi.lint_commands.push(format!("npm run {}", k));
                                            }
                                            if k.contains("deploy") {
                                                pi.deploy_commands.push(format!("npm run {}", k));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        "pyproject.toml" | "requirements.txt" => {
                            has_python = true;
                            pi.package_managers.push("pip/poetry".to_string());
                            pi.important_files.push(fname.clone());
                        }
                        "next.config.js" | "next.config.mjs" => {
                            pi.frameworks.push("Next.js".to_string());
                            pi.important_files.push(fname.clone());
                        }
                        "vite.config.js" | "vite.config.ts" => {
                            pi.frameworks.push("Vite".to_string());
                            pi.important_files.push(fname.clone());
                        }
                        "Dockerfile" | "docker-compose.yml" => {
                            has_docker = true;
                            pi.important_files.push(fname.clone());
                        }
                        "README.md" | "readme.md" => {
                            pi.important_files.push(fname.clone());
                            pi.readme_summary = "README detected.".to_string();
                        }
                        _ => {
                            if fname.ends_with(".rs") {
                                has_rust = true;
                            }
                            if fname.ends_with(".js") || fname.ends_with(".ts") {
                                has_node = true;
                            }
                            if fname.ends_with(".py") {
                                has_python = true;
                            }
                        }
                    }
                }
            }
        }

        if has_rust {
            pi.languages.push("Rust".to_string());
            pi.detected_stack.push("Rust".to_string());
            pi.build_commands.push("cargo build".to_string());
            pi.test_commands.push("cargo test".to_string());
            pi.dev_commands.push("cargo run".to_string());
            pi.available_commands.push("cargo check".to_string());
        }
        if has_node {
            pi.languages.push("JavaScript/TypeScript".to_string());
            pi.detected_stack.push("Node.js".to_string());
        }
        if has_python {
            pi.languages.push("Python".to_string());
            pi.detected_stack.push("Python".to_string());
        }
        if has_docker {
            pi.detected_stack.push("Docker".to_string());
        }
        if has_tauri {
            pi.frameworks.push("Tauri".to_string());
            pi.detected_stack.push("Tauri".to_string());
            pi.dev_commands.push("cargo tauri dev".to_string());
            pi.build_commands.push("cargo tauri build".to_string());
        }
        if has_github_actions {
            pi.detected_stack.push("GitHub Actions".to_string());
        }

        pi.package_managers.dedup();
        pi.languages.dedup();
        pi.frameworks.dedup();
        pi.detected_stack.dedup();
        pi.important_files.sort();

        let arch = if !pi.frameworks.is_empty() {
            format!(
                "This appears to be a {} project using {}.",
                pi.languages.join("/"),
                pi.frameworks.join(" + ")
            )
        } else if !pi.languages.is_empty() {
            format!("This appears to be a {} project.", pi.languages.join("/"))
        } else {
            "Unknown architecture.".to_string()
        };
        pi.architecture_summary = arch;

        Ok(pi)
    }
}
