import re

with open("src/skills.rs", "r") as f:
    content = f.read()

# Add serde
if "use serde::" not in content:
    content = content.replace("use anyhow::{Context, Result};", "use anyhow::{Context, Result};\nuse serde::{Deserialize, Serialize};")

# Replace Skill struct
old_struct = """pub struct Skill {
    pub name: String,
    pub description: String,
    pub triggers: String,
    pub content: String,
    pub is_suspicious: bool,
    pub warnings: Vec<String>,
}"""

new_struct = """#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub version: String,
    pub status: String,
    pub source: String,
    pub source_mission_id: Option<String>,
    pub risk_level: String,
    pub triggers: String,
    #[serde(skip)]
    pub content: String,
    pub is_suspicious: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillIndex {
    pub updated_at: i64,
    pub skills: Vec<Skill>,
}"""

content = content.replace("#[derive(Debug, Clone)]\n" + old_struct, new_struct)
if "#[derive(Debug, Clone)]" in content and "pub struct Skill" in content:
    print("WARNING: Replacement may not have worked exactly")
    # manual replace
    content = re.sub(r'#\[derive\(Debug, Clone\)\]\npub struct Skill \{.*?\n\}', new_struct, content, flags=re.DOTALL)

with open("src/skills.rs", "w") as f:
    f.write(content)
