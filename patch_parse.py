import re

with open("src/skills.rs", "r") as f:
    content = f.read()

new_parse = """    fn parse_skill(&self, path: &Path, name: String) -> Result<Skill> {
        let content = fs::read_to_string(path).context("Failed to read skill file")?;

        let mut description = String::new();
        let mut triggers = String::new();
        let mut version = "0.1.0".to_string();
        let mut status = "active".to_string();
        let mut source = "manual".to_string();
        let mut source_mission_id = None;
        let mut risk_level = "low".to_string();
        
        let mut is_suspicious = false;
        let mut warnings = Vec::new();

        // Very basic markdown parsing
        let mut in_description = false;
        let mut in_triggers = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name:") && name.is_empty() {
                // name override
            } else if trimmed.starts_with("description:") {
                description = trimmed.replace("description:", "").trim().to_string();
            } else if trimmed.starts_with("version:") {
                version = trimmed.replace("version:", "").trim().to_string();
            } else if trimmed.starts_with("status:") {
                status = trimmed.replace("status:", "").trim().to_string();
            } else if trimmed.starts_with("source:") {
                source = trimmed.replace("source:", "").trim().to_string();
            } else if trimmed.starts_with("source_mission_id:") {
                source_mission_id = Some(trimmed.replace("source_mission_id:", "").trim().to_string());
            } else if trimmed.starts_with("risk_level:") {
                risk_level = trimmed.replace("risk_level:", "").trim().to_string();
            } else if trimmed.starts_with("## Description") {
                in_description = true;
                in_triggers = false;
                continue;
            } else if trimmed.starts_with("## Triggers") || trimmed.starts_with("## When to use") {
                in_description = false;
                in_triggers = true;
                continue;
            } else if trimmed.starts_with("## ") {
                in_description = false;
                in_triggers = false;
            }

            if in_description && !trimmed.is_empty() && description.is_empty() {
                description = trimmed.to_string();
            } else if in_triggers && !trimmed.is_empty() {
                if triggers.is_empty() {
                    triggers = trimmed.to_string();
                } else {
                    triggers.push(' ');
                    triggers.push_str(trimmed);
                }
            }
        }

        // Basic secret detection
        let lower_content = content.to_lowercase();
        if lower_content.contains("sk-") || lower_content.contains("gsk_") {
            is_suspicious = true;
            warnings.push("Suspicious API key pattern detected (sk- or gsk_)".to_string());
        }
        if lower_content.contains("rm -rf") {
            is_suspicious = true;
            warnings.push("Suspicious command: rm -rf".to_string());
        }
        if lower_content.contains("curl | sh") || lower_content.contains("wget -qO-") {
            is_suspicious = true;
            warnings.push("Suspicious command: curl | sh (arbitrary script execution)".to_string());
        }
        if lower_content.contains("sudo ") {
            is_suspicious = true;
            warnings.push("Suspicious command: sudo (privilege escalation)".to_string());
        }
        if lower_content.contains("password=") || lower_content.contains("secret=") {
            is_suspicious = true;
            warnings.push("Suspicious string: password= or secret=".to_string());
        }
        if lower_content.contains("-----begin rsa") || lower_content.contains("-----begin openssh")
        {
            is_suspicious = true;
            warnings.push("Suspicious string: RSA/SSH private key".to_string());
        }

        Ok(Skill {
            name,
            description,
            version,
            status,
            source,
            source_mission_id,
            risk_level,
            triggers,
            content,
            is_suspicious,
            warnings,
        })
    }"""

content = re.sub(r'    fn parse_skill.*?Ok\(Skill \{.*?warnings,\n        \}\)\n    \}', new_parse, content, flags=re.DOTALL)

with open("src/skills.rs", "w") as f:
    f.write(content)

