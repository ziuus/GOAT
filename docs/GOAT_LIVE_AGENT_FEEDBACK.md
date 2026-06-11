# GOAT Live Agent Feedback

## Strategy
Long-running agent workflows provide live feedback through the daemon/dashboard layer using the internal event bus.

## Events
- `collaboration_started`
- `collaboration_step_started`
- `collaboration_step_completed`
- `collaboration_waiting_for_approval`
- `collaboration_handoff_created`
- `collaboration_paused`
- `collaboration_resumed`
- `collaboration_completed`
- `collaboration_cancelled`

Dashboard fetches these events or live-polls to update the UI without reloading.
