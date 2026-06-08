#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubagentKind {
    Coder,
    Browser,
    Researcher,
    General,
}

#[derive(Clone, Debug)]
pub struct SubagentProfile {
    pub kind: SubagentKind,
    pub name: &'static str,
    pub provider: &'static str,
    pub model: &'static str,
    pub system_prompt: &'static str,
}

#[derive(Clone, Debug)]
pub struct RouteDecision {
    pub profile: SubagentProfile,
    pub confidence: u8,
    pub reason: String,
}

#[derive(Default)]
pub struct SwarmRouter;

impl SwarmRouter {
    pub fn route(&self, input: &str) -> RouteDecision {
        let normalized = input.to_lowercase();

        if contains_any(
            &normalized,
            &[
                "browser",
                "website",
                "webpage",
                "click",
                "screenshot",
                "scrape",
                "login",
                "navigate",
                "research online",
            ],
        ) {
            return RouteDecision {
                profile: browser_profile(),
                confidence: 88,
                reason: "browser or web automation intent detected".to_string(),
            };
        }

        if contains_any(
            &normalized,
            &[
                "code",
                "fix",
                "bug",
                "compile",
                "test",
                "refactor",
                "cargo",
                "typescript",
                "rust",
                "python",
                "repo",
                "commit",
                "pull request",
            ],
        ) {
            return RouteDecision {
                profile: coder_profile(),
                confidence: 86,
                reason: "coding or repository task intent detected".to_string(),
            };
        }

        if contains_any(
            &normalized,
            &[
                "research",
                "compare",
                "find",
                "source",
                "paper",
                "latest",
                "market",
                "audit",
                "analyze options",
            ],
        ) {
            return RouteDecision {
                profile: researcher_profile(),
                confidence: 80,
                reason: "research and synthesis intent detected".to_string(),
            };
        }

        RouteDecision {
            profile: general_profile(),
            confidence: 60,
            reason: "no specialized intent crossed routing threshold".to_string(),
        }
    }
}

fn contains_any(input: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| input.contains(needle))
}

fn coder_profile() -> SubagentProfile {
    SubagentProfile {
        kind: SubagentKind::Coder,
        name: "Coder",
        provider: "openai",
        model: "gpt-4o-mini",
        system_prompt: "You are GOAT Coder, a precise software engineering subagent. Prefer minimal correct edits, verify changes, and preserve user work.",
    }
}

fn browser_profile() -> SubagentProfile {
    SubagentProfile {
        kind: SubagentKind::Browser,
        name: "Browser",
        provider: "openai",
        model: "gpt-4o-mini",
        system_prompt: "You are GOAT Browser, a web and desktop automation subagent. Use browser and GUI tools carefully, report observable outcomes, and avoid irreversible actions without approval.",
    }
}

fn researcher_profile() -> SubagentProfile {
    SubagentProfile {
        kind: SubagentKind::Researcher,
        name: "Researcher",
        provider: "openai",
        model: "gpt-4o-mini",
        system_prompt: "You are GOAT Researcher, a source-aware investigation subagent. Compare evidence, separate facts from assumptions, and return concise actionable findings.",
    }
}

fn general_profile() -> SubagentProfile {
    SubagentProfile {
        kind: SubagentKind::General,
        name: "General",
        provider: "openai",
        model: "gpt-4o-mini",
        system_prompt: "You are GOAT General, a practical personal AI coordinator. Clarify ambiguity only when necessary and route work to stronger specialized tools when useful.",
    }
}
