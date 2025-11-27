use chrono::Utc;
use redb::ReadableTable;

use crate::db::{
    Database, DbError, DbResult, COLLECTIONS, COLLECTIONS_BY_WORKSPACE,
};

use super::types::{Collection, CreateCollectionInput, UpdateCollectionInput};

pub struct CollectionService {
    db: Database,
}

impl CollectionService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn get_all(&self) -> DbResult<Vec<Collection>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(COLLECTIONS)?;

        let mut collections = Vec::new();
        for entry in table.iter()? {
            let (_, value) = entry?;
            let collection: Collection = serde_json::from_str(value.value())
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            collections.push(collection);
        }

        collections.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(collections)
    }

    pub fn get(&self, id: &str) -> DbResult<Collection> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(COLLECTIONS)?;

        let value = table
            .get(id)?
            .ok_or_else(|| DbError::NotFound(format!("Collection not found: {}", id)))?;

        let collection: Collection = serde_json::from_str(value.value())
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        Ok(collection)
    }

    pub fn get_by_workspace(&self, workspace_id: &str) -> DbResult<Vec<Collection>> {
        let read_txn = self.db.begin_read()?;
        let idx_table = read_txn.open_table(COLLECTIONS_BY_WORKSPACE)?;

        let collection_ids: Vec<String> = match idx_table.get(workspace_id)? {
            Some(value) => serde_json::from_str(value.value())
                .map_err(|e| DbError::Serialization(e.to_string()))?,
            None => return Ok(Vec::new()),
        };

        drop(idx_table);
        drop(read_txn);

        let mut collections = Vec::new();
        for id in collection_ids {
            if let Ok(collection) = self.get(&id) {
                collections.push(collection);
            }
        }

        collections.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(collections)
    }

    pub fn create(&self, input: CreateCollectionInput) -> DbResult<Collection> {
        let collection = Collection::new(input.name, input.description, input.workspace_id.clone());

        let json = serde_json::to_string(&collection)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let write_txn = self.db.begin_write()?;

        {
            let mut table = write_txn.open_table(COLLECTIONS)?;
            table.insert(collection.id.as_str(), json.as_str())?;
        }

        // Update index
        {
            let mut idx_table = write_txn.open_table(COLLECTIONS_BY_WORKSPACE)?;
            let ids_json = match idx_table.get(input.workspace_id.as_str())? {
                Some(value) => value.value().to_string(),
                None => "[]".to_string(),
            };

            let mut ids: Vec<String> = serde_json::from_str(&ids_json)
                .map_err(|e| DbError::Serialization(e.to_string()))?;

            ids.push(collection.id.clone());

            let new_json = serde_json::to_string(&ids)
                .map_err(|e| DbError::Serialization(e.to_string()))?;

            idx_table.insert(input.workspace_id.as_str(), new_json.as_str())?;
        }

        write_txn.commit()?;

        Ok(collection)
    }

    pub fn update(&self, input: UpdateCollectionInput) -> DbResult<Collection> {
        let mut collection = self.get(&input.id)?;

        if let Some(name) = input.name {
            collection.name = name;
        }
        if let Some(description) = input.description {
            collection.description = description;
        }
        collection.updated_at = Utc::now();

        let json = serde_json::to_string(&collection)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(COLLECTIONS)?;
            table.insert(collection.id.as_str(), json.as_str())?;
        }
        write_txn.commit()?;

        Ok(collection)
    }

    pub fn delete(&self, id: &str) -> DbResult<()> {
        let collection = self.get(id)?;

        let write_txn = self.db.begin_write()?;

        // Remove from collections table
        {
            let mut table = write_txn.open_table(COLLECTIONS)?;
            table.remove(id)?;
        }

        // Update index
        {
            let mut idx_table = write_txn.open_table(COLLECTIONS_BY_WORKSPACE)?;
            let ids_json = idx_table
                .get(collection.workspace_id.as_str())?
                .map(|v| v.value().to_string())
                .unwrap_or_else(|| "[]".to_string());
            let mut ids: Vec<String> = serde_json::from_str(&ids_json)
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            ids.retain(|i| i != id);
            let new_json = serde_json::to_string(&ids)
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            idx_table.insert(collection.workspace_id.as_str(), new_json.as_str())?;
        }

        write_txn.commit()?;

        Ok(())
    }
}
