# GOAT Builder Repository Inspection

Repository inspection allows the Builder Agent to understand project configuration, structure, and active files without reading massive directories or exposing credentials.

## Inspection Model
- **Snapshot:** Holds root path, language summary, package configurations, source directories, test suites, and git status.
- **Ignore Rules:** Enforces skipping of large directories like `.git`, `node_modules`, `target`, `.next`, and vendor files.
- **Risk Files:** Scans for `.env` or configuration files containing credentials and prevents reading their contents.
