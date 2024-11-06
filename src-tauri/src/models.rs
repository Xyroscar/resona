use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variable {
    pub id: String,
    pub name: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Header {
    pub id: String,
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pub url: String,
    pub method: String,
    pub headers: Vec<Header>,
    pub body: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ResponseBody {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub time: u128,
} 