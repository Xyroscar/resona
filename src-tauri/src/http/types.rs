use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub headers: Vec<HttpRequestHeader>,
    #[serde(default)]
    pub params: Vec<HttpRequestParam>,
    pub body_type: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub form_data: Vec<HttpFormDataItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequestHeader {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequestParam {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpFormDataItem {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<HttpResponseHeader>,
    pub body: String,
    pub time_ms: u64,
    pub size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponseHeader {
    pub key: String,
    pub value: String,
}
