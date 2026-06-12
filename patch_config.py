import re

with open("src/api_server.rs", "r") as f:
    content = f.read()

content = content.replace(
"""    let config = crate::config::Config::load_from_path(config_path).unwrap_or_default().config;""",
"""    let config = crate::config::Config::load_from_path(config_path).map(|r| r.config).unwrap_or_else(|_| crate::config::Config::default());"""
)

with open("src/api_server.rs", "w") as f:
    f.write(content)

