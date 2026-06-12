import sys

with open("src/api_server.rs", "r") as f:
    content = f.read()

# Add routes
content = content.replace(
    '.route("/v1/projects/:id/context", get(projects_context_handler))',
    '.route("/v1/projects/:id/context", get(projects_context_handler))\n        .route("/v1/patches", get(patches_list_handler))\n        .route("/v1/checkpoints", get(checkpoints_list_handler))'
)

# Add handlers at the end of the file
handlers = """
async fn patches_list_handler() -> impl axum::response::IntoResponse {
    let patch_mgr = crate::patch_manager::PatchManager::new();
    let patches = patch_mgr.get_patches();
    axum::Json(patches)
}

async fn checkpoints_list_handler() -> impl axum::response::IntoResponse {
    let cp_mgr = crate::checkpoint::CheckpointManager::new(&crate::paths::GoatPaths::default().data_dir);
    if let Ok(checkpoints) = cp_mgr.list_checkpoints() {
        axum::Json(checkpoints)
    } else {
        axum::Json(Vec::<crate::checkpoint::Checkpoint>::new())
    }
}
"""

content += handlers

with open("src/api_server.rs", "w") as f:
    f.write(content)

