use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectProfileKind {
    RustCli,
    RustTauri,
    NextjsDashboard,
    FullStackWebApp,
    PythonMl,
    NodeBackend,
    MobileApp,
    DocsSite,
    Unknown,
}

impl Default for ProjectProfileKind {
    fn default() -> Self {
        ProjectProfileKind::Unknown
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectSetupChecklist {
    pub github_linked: bool,
    pub mcp_tools_configured: bool,
    pub index_built: bool,
    pub environment_variables_set: bool,
}

impl Default for ProjectSetupChecklist {
    fn default() -> Self {
        Self {
            github_linked: false,
            mcp_tools_configured: false,
            index_built: false,
            environment_variables_set: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectProfile {
    pub kind: ProjectProfileKind,
    pub project_root: String,
    pub build_command: Option<String>,
    pub test_command: Option<String>,
    pub lint_command: Option<String>,
    pub preferred_mode_profiles: Vec<String>,
    pub recommended_skill_packs: Vec<String>,
    pub recommended_recipes: Vec<String>,
    pub recommended_browser_qa: bool,
    pub checklist: ProjectSetupChecklist,
}

impl Default for ProjectProfile {
    fn default() -> Self {
        Self {
            kind: ProjectProfileKind::Unknown,
            project_root: ".".into(),
            build_command: None,
            test_command: None,
            lint_command: None,
            preferred_mode_profiles: vec![],
            recommended_skill_packs: vec![],
            recommended_recipes: vec![],
            recommended_browser_qa: false,
            checklist: ProjectSetupChecklist::default(),
        }
    }
}

pub struct ProjectProfileDetector;

impl ProjectProfileDetector {
    pub fn detect(project_root: &str) -> ProjectProfile {
        let mut profile = ProjectProfile {
            project_root: project_root.into(),
            ..Default::default()
        };

        let root = std::path::Path::new(project_root);
        let has_cargo = root.join("Cargo.toml").exists();
        let has_package = root.join("package.json").exists();
        let has_next =
            root.join("next.config.js").exists() || root.join("next.config.mjs").exists();
        let has_tauri = root.join("src-tauri").exists();
        let has_python =
            root.join("requirements.txt").exists() || root.join("pyproject.toml").exists();

        if has_cargo && has_tauri {
            profile.kind = ProjectProfileKind::RustTauri;
            profile.build_command = Some("cargo tauri build".into());
            profile.test_command = Some("cargo test".into());
            profile.lint_command = Some("cargo clippy".into());
            profile.preferred_mode_profiles =
                vec!["Coding Assistant".into(), "UI/UX Designer".into()];
            profile.recommended_skill_packs =
                vec!["Rust CLI Pack".into(), "antigravity-master-ui".into()];
            profile.recommended_recipes = vec!["cargo-check-on-save".into()];
        } else if has_cargo {
            profile.kind = ProjectProfileKind::RustCli;
            profile.build_command = Some("cargo build".into());
            profile.test_command = Some("cargo test".into());
            profile.lint_command = Some("cargo clippy".into());
            profile.preferred_mode_profiles = vec!["Coding Assistant".into(), "Test Runner".into()];
            profile.recommended_skill_packs = vec!["Rust CLI Pack".into()];
            profile.recommended_recipes = vec!["cargo-check-on-save".into()];
        } else if has_package && has_next {
            profile.kind = ProjectProfileKind::NextjsDashboard;
            profile.build_command = Some("npm run build".into());
            profile.test_command = Some("npm run test".into());
            profile.lint_command = Some("npm run lint".into());
            profile.preferred_mode_profiles = vec![
                "Coding Assistant".into(),
                "UI/UX Designer".into(),
                "Browser QA".into(),
            ];
            profile.recommended_skill_packs = vec!["antigravity-master-ui".into()];
            profile.recommended_recipes = vec!["browser-qa".into()];
            profile.recommended_browser_qa = true;
        } else if has_python {
            profile.kind = ProjectProfileKind::PythonMl;
            profile.build_command = None;
            profile.test_command = Some("pytest".into());
            profile.lint_command = Some("flake8".into());
            profile.preferred_mode_profiles = vec!["Coding Assistant".into(), "Researcher".into()];
            profile.recommended_skill_packs = vec!["Python Data Pack".into()];
            profile.recommended_recipes = vec![];
        } else if has_package {
            profile.kind = ProjectProfileKind::NodeBackend;
            profile.build_command = Some("npm run build".into());
            profile.test_command = Some("npm run test".into());
            profile.lint_command = Some("npm run lint".into());
            profile.preferred_mode_profiles = vec!["Coding Assistant".into()];
            profile.recommended_skill_packs = vec![];
        }

        profile
    }
}
