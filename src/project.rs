use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub root_path: PathBuf,
    pub is_git_repo: bool,
    pub stack: Vec<String>,
    pub package_files: Vec<String>,
    pub source_dirs: Vec<String>,
    pub ignored_dirs_count: usize,
    pub detected_commands: Vec<String>,
    pub last_scan: SystemTime,
}

pub struct ProjectScanner {
    root: PathBuf,
}

impl ProjectScanner {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn scan(&self) -> Result<ProjectMetadata> {
        let is_git_repo = self.root.join(".git").exists();
        
        let mut stack = Vec::new();
        let mut package_files = Vec::new();
        let mut detected_commands = Vec::new();

        // 1. Package files & tech stack
        let known_packages = vec![
            ("Cargo.toml", "Rust"),
            ("package.json", "Node/JS"),
            ("pnpm-lock.yaml", "pnpm"),
            ("yarn.lock", "Yarn"),
            ("package-lock.json", "npm"),
            ("pyproject.toml", "Python"),
            ("requirements.txt", "Python"),
            ("go.mod", "Go"),
            ("pom.xml", "Java/Maven"),
            ("build.gradle", "Java/Gradle"),
            ("docker-compose.yml", "Docker"),
            ("Dockerfile", "Docker"),
            ("Makefile", "Make"),
        ];

        let mut has_package_json = false;

        for (filename, tech) in known_packages {
            if self.root.join(filename).exists() {
                package_files.push(filename.to_string());
                if !stack.contains(&tech.to_string()) {
                    stack.push(tech.to_string());
                }
                
                if filename == "Cargo.toml" {
                    detected_commands.push("cargo run".to_string());
                    detected_commands.push("cargo test".to_string());
                    detected_commands.push("cargo check".to_string());
                } else if filename == "package.json" {
                    has_package_json = true;
                } else if filename == "Makefile" {
                    detected_commands.push("make".to_string());
                }
            }
        }

        // Parse package.json for commands
        if has_package_json {
            let pkg_path = self.root.join("package.json");
            if let Ok(content) = fs::read_to_string(&pkg_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(scripts) = json.get("scripts").and_then(|s| s.as_object()) {
                        for script in scripts.keys() {
                            // Common dev commands
                            if ["dev", "build", "start", "test", "lint"].contains(&script.as_str()) {
                                detected_commands.push(format!("npm run {}", script));
                            }
                        }
                    }
                }
            }
        }

        // Fallback generic stack hints based on files if no package managers
        if stack.is_empty() {
            if self.root.join("main.py").exists() || self.root.join("app.py").exists() {
                stack.push("Python".to_string());
            }
            if self.root.join("main.go").exists() {
                stack.push("Go".to_string());
            }
        }

        // 2. Source directories
        let known_src_dirs = vec![
            "src", "app", "pages", "components", "lib", "crates", "packages", "tests", "docs"
        ];
        let mut source_dirs = Vec::new();
        
        for dir in known_src_dirs {
            let dir_path = self.root.join(dir);
            if dir_path.is_dir() {
                source_dirs.push(dir.to_string());
            }
        }

        // 3. Ignored directories count
        let known_ignore_dirs = vec![
            "node_modules", "target", "dist", "build", ".next", ".turbo", ".cache", "venv", ".venv", "__pycache__"
        ];
        let mut ignored_dirs_count = 0;
        
        for dir in known_ignore_dirs {
            let dir_path = self.root.join(dir);
            if dir_path.is_dir() {
                ignored_dirs_count += 1;
            }
        }

        Ok(ProjectMetadata {
            root_path: self.root.clone(),
            is_git_repo,
            stack,
            package_files,
            source_dirs,
            ignored_dirs_count,
            detected_commands,
            last_scan: SystemTime::now(),
        })
    }
}
