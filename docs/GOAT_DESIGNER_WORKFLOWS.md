# GOAT Designer Agent Workflows

The Designer Agent uses strict workflows to systematically critique and improve user interfaces. 
These workflows are built to complement the Builder agent, rather than overwrite code autonomously.

## 1. Initialization and Auditing

The workflow begins by registering the interface to be evaluated.

**Command:**
`/designer review <url_or_path> [description]`

**Process:**
1. The Designer agent fetches context about the target. (If it's a URL, it may use the Browser integration. If it's a path, it reads the files.)
2. It transitions to the `Pending` state.
3. The developer can ask for a full scorecard.

## 2. Generating the Scorecard

**Command:**
`/designer score <review_id>`

**Process:**
1. The Designer uses an LLM-based design prompt to grade the target out of 5 across:
   - Aesthetics
   - Usability
   - Performance
   - Accessibility
2. The agent transitions to the `ScorecardGenerated` state.

## 3. Targeted Checks

The Designer performs targeted checks on accessibility and responsiveness.

**Commands:**
- `/designer accessibility <review_id>`
- `/designer responsive <review_id>`

**Process:**
1. The agent checks for WCAG compliance (contrast, ARIA labels, interactive states).
2. The agent checks for mobile-first CSS strategies and appropriate breakpoint handling.
3. It aggregates the results and transitions to the `AccessibilityChecked` and `ResponsiveChecked` states.

## 4. Improvement Planning

Once issues have been identified, the Designer creates a structured plan.

**Command:**
`/designer plan <review_id>`

**Process:**
1. The agent compiles all issues from the scorecard and targeted checks.
2. It builds a prioritized checklist of actionable improvements.
3. It transitions to the `PlanReady` state.

## 5. Builder Handoff

The Designer is explicitly *not* designed to rewrite code unsupervised. Instead, it hands off its plan to a human or the Builder Agent.

**Command:**
`/designer handoff <review_id>`

**Process:**
1. The Designer produces a `HandoffBrief` containing the exact files to edit, the CSS classes to update, and the structural changes needed.
2. The Handoff Brief can be read by the Builder Agent as part of its task execution.
3. The workflow transitions to `HandoffReady`.

## 6. Report Generation

At any point after a scorecard is generated, the Designer can write a formal markdown report.

**Command:**
`/designer report <review_id>`

**Process:**
1. The Designer serializes the review, scorecard, issues, and handoff brief into a comprehensive markdown document.
2. The document is saved to the `GOAT` reporting system.
3. The report ID is returned to the user.
