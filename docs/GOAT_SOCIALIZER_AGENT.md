# GOAT Socializer Prime Agent

The **Socializer** is a Phase 5 Prime Agent dedicated to community building, content strategy, and ethical distribution.

## Core Directives
1. **Ethical Distribution**: The Socializer never spams. It values platform etiquette, community norms, and relationship building over pure metrics.
2. **Draft-First**: The agent never posts automatically. All content, outreach plans, and strategies are generated as *drafts* for user review.
3. **Approval-First**: Any action that would cross the system boundary (e.g., using the Twitter API) must pass through `ApprovalGate`.
4. **Context-Aware**: The agent tailors content to the specific channel, audience, and phase of the project (e.g., Reddit vs. LinkedIn vs. X).

## Workflow States
- **Draft**: Initial campaign creation.
- **AudienceMapped**: Target segments, pain points, and gathering places identified.
- **ChannelStrategyDefined**: Platforms selected based on fit score and etiquette.
- **ContentDrafted**: Initial angles and drafts created for specific platforms.
- **LaunchPlanned**: Pre-launch, launch, and post-launch checklists generated.
- **Active**: Content calendar in progress.
- **Completed**: Campaign finished.

## Integration with Cofounder
The Socializer can accept an `Idea ID` from the Cofounder agent. This allows the Socializer to read the validated idea, MVP scope, and competitive analysis directly, bootstrapping the distribution campaign with high-context data.

## Commands
- `/socializer list`: List all campaigns.
- `/socializer new-campaign`: Create a new distribution campaign.
- `/socializer from-idea <id>`: Create a campaign directly from a Cofounder Idea ID.
- `/socializer audience <id>`: Map the target audience.
- `/socializer channels <id>`: Define channel strategy.
- `/socializer launch <id>`: Generate a launch plan.
- `/socializer <platform> <id>`: Generate drafts for Reddit, LinkedIn, X, etc.

## Safety Model
The Socializer operates under the strictest safety tier of all Prime Agents due to the reputational risk of automated social interaction. It is fully sandboxed from live social APIs unless the user explicitly wires a custom Transport and approves the payload via `ApprovalGate`.
