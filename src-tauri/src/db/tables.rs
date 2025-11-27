//! Table definitions for redb
//!
//! All tables are defined here as constants for consistent access across the application.

use redb::TableDefinition;

/// Workspaces table: workspace_id -> workspace JSON
pub const WORKSPACES: TableDefinition<&str, &str> = TableDefinition::new("workspaces");

/// Workspace sync groups table: sync_group_id -> sync_group JSON
pub const WORKSPACE_SYNC_GROUPS: TableDefinition<&str, &str> =
    TableDefinition::new("workspace_sync_groups");

/// Collections table: collection_id -> collection JSON
pub const COLLECTIONS: TableDefinition<&str, &str> = TableDefinition::new("collections");

/// Requests table: request_id -> request JSON
pub const REQUESTS: TableDefinition<&str, &str> = TableDefinition::new("requests");

/// Variables table: variable_id -> variable JSON
pub const VARIABLES: TableDefinition<&str, &str> = TableDefinition::new("variables");

/// App settings table: "settings" -> settings JSON (single row)
pub const APP_SETTINGS: TableDefinition<&str, &str> = TableDefinition::new("app_settings");

// Index tables for efficient lookups

/// Collections by workspace index: workspace_id -> collection_ids JSON array
pub const COLLECTIONS_BY_WORKSPACE: TableDefinition<&str, &str> =
    TableDefinition::new("idx_collections_by_workspace");

/// Requests by collection index: collection_id -> request_ids JSON array
pub const REQUESTS_BY_COLLECTION: TableDefinition<&str, &str> =
    TableDefinition::new("idx_requests_by_collection");

/// Requests by workspace (standalone) index: workspace_id -> request_ids JSON array
pub const REQUESTS_BY_WORKSPACE: TableDefinition<&str, &str> =
    TableDefinition::new("idx_requests_by_workspace");

/// Variables by scope index: scope_key -> variable_ids JSON array
/// scope_key format: "global", "workspace:{id}", "collection:{id}", "request:{id}"
pub const VARIABLES_BY_SCOPE: TableDefinition<&str, &str> =
    TableDefinition::new("idx_variables_by_scope");

/// Workspaces by sync group index: sync_group_id -> workspace_ids JSON array
pub const WORKSPACES_BY_SYNC_GROUP: TableDefinition<&str, &str> =
    TableDefinition::new("idx_workspaces_by_sync_group");
