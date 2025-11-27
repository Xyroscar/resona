use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_group_id: Option<String>,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl Workspace {
    pub fn new(name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            tags: Vec::new(),
            sync_group_id: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceInput {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkspaceInput {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub sync_group_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSyncGroup {
    pub id: String,
    pub name: String,
    pub workspace_ids: Vec<String>,
    pub synced_variable_names: Vec<String>,
    pub sync_secrets: bool,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl WorkspaceSyncGroup {
    pub fn new(name: String, workspace_ids: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            workspace_ids,
            synced_variable_names: Vec::new(),
            sync_secrets: false,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSyncGroupInput {
    pub name: String,
    pub workspace_ids: Vec<String>,
    #[serde(default)]
    pub synced_variable_names: Vec<String>,
    #[serde(default)]
    pub sync_secrets: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSyncGroupInput {
    pub id: String,
    pub name: Option<String>,
    pub synced_variable_names: Option<Vec<String>>,
    pub sync_secrets: Option<bool>,
}
