use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    System,
    Light,
    Dark,
    Latte,
    Frappe,
    Macchiato,
    Mocha,
    Custom,
}

impl Default for Theme {
    fn default() -> Self {
        Self::System
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomTheme {
    pub name: String,
    pub colors: ThemeColors,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColors {
    pub background: String,
    pub foreground: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub muted: String,
    pub border: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub theme: Theme,
    #[serde(default)]
    pub custom_themes: Vec<CustomTheme>,
    #[serde(default = "default_timeout")]
    pub default_timeout: u32,
    #[serde(default = "default_true")]
    pub follow_redirects: bool,
    #[serde(default = "default_true")]
    pub validate_ssl: bool,
    #[serde(default = "default_max_history")]
    pub max_history_items: u32,
    #[serde(default = "default_true")]
    pub auto_save_requests: bool,
}

fn default_timeout() -> u32 {
    30000
}

fn default_true() -> bool {
    true
}

fn default_max_history() -> u32 {
    100
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            custom_themes: Vec::new(),
            default_timeout: 30000,
            follow_redirects: true,
            validate_ssl: true,
            max_history_items: 100,
            auto_save_requests: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsInput {
    pub theme: Option<Theme>,
    pub custom_themes: Option<Vec<CustomTheme>>,
    pub default_timeout: Option<u32>,
    pub follow_redirects: Option<bool>,
    pub validate_ssl: Option<bool>,
    pub max_history_items: Option<u32>,
    pub auto_save_requests: Option<bool>,
}
