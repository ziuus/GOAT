import re

with open("src/cli.rs", "r") as f:
    content = f.read()

# Find the first Command::Patch
content = re.sub(
    r"        Command::Patch { action, args: _ } => {\n            handle_patch_command\(action\);\n            Ok\(true\)\n        }",
    "",
    content,
    count=1
)

content = re.sub(
    r"        Command::CheckpointCmd { action, args: _ } => {.*?\n        }",
    "",
    content,
    flags=re.DOTALL,
    count=1
)

with open("src/cli.rs", "w") as f:
    f.write(content)
