import re

with open('src/command_registry.rs', 'r') as f:
    content = f.read()

# Add to CommandCategory enum
content = content.replace(
    "    System,\n    /// Commands planned for future phases — not yet implemented.\n    Future,",
    "    System,\n    Hooks,\n    Scheduler,\n    Jobs,\n    /// Commands planned for future phases — not yet implemented.\n    Future,"
)

# Add to CommandCategory::label
content = content.replace(
    "            CommandCategory::System => \"System\",\n            CommandCategory::Future => \"Future (Planned)\",",
    "            CommandCategory::System => \"System\",\n            CommandCategory::Hooks => \"Hooks\",\n            CommandCategory::Scheduler => \"Scheduler\",\n            CommandCategory::Jobs => \"Jobs\",\n            CommandCategory::Future => \"Future (Planned)\","
)

# Add to CommandCategory::emoji
content = content.replace(
    "            CommandCategory::System => \"⚙\",\n            CommandCategory::Future => \"🔮\",",
    "            CommandCategory::System => \"⚙\",\n            CommandCategory::Hooks => \"🪝\",\n            CommandCategory::Scheduler => \"⏱\",\n            CommandCategory::Jobs => \"🏗\",\n            CommandCategory::Future => \"🔮\","
)

# Replace /hooks metadata
content = re.sub(
    r'        CommandMetadata \{\s+name: "/hooks",\s+aliases: &\[\],\s+category: CommandCategory::Future,\s+description: "Configure lifecycle hooks \(on-submit, on-tool-call, etc\.\)",\s+usage: "/hooks \[list\|add\|remove\]",\s+examples: &\["/hooks list"\],\s+shortcut: None,\s+surface: CommandSurface::both\(\),\s+requires_approval: true,\s+risk: CommandRisk::High,\s+status: CommandStatus::Planned,\s+related: None,\s+\},',
    '''        CommandMetadata {
            name: "/hooks",
            aliases: &[],
            category: CommandCategory::Hooks,
            description: "Configure lifecycle hooks (on-submit, on-tool-call, etc.)",
            usage: "/hooks [list|show|enable|disable|run]",
            examples: &["/hooks list", "/hooks run format-after-patch"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::Medium,
            status: CommandStatus::Partial,
            related: None,
        },''',
    content,
    flags=re.MULTILINE
)

# Replace /schedule metadata
content = re.sub(
    r'        CommandMetadata \{\s+name: "/schedule",\s+aliases: &\[\],\s+category: CommandCategory::Future,\s+description: "Schedule background tasks or reminders",\s+usage: "/schedule <cron> <task>",\s+examples: &\["/schedule \x270 9 \* \* 1\x27 run tests"\],\s+shortcut: None,\s+surface: CommandSurface::both\(\),\s+requires_approval: true,\s+risk: CommandRisk::High,\s+status: CommandStatus::Planned,\s+related: None,\s+\},',
    '''        CommandMetadata {
            name: "/schedule",
            aliases: &[],
            category: CommandCategory::Scheduler,
            description: "Schedule background tasks or reminders",
            usage: "/schedule [list|add|show|enable|disable|run|delete]",
            examples: &["/schedule list", "/schedule add interval 5 my_task test"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::Medium,
            status: CommandStatus::Partial,
            related: None,
        },''',
    content,
    flags=re.MULTILINE
)

# Replace /jobs metadata
content = re.sub(
    r'        CommandMetadata \{\s+name: "/jobs",\s+aliases: &\[\],\s+category: CommandCategory::Future,\s+description: "View and manage background jobs",\s+usage: "/jobs",\s+examples: &\["/jobs"\],\s+shortcut: None,\s+surface: CommandSurface::both\(\),\s+requires_approval: false,\s+risk: CommandRisk::None,\s+status: CommandStatus::Planned,\s+related: None,\s+\},',
    '''        CommandMetadata {
            name: "/jobs",
            aliases: &[],
            category: CommandCategory::Jobs,
            description: "View and manage background jobs",
            usage: "/jobs [list|show|cancel|logs]",
            examples: &["/jobs list", "/jobs show 1234"],
            shortcut: None,
            surface: CommandSurface::both(),
            requires_approval: false,
            risk: CommandRisk::None,
            status: CommandStatus::Partial,
            related: None,
        },''',
    content,
    flags=re.MULTILINE
)

with open('src/command_registry.rs', 'w') as f:
    f.write(content)
