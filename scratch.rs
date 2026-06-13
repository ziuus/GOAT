#[derive(Debug, Clone)]
pub enum SkillStepType {
    InspectFile,
    ProposePatch,
    ApplyPatch,
    RunValidation,
    RecordNote,
    AskUser,
    ManualStep,
}

#[derive(Debug, Clone)]
pub struct SkillStep {
    pub step_type: SkillStepType,
    pub description: String,
    pub command: Option<String>,
}

pub fn parse_steps(skill_content: &str) -> Vec<SkillStep> {
    let mut steps = Vec::new();
    let mut current_step_type = SkillStepType::ManualStep;
    let mut current_desc = String::new();
    let mut current_cmd = None;
    let mut in_code_block = false;
    
    for line in skill_content.lines() {
        let line = line.trim();
        if line.starts_with("```bash") || line.starts_with("```sh") {
            in_code_block = true;
            current_cmd = Some(String::new());
        } else if line.starts_with("```") && in_code_block {
            in_code_block = false;
            if let Some(cmd) = &mut current_cmd {
                *cmd = cmd.trim().to_string();
            }
        } else if in_code_block {
            if let Some(cmd) = &mut current_cmd {
                cmd.push_str(line);
                cmd.push('\n');
            }
        } else if line.starts_with("- ") || line.starts_with("* ") {
            if !current_desc.is_empty() {
                steps.push(SkillStep {
                    step_type: current_step_type.clone(),
                    description: current_desc.clone(),
                    command: current_cmd.take(),
                });
            }
            let text = line[2..].to_string();
            current_step_type = if text.to_lowercase().contains("inspect") || text.to_lowercase().contains("read") {
                SkillStepType::InspectFile
            } else if text.to_lowercase().contains("patch") {
                SkillStepType::ProposePatch
            } else if text.to_lowercase().contains("validate") || text.to_lowercase().contains("test") {
                SkillStepType::RunValidation
            } else if text.to_lowercase().contains("ask") {
                SkillStepType::AskUser
            } else if text.to_lowercase().contains("note") {
                SkillStepType::RecordNote
            } else {
                SkillStepType::ManualStep
            };
            current_desc = text;
        }
    }
    
    if !current_desc.is_empty() {
        steps.push(SkillStep {
            step_type: current_step_type,
            description: current_desc,
            command: current_cmd,
        });
    }
    
    if steps.is_empty() {
        steps.push(SkillStep {
            step_type: SkillStepType::ManualStep,
            description: "Execute the skill as described in the documentation.".to_string(),
            command: None,
        });
    }
    
    steps
}

fn main() {
    let content = std::fs::read_to_string("/home/zius/.local/share/goat/skills/test-skill/SKILL.md").unwrap();
    let steps = parse_steps(&content);
    println!("{:#?}", steps);
}
