import re

with open("src/app.rs", "r") as f:
    content = f.read()

new_commands = """            "/skills" | "@skills" => {
                let paths = crate::paths::GoatPaths::resolve().unwrap();
                let config = crate::config::Config::load();
                let skill_manager = crate::skills::SkillManager::new(paths, config.skills);
                let skills = skill_manager.list_skills();
                if skills.is_empty() {
                    self.push_log("[SKILLS] No skills found.");
                } else {
                    self.push_log(format!("[SKILLS] {} skills available:", skills.len()));
                    for s in skills {
                        let status = if s.is_suspicious { "[SUSPICIOUS]" } else { "" };
                        self.push_log(format!("- {} v{} {}: {}", s.name, s.version, status, s.description));
                    }
                }
                true
            }
            "/save-skill" | "@save-skill" => {
                let manager = crate::mission_control::MissionControlManager::new();
                let missions = manager.get_missions();
                if let Some(m) = missions.first() {
                    let mission_id = m.mission_id.clone();
                    self.push_log(format!("[SKILLS] Run the following CLI command to save this mission as a skill:"));
                    let name = if _args.is_empty() { "new-skill" } else { _args.trim() };
                    self.push_log(format!("  goat skill create-from-mission {} --name {}", mission_id, name));
                } else {
                    self.push_log("[SKILLS] No active mission found to save.");
                }
                true
            }"""

if '"/skills" | "@skills"' not in content:
    content = content.replace('"/patch" | "/patches" | "@patch" => {', new_commands + '\n            "/patch" | "/patches" | "@patch" => {')

with open("src/app.rs", "w") as f:
    f.write(content)

