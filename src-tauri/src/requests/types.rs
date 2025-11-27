use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::Get
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum BodyType {
    None,
    Json,
    Xml,
    Text,
    Html,
    FormData,
    #[serde(rename = "x-www-form-urlencoded")]
    XWwwFormUrlencoded,
}

impl Default for BodyType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestHeader {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestParam {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormDataItem {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub item_type: String, // "text" or "file"
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: String,
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
    #[serde(default)]
    pub headers: Vec<RequestHeader>,
    #[serde(default)]
    pub params: Vec<RequestParam>,
    #[serde(default)]
    pub body_type: BodyType,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub form_data: Vec<FormDataItem>,
    pub collection_id: Option<String>,
    pub workspace_id: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl Request {
    pub fn new(name: String, method: HttpMethod, workspace_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            method,
            url: String::new(),
            headers: Vec::new(),
            params: Vec::new(),
            body_type: BodyType::None,
            body: String::new(),
            form_data: Vec::new(),
            collection_id: None,
            workspace_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestInput {
    pub name: String,
    pub method: HttpMethod,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub headers: Vec<RequestHeader>,
    #[serde(default)]
    pub params: Vec<RequestParam>,
    #[serde(default)]
    pub body_type: BodyType,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub form_data: Vec<FormDataItem>,
    pub collection_id: Option<String>,
    pub workspace_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRequestInput {
    pub id: String,
    pub name: Option<String>,
    pub method: Option<HttpMethod>,
    pub url: Option<String>,
    pub headers: Option<Vec<RequestHeader>>,
    pub params: Option<Vec<RequestParam>>,
    pub body_type: Option<BodyType>,
    pub body: Option<String>,
    pub form_data: Option<Vec<FormDataItem>>,
    pub collection_id: Option<Option<String>>,
}
