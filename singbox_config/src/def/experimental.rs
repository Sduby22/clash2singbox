use serde::{Deserialize, Serialize};

use super::types::OneOrMany;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experimental {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_file: Option<CacheFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_api: Option<ClashApi>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2ray_api: Option<V2RayApi>,
}

// Define the structures for each experimental field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_id: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_fakeip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_rdrc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdrc_timeout: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClashApi {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_controller: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ui: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ui_download_url: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ui_download_detour: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_allow_origin: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_allow_private_network: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_fakeip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_file: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_id: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V2RayApi {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbounds: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbounds: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<OneOrMany<String>>,
}
