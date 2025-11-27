use chrono::Utc;
use redb::ReadableTable;

use crate::db::{
    Database, DbError, DbResult, REQUESTS, REQUESTS_BY_COLLECTION, REQUESTS_BY_WORKSPACE,
};

use super::types::{CreateRequestInput, Request, UpdateRequestInput};

pub struct RequestService {
    db: Database,
}

impl RequestService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn get(&self, id: &str) -> DbResult<Request> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(REQUESTS)?;

        let value = table
            .get(id)?
            .ok_or_else(|| DbError::NotFound(format!("Request not found: {}", id)))?;

        let request: Request = serde_json::from_str(value.value())
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        Ok(request)
    }

    pub fn get_by_collection(&self, collection_id: &str) -> DbResult<Vec<Request>> {
        let read_txn = self.db.begin_read()?;
        let idx_table = read_txn.open_table(REQUESTS_BY_COLLECTION)?;

        let request_ids: Vec<String> = match idx_table.get(collection_id)? {
            Some(value) => serde_json::from_str(value.value())
                .map_err(|e| DbError::Serialization(e.to_string()))?,
            None => return Ok(Vec::new()),
        };

        drop(idx_table);
        drop(read_txn);

        let mut requests = Vec::new();
        for id in request_ids {
            if let Ok(request) = self.get(&id) {
                requests.push(request);
            }
        }

        requests.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(requests)
    }

    pub fn get_standalone_by_workspace(&self, workspace_id: &str) -> DbResult<Vec<Request>> {
        let read_txn = self.db.begin_read()?;
        let idx_table = read_txn.open_table(REQUESTS_BY_WORKSPACE)?;

        let request_ids: Vec<String> = match idx_table.get(workspace_id)? {
            Some(value) => serde_json::from_str(value.value())
                .map_err(|e| DbError::Serialization(e.to_string()))?,
            None => return Ok(Vec::new()),
        };

        drop(idx_table);
        drop(read_txn);

        let mut requests = Vec::new();
        for id in request_ids {
            if let Ok(request) = self.get(&id) {
                if request.collection_id.is_none() {
                    requests.push(request);
                }
            }
        }

        requests.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(requests)
    }

    pub fn get_all_by_workspace(&self, workspace_id: &str) -> DbResult<Vec<Request>> {
        let read_txn = self.db.begin_read()?;
        let idx_table = read_txn.open_table(REQUESTS_BY_WORKSPACE)?;

        let request_ids: Vec<String> = match idx_table.get(workspace_id)? {
            Some(value) => serde_json::from_str(value.value())
                .map_err(|e| DbError::Serialization(e.to_string()))?,
            None => return Ok(Vec::new()),
        };

        drop(idx_table);
        drop(read_txn);

        let mut requests = Vec::new();
        for id in request_ids {
            if let Ok(request) = self.get(&id) {
                requests.push(request);
            }
        }

        requests.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(requests)
    }

    pub fn create(&self, input: CreateRequestInput) -> DbResult<Request> {
        let mut request = Request::new(input.name, input.method, input.workspace_id.clone());
        request.url = input.url;
        request.headers = input.headers;
        request.params = input.params;
        request.body_type = input.body_type;
        request.body = input.body;
        request.form_data = input.form_data;
        request.collection_id = input.collection_id.clone();

        let json = serde_json::to_string(&request)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let write_txn = self.db.begin_write()?;

        {
            let mut table = write_txn.open_table(REQUESTS)?;
            table.insert(request.id.as_str(), json.as_str())?;
        }

        // Update workspace index
        {
            let mut idx_table = write_txn.open_table(REQUESTS_BY_WORKSPACE)?;
            let ids_json = match idx_table.get(input.workspace_id.as_str())? {
                Some(value) => value.value().to_string(),
                None => "[]".to_string(),
            };

            let mut ids: Vec<String> = serde_json::from_str(&ids_json)
                .map_err(|e| DbError::Serialization(e.to_string()))?;

            ids.push(request.id.clone());

            let new_json = serde_json::to_string(&ids)
                .map_err(|e| DbError::Serialization(e.to_string()))?;

            idx_table.insert(input.workspace_id.as_str(), new_json.as_str())?;
        }

        // Update collection index if applicable
        if let Some(ref collection_id) = input.collection_id {
            let mut idx_table = write_txn.open_table(REQUESTS_BY_COLLECTION)?;
            let ids_json = match idx_table.get(collection_id.as_str())? {
                Some(value) => value.value().to_string(),
                None => "[]".to_string(),
            };

            let mut ids: Vec<String> = serde_json::from_str(&ids_json)
                .map_err(|e| DbError::Serialization(e.to_string()))?;

            ids.push(request.id.clone());

            let new_json = serde_json::to_string(&ids)
                .map_err(|e| DbError::Serialization(e.to_string()))?;

            idx_table.insert(collection_id.as_str(), new_json.as_str())?;
        }

        write_txn.commit()?;

        Ok(request)
    }

    pub fn update(&self, input: UpdateRequestInput) -> DbResult<Request> {
        let mut request = self.get(&input.id)?;
        let old_collection_id = request.collection_id.clone();

        if let Some(name) = input.name {
            request.name = name;
        }
        if let Some(method) = input.method {
            request.method = method;
        }
        if let Some(url) = input.url {
            request.url = url;
        }
        if let Some(headers) = input.headers {
            request.headers = headers;
        }
        if let Some(params) = input.params {
            request.params = params;
        }
        if let Some(body_type) = input.body_type {
            request.body_type = body_type;
        }
        if let Some(body) = input.body {
            request.body = body;
        }
        if let Some(form_data) = input.form_data {
            request.form_data = form_data;
        }
        if let Some(collection_id) = input.collection_id {
            request.collection_id = collection_id;
        }
        request.updated_at = Utc::now();

        let json = serde_json::to_string(&request)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let write_txn = self.db.begin_write()?;

        {
            let mut table = write_txn.open_table(REQUESTS)?;
            table.insert(request.id.as_str(), json.as_str())?;
        }

        // Update collection index if collection changed
        if old_collection_id != request.collection_id {
            // Remove from old collection index
            if let Some(ref old_id) = old_collection_id {
                let mut idx_table = write_txn.open_table(REQUESTS_BY_COLLECTION)?;
                let ids_json = idx_table
                    .get(old_id.as_str())?
                    .map(|v| v.value().to_string())
                    .unwrap_or_else(|| "[]".to_string());
                let mut ids: Vec<String> = serde_json::from_str(&ids_json)
                    .map_err(|e| DbError::Serialization(e.to_string()))?;
                ids.retain(|i| i != &request.id);
                let new_json = serde_json::to_string(&ids)
                    .map_err(|e| DbError::Serialization(e.to_string()))?;
                idx_table.insert(old_id.as_str(), new_json.as_str())?;
            }

            // Add to new collection index
            if let Some(ref new_id) = request.collection_id {
                let mut idx_table = write_txn.open_table(REQUESTS_BY_COLLECTION)?;
                let ids_json = idx_table
                    .get(new_id.as_str())?
                    .map(|v| v.value().to_string())
                    .unwrap_or_else(|| "[]".to_string());
                let mut ids: Vec<String> = serde_json::from_str(&ids_json)
                    .map_err(|e| DbError::Serialization(e.to_string()))?;
                if !ids.contains(&request.id) {
                    ids.push(request.id.clone());
                }
                let new_json = serde_json::to_string(&ids)
                    .map_err(|e| DbError::Serialization(e.to_string()))?;
                idx_table.insert(new_id.as_str(), new_json.as_str())?;
            }
        }

        write_txn.commit()?;

        Ok(request)
    }

    pub fn delete(&self, id: &str) -> DbResult<()> {
        let request = self.get(id)?;

        let write_txn = self.db.begin_write()?;

        // Remove from requests table
        {
            let mut table = write_txn.open_table(REQUESTS)?;
            table.remove(id)?;
        }

        // Update workspace index
        {
            let mut idx_table = write_txn.open_table(REQUESTS_BY_WORKSPACE)?;
            let ids_json = idx_table
                .get(request.workspace_id.as_str())?
                .map(|v| v.value().to_string())
                .unwrap_or_else(|| "[]".to_string());
            let mut ids: Vec<String> = serde_json::from_str(&ids_json)
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            ids.retain(|i| i != id);
            let new_json = serde_json::to_string(&ids)
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            idx_table.insert(request.workspace_id.as_str(), new_json.as_str())?;
        }

        // Update collection index if applicable
        if let Some(ref collection_id) = request.collection_id {
            let mut idx_table = write_txn.open_table(REQUESTS_BY_COLLECTION)?;
            let ids_json = idx_table
                .get(collection_id.as_str())?
                .map(|v| v.value().to_string())
                .unwrap_or_else(|| "[]".to_string());
            let mut ids: Vec<String> = serde_json::from_str(&ids_json)
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            ids.retain(|i| i != id);
            let new_json = serde_json::to_string(&ids)
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            idx_table.insert(collection_id.as_str(), new_json.as_str())?;
        }

        write_txn.commit()?;

        Ok(())
    }
}
