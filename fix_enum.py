with open("src/cli.rs", "r") as f:
    lines = f.readlines()

new_lines = []
skip = False
for i, line in enumerate(lines):
    if "goat patch apply    → apply the pending patch" in line:
        # Start deleting backwards 3 lines
        new_lines = new_lines[:-3]
        skip = True
    if skip and "Rollback to a specific checkpoint" in line:
        skip = False
    
    if not skip:
        new_lines.append(line)

with open("src/cli.rs", "w") as f:
    f.writelines(new_lines)
