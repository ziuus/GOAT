# GOAT UI/UX Principles

## 1. Calm Productivity
GOAT is a powerful tool, but it should never induce anxiety. We use dark, soft colors, restrained typography, and plenty of negative space.

## 2. Honest and Inspectable
Never fake AI actions. Never fake live data. If a feature is partial or in alpha, clearly state it. The `ApprovalGate` must always be visible when actions are pending.

## 3. Human-Centric Copy
Avoid overly technical jargon unless the user is explicitly in a deep configuration menu.
- **Instead of**: "Launch CofounderAgent struct to memory_galaxy"
- **Use**: "Validate an idea"

## 4. Progressive Disclosure
Don't overwhelm the user. High-level dashboard views should show status and quick actions. Deep metrics and logs should be one click away.

## 5. Keyboard Friendly
Power users prefer the keyboard. Ensure `Ctrl+K` command palettes and clear focus states are present.

## 6. Meaningful Motion
Animations should communicate state changes (like an agent starting work) but shouldn't just be decoration. Avoid layout jank on list renders.
