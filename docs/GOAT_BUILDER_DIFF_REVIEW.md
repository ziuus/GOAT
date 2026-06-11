# GOAT Builder Diff Review

Diff review runs after patch generation or code modifications to evaluate logical, import, or syntax regression.

## Review Metrics
- **Logical Bugs:** Identifying infinite loops, bad variables, or condition drifts.
- **Unsafe Calls:** Finding un-gated filesystem writes or unsafe system functions.
- **Severity Classifications:**
  - **Info / Low:** Stylistic or formatting suggestions.
  - **Medium:** Minor import issue or missing tests.
  - **High / Critical:** Logic regressions, build breaking imports, or credential exposure.
