import re

with open("src/cli.rs", "r") as f:
    content = f.read()

new_action = """        "create-from-mission" => {
            let mission_id = arg.ok_or_else(|| anyhow::anyhow!("Expected mission ID"))?;
            let mission_manager = crate::mission_control::MissionControlManager::new();
            
            let mission = mission_manager.get_missions().into_iter().find(|m| m.mission_id == mission_id).ok_or_else(|| {
                anyhow::anyhow!("Mission '{}' not found", mission_id)
            })?;

            let name = name_arg.unwrap_or_else(|| "mission-skill").to_string();

            // Ask for confirmation
            println!("Do you want to save this mission as a reusable skill '{}'? (y/N)", name);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Skill creation cancelled.");
                return Ok(());
            }

            println!("Extracting skill '{}' from mission {}...", name, mission_id);

            let mut registry = crate::models::ProfileRegistry::from_config(&config.profiles);
            let mut router = crate::llm::LlmRouter::from_config(config);

            let profile_name = &registry.default_profile;
            let (_, chain) = registry.resolve(profile_name);

            let steps: Vec<String> = mission.plan_steps.iter().map(|s| format!("- {}: {}", s.title, s.description)).collect();
            let steps_str = steps.join("\n");

            let prompt = format!(
                "You are a skill curator. The user wants to extract a reusable skill from the following completed mission.\\n\\
                 Generate a valid SKILL.md file.\\n\\
                 The skill name should be: {}\\n\\
                 Mission Goal: {}\\n\\
                 Steps Executed:\\n{}\\n\\
                 Artifacts Produced: {}\\n\\n\\
                 Use the following format strictly:\\n\\
                 ---\\n\\
                 name: {}\\n\\
                 description: <short summary>\\n\\
                 version: 0.1.0\\n\\
                 status: active\\n\\
                 source: mission\\n\\
                 source_mission_id: {}\\n\\
                 risk_level: <low|medium|high>\\n\\
                 ---\\n\\n\\
                 # Skill: {}\\n\\n\\
                 ## When to use\\n\\
                 <triggers>\\n\\n\\
                 ## Required context\\n\\
                 <context>\\n\\n\\
                 ## Procedure\\n\\
                 <step by step>\\n\\n\\
                 ## Success criteria\\n\\
                 <criteria>\\n\\n\\
                 Output only the Markdown content.",
                name, mission.raw_goal, steps_str, mission.expected_artifacts.join(", "), name, mission_id, name
            );

            let messages = vec![crate::llm::Message {
                role: "user".to_string(),
                content: Some(prompt),
                tool_calls: None,
                tool_call_id: None,
            }];

            match router
                .completion_with_fallback(&chain, messages, None)
                .await
            {
                Ok((resp, _)) => {
                    let content = resp.content.unwrap_or_default();
                    let skill_dir = skill_manager.skills_dir().join(&name);
                    std::fs::create_dir_all(&skill_dir)?;
                    let skill_file = skill_dir.join("SKILL.md");
                    std::fs::write(&skill_file, content)?;
                    println!(
                        "Extracted and saved skill '{}' to {}",
                        name,
                        skill_file.display()
                    );
                    let _ = skill_manager.list_skills(); // updates index
                }
                Err(e) => anyhow::bail!("Failed to extract skill from LLM: {}", e),
            }
        }"""

content = content.replace('"create-from-session" => {', new_action + '\n        "create-from-session" => {')

# Also handle `new` properly
new_action_new = """        "new" => {
            let name = name_arg.or(arg).ok_or_else(|| anyhow::anyhow!("Expected skill name"))?;
            let path = skill_manager.create_template(name)?;
            println!("Created skill template at: {}", path.display());
            println!("Edit this file to implement your skill.");
            let _ = skill_manager.list_skills(); // update index
        }"""
content = content.replace('"create" => {', new_action_new + '\n        "create" => {')

with open("src/cli.rs", "w") as f:
    f.write(content)

