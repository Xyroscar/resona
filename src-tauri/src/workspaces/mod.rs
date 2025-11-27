//! Workspaces module
//!
//! Handles workspace management including CRUD operations and sync groups.

mod commands;
mod types;
mod workspace;

// Re-export commands for use in lib.rs
pub use commands::*;

// Re-export types for external use (frontend bindings)
#[allow(unused_imports)]
pub use types::{
    CreateSyncGroupInput, CreateWorkspaceInput, UpdateSyncGroupInput, UpdateWorkspaceInput,
    Workspace, WorkspaceSyncGroup,
};

// WorkspaceService is used internally by commands
#[allow(unused_imports)]
pub(crate) use workspace::WorkspaceService;
