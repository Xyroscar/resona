use chrono::Utc;
use redb::ReadableTable;

use crate::db::{Database, DbError, DbResult, VARIABLES, VARIABLES_BY_SCOPE};

use super::types::{CreateVariableInput, ResolvedVariable, UpdateVariableInput, Variable, VariableScope};

pub struct VariableService {
    db: Database,
}

impl VariableService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn get(&self, id: &str) -> DbResult<Variable> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(VARIABLES)?;

        let value = table
            .get(id)?
            .ok_or_else(|| DbError::NotFound(format!("Variable not found: {}", id)))?;

        let variable: Variable = serde_json::from_str(value.value())
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        Ok(variable)
    }

    fn get_by_scope_key(&self, scope_key: &str) -> DbResult<Vec<Variable>> {
        let read_txn = self.db.begin_read()?;
        let idx_table = read_txn.open_table(VARIABLES_BY_SCOPE)?;

        let variable_ids: Vec<String> = match idx_table.get(scope_key)? {
            Some(value) => serde_json::from_str(value.value())
                .map_err(|e| DbError::Serialization(e.to_string()))?,
            None => return Ok(Vec::new()),
        };

        drop(idx_table);
        drop(read_txn);

        let mut variables = Vec::new();
        for id in variable_ids {
            if let Ok(variable) = self.get(&id) {
                variables.push(variable);
            }
        }

        variables.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(variables)
    }

    pub fn get_global(&self) -> DbResult<Vec<Variable>> {
        self.get_by_scope_key("global")
    }

    pub fn get_by_workspace(&self, workspace_id: &str) -> DbResult<Vec<Variable>> {
        self.get_by_scope_key(&format!("workspace:{}", workspace_id))
    }

    pub fn get_by_collection(&self, collection_id: &str) -> DbResult<Vec<Variable>> {
        self.get_by_scope_key(&format!("collection:{}", collection_id))
    }

    pub fn get_by_request(&self, request_id: &str) -> DbResult<Vec<Variable>> {
        self.get_by_scope_key(&format!("request:{}", request_id))
    }

    pub fn get_resolved(
        &self,
        workspace_id: Option<&str>,
        collection_id: Option<&str>,
        request_id: Option<&str>,
    ) -> DbResult<Vec<ResolvedVariable>> {
        let mut resolved_map = std::collections::HashMap::new();

        // Global variables (lowest priority)
        for var in self.get_global()? {
            resolved_map.insert(var.name.clone(), ResolvedVariable {
                name: var.name,
                value: var.value,
                scope: var.scope,
                is_secret: var.is_secret,
            });
        }

        // Workspace variables
        if let Some(ws_id) = workspace_id {
            for var in self.get_by_workspace(ws_id)? {
                resolved_map.insert(var.name.clone(), ResolvedVariable {
                    name: var.name,
                    value: var.value,
                    scope: var.scope,
                    is_secret: var.is_secret,
                });
            }
        }

        // Collection variables
        if let Some(coll_id) = collection_id {
            for var in self.get_by_collection(coll_id)? {
                resolved_map.insert(var.name.clone(), ResolvedVariable {
                    name: var.name,
                    value: var.value,
                    scope: var.scope,
                    is_secret: var.is_secret,
                });
            }
        }

        // Request variables (highest priority)
        if let Some(req_id) = request_id {
            for var in self.get_by_request(req_id)? {
                resolved_map.insert(var.name.clone(), ResolvedVariable {
                    name: var.name,
                    value: var.value,
                    scope: var.scope,
                    is_secret: var.is_secret,
                });
            }
        }

        let mut resolved: Vec<_> = resolved_map.into_values().collect();
        resolved.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(resolved)
    }

    pub fn create(&self, input: CreateVariableInput) -> DbResult<Variable> {
        let mut variable = Variable::new(
            input.name,
            input.value,
            input.scope,
            input.scope_id,
        );
        variable.is_secret = input.is_secret;
        variable.description = input.description;

        let scope_key = variable.scope_key();
        let json = serde_json::to_string(&variable)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let write_txn = self.db.begin_write()?;

        {
            let mut table = write_txn.open_table(VARIABLES)?;
            table.insert(variable.id.as_str(), json.as_str())?;
        }

        // Update scope index
        {
            let mut idx_table = write_txn.open_table(VARIABLES_BY_SCOPE)?;
            let ids_json = match idx_table.get(scope_key.as_str())? {
                Some(value) => value.value().to_string(),
                None => "[]".to_string(),
            };

            let mut ids: Vec<String> = serde_json::from_str(&ids_json)
                .map_err(|e| DbError::Serialization(e.to_string()))?;

            ids.push(variable.id.clone());

            let new_json = serde_json::to_string(&ids)
                .map_err(|e| DbError::Serialization(e.to_string()))?;

            idx_table.insert(scope_key.as_str(), new_json.as_str())?;
        }

        write_txn.commit()?;

        Ok(variable)
    }

    pub fn update(&self, input: UpdateVariableInput) -> DbResult<Variable> {
        let mut variable = self.get(&input.id)?;

        if let Some(name) = input.name {
            variable.name = name;
        }
        if let Some(value) = input.value {
            variable.value = value;
        }
        if let Some(is_secret) = input.is_secret {
            variable.is_secret = is_secret;
        }
        if let Some(description) = input.description {
            variable.description = Some(description);
        }
        variable.updated_at = Utc::now();

        let json = serde_json::to_string(&variable)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(VARIABLES)?;
            table.insert(variable.id.as_str(), json.as_str())?;
        }
        write_txn.commit()?;

        Ok(variable)
    }

    pub fn delete(&self, id: &str) -> DbResult<()> {
        let variable = self.get(id)?;
        let scope_key = variable.scope_key();

        let write_txn = self.db.begin_write()?;

        // Remove from variables table
        {
            let mut table = write_txn.open_table(VARIABLES)?;
            table.remove(id)?;
        }

        // Update scope index
        {
            let mut idx_table = write_txn.open_table(VARIABLES_BY_SCOPE)?;
            let ids_json = idx_table
                .get(scope_key.as_str())?
                .map(|v| v.value().to_string())
                .unwrap_or_else(|| "[]".to_string());
            let mut ids: Vec<String> = serde_json::from_str(&ids_json)
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            ids.retain(|i| i != id);
            let new_json = serde_json::to_string(&ids)
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            idx_table.insert(scope_key.as_str(), new_json.as_str())?;
        }

        write_txn.commit()?;

        Ok(())
    }
}
