use crate::db::{Database, DbResult, APP_SETTINGS};

use super::types::{AppSettings, UpdateSettingsInput};

const SETTINGS_KEY: &str = "settings";

pub struct SettingsService {
    db: Database,
}

impl SettingsService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn get(&self) -> DbResult<AppSettings> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(APP_SETTINGS)?;

        match table.get(SETTINGS_KEY)? {
            Some(value) => {
                let settings: AppSettings = serde_json::from_str(value.value())?;
                Ok(settings)
            }
            None => Ok(AppSettings::default()),
        }
    }

    pub fn update(&self, input: UpdateSettingsInput) -> DbResult<AppSettings> {
        let mut settings = self.get()?;

        if let Some(theme) = input.theme {
            settings.theme = theme;
        }
        if let Some(custom_themes) = input.custom_themes {
            settings.custom_themes = custom_themes;
        }
        if let Some(default_timeout) = input.default_timeout {
            settings.default_timeout = default_timeout;
        }
        if let Some(follow_redirects) = input.follow_redirects {
            settings.follow_redirects = follow_redirects;
        }
        if let Some(validate_ssl) = input.validate_ssl {
            settings.validate_ssl = validate_ssl;
        }
        if let Some(max_history_items) = input.max_history_items {
            settings.max_history_items = max_history_items;
        }
        if let Some(auto_save_requests) = input.auto_save_requests {
            settings.auto_save_requests = auto_save_requests;
        }

        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(APP_SETTINGS)?;
            let json = serde_json::to_string(&settings)?;
            table.insert(SETTINGS_KEY, json.as_str())?;
        }
        write_txn.commit()?;

        Ok(settings)
    }

    pub fn reset(&self) -> DbResult<AppSettings> {
        let settings = AppSettings::default();

        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(APP_SETTINGS)?;
            let json = serde_json::to_string(&settings)?;
            table.insert(SETTINGS_KEY, json.as_str())?;
        }
        write_txn.commit()?;

        Ok(settings)
    }
}
