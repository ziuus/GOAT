# GOAT Builder Tool-Use Policy

This document defines the strict policy governing how the Builder agent selects, executes, and requests authorization for tools in the GOAT runtime environment.

## 1. Targeted Tool Selection over Shell Commands
Generic shell executions (`bash`) are extremely powerful but present high risks and high cognitive overhead for review. 
The Builder agent **MUST** prioritize targeted tools over shell commands where possible:

- **Filesystem Actions:** Prefer using `read_file` instead of `cat` or `head`. Prefer using `write_file` instead of `echo "..." > file`.
- **Search & Inspection:** Prefer `grep_search` and structured directory listing utilities over raw shell equivalents (`grep`, `find`, `ls`).
- **Dependencies:** Never run `npm install` or `cargo add` inside a shell unless it has been explicitly analyzed and approved.

## 2. Risky Operations & Explanations
Before calling any tool with a risk classification of **Medium**, **High**, or **Critical**, the Builder **MUST** write an internal explanation describing:
1. Why the operation is necessary.
2. The expected impact on files, compilation, or behavior.
3. The mitigation or rollback strategy if it fails.

## 3. Directory Traversal Discipline
To keep systems lightweight and prevent security/resource bloat:
- Never scan large directories blindly (e.g. `node_modules`, `target`, `.git`, `dist`).
- Respect defined ignore rules automatically.
- Read only relevant portions of files when searching for specific symbols or definitions.
