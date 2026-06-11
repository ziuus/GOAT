# GOAT Desktop Automation Boundary (Phase 6.9)

## 1. Safety Boundary
Full mouse, keyboard, and screen capture desktop automation is restricted. In Phase 6.9, this system establishes a safe boundary:
- **No Uncontrolled Clicks**: Mouse movements or click events on external coordinates are blocked.
- **No Invisible Shell commands**: All command plans must display to the user.
- **Active Window Auditing**: System checks active windows and warns on sensitive background tasks.
- **Future Integration**: Integrations with Tauri desktop client will strictly adhere to the ApprovalGate.
