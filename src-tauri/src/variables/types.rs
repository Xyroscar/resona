use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VariableScope {
    Global,
    Workspace,
    Collection,
    Request,
}

impl Default for VariableScope {
    fn default() -> Self {
        Self::Global
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub id: String,
    pub name: String,
    pub value: String,
    pub scope: VariableScope,
    pub scope_id: Option<String>,
    pub is_secret: bool,
    pub description: Option<String>,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl Variable {
    pub fn new(name: String, value: String, scope: VariableScope, scope_id: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            value,
            scope,
            scope_id,
            is_secret: false,
            description: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn scope_key(&self) -> String {
        match self.scope {
            VariableScope::Global => "global".to_string(),
            VariableScope::Workspace => format!("workspace:{}", self.scope_id.as_deref().unwrap_or("")),
            VariableScope::Collection => format!("collection:{}", self.scope_id.as_deref().unwrap_or("")),
            VariableScope::Request => format!("request:{}", self.scope_id.as_deref().unwrap_or("")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVariableInput {
    pub name: String,
    pub value: String,
    pub scope: VariableScope,
    pub scope_id: Option<String>,
    #[serde(default)]
    pub is_secret: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVariableInput {
    pub id: String,
    pub name: Option<String>,
    pub value: Option<String>,
    pub is_secret: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedVariable {
    pub name: String,
    pub value: String,
    pub scope: VariableScope,
    pub is_secret: bool,
}
