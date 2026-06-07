use crate::brain::Brain;
use crate::llm::LlmRouter;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub running: bool,
    pub logs: Vec<String>,
    pub current_task: String,
    pub input: String,
    pub input_mode: InputMode,
    pub brain: Option<Brain>,
    pub llm_router: LlmRouter,
    pub mcp_client: Option<crate::mcp::McpClient>,
}

impl App {
    pub fn new() -> Self {
        let brain = Brain::new("goat_brain.db").ok();
        let mut logs = vec![
            "[SYSTEM] GOAT Engine Initialized.".to_string(),
            "[SYSTEM] Awaiting MCP connections...".to_string(),
        ];
        
        if brain.is_none() {
            logs.push("[ERROR] Failed to initialize Brain (SQLite).".to_string());
        } else {
            logs.push("[SYSTEM] Brain connected (SQLite).".to_string());
        }

        Self {
            running: true,
            logs,
            current_task: "Ready for mission...".to_string(),
            input: String::new(),
            input_mode: InputMode::Normal,
            brain,
            llm_router: LlmRouter::new(),
            mcp_client: None,
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
}
