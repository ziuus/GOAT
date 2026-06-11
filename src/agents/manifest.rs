use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgentTier {
    Prime,
    Specialist,
    Subagent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Planned,
    Experimental,
    Active,
    Disabled,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentManifest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tier: AgentTier,
    pub status: AgentStatus,
    #[serde(default)]
    pub prime_affinity: Option<String>,
    #[serde(default)]
    pub traits: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
}
