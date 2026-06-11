pub struct QuickAccessParser;

impl QuickAccessParser {
    pub fn parse_and_rewrite(input: &str) -> String {
        let input = input.trim_start();
        if input.is_empty() {
            return input.to_string();
        }

        let first_char = input.chars().next().unwrap();

        let rewritten = match first_char {
            '@' => {
                let rest = &input[1..];
                let parts: Vec<&str> = rest.splitn(2, ' ').collect();
                let cmd = parts[0];
                let args = if parts.len() > 1 { parts[1] } else { "" };

                match cmd {
                    "agents" => {
                        if args.is_empty() {
                            "/agents list".to_string()
                        } else {
                            format!("/agents {}", args)
                        }
                    }
                    "prime" => format!("/agents list"), // or something equivalent
                    "specialists" => {
                        if args.is_empty() {
                            "/agents list".to_string()
                        } else {
                            format!("/agents specialists {}", args)
                        }
                    }
                    "cofounder" => {
                        if args.is_empty() {
                            "/cofounder list".to_string()
                        } else {
                            format!("/cofounder {}", args)
                        }
                    }
                    "socializer" | "designer" | "researcher" | "builder" | "operator"
                    | "learner" | "finance" | "reddit" | "ui" | "reviewer" => {
                        if args.is_empty() {
                            format!("/agents show {}", cmd)
                        } else {
                            // If they type a message, maybe suggest they enable the agent
                            format!("/agents show {}", cmd)
                        }
                    }
                    _ => format!("/subagents ask {}", rest),
                }
            }
            '#' => {
                let rest = &input[1..];
                let parts: Vec<&str> = rest.splitn(2, ' ').collect();
                let cmd = parts[0];
                let args = if parts.len() > 1 { parts[1] } else { "" };

                match cmd {
                    "suggest" | "attach" | "detach" | "session" | "clear" | "promote"
                    | "sources" | "quota" => {
                        format!("/skills {} {}", cmd, args)
                    }
                    "pack" => format!("/skills pack {}", args),
                    "research" => format!("/skill-research {}", args),
                    _ => format!("/skills {}", rest), // Fallback
                }
            }
            '~' => {
                let rest = &input[1..];
                let parts: Vec<&str> = rest.splitn(2, ' ').collect();
                let cmd = parts[0];
                let args = if parts.len() > 1 { parts[1] } else { "" };

                match cmd {
                    "recall" => format!("/recall {}", args),
                    "search" => format!("/brain search {}", args),
                    "timeline" | "recent" | "today" | "yesterday" | "replay" | "history"
                    | "what-did-we-do" => {
                        if args.is_empty() {
                            format!("/timeline {}", cmd)
                        } else {
                            format!("/timeline {} {}", cmd, args)
                        }
                    }
                    _ => format!("/recall {}", rest),
                }
            }
            '|' => {
                let rest = &input[1..];
                let parts: Vec<&str> = rest.splitn(2, ' ').collect();
                let cmd = parts[0];
                let args = if parts.len() > 1 { parts[1] } else { "" };

                match cmd {
                    "recipes" => format!("/recipes"),
                    "run" => format!("/recipes run {}", args),
                    "plan" => format!("/recipes plan {}", args),
                    "activate" => format!("/hooks activate {}", args),
                    "workflow" => format!("/recipes {}", args),
                    _ => format!("/recipes {}", rest),
                }
            }
            '\\' => {
                let rest = &input[1..];
                let parts: Vec<&str> = rest.splitn(2, ' ').collect();
                let cmd = parts[0];
                let args = if parts.len() > 1 { parts[1] } else { "" };

                match cmd {
                    "model" => format!("/model {}", args),
                    "profile" => format!("/profile {}", args),
                    "layout" => format!("/layout {}", args),
                    "session" => format!("/session {}", args),
                    "context" => format!("/context {}", args),
                    "quota" => format!("/quota {}", args),
                    "history" | "timeline" => format!("/timeline {}", args),
                    "gh" | "github" => format!("/github {}", args),
                    _ => format!("/session {}", rest),
                }
            }
            '/' => input.to_string(), // already a slash command
            _ => input.to_string(),   // not a quick prefix
        };

        rewritten.trim_end().to_string()
    }
}
