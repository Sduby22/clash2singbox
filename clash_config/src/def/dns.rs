use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all="kebab-case")]
pub struct Dns {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_h3: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fake_ip_range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fake_ip_filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_hosts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_nameserver: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameserver_policy: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameserver: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_server_nameserver: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_filter: Option<FallbackFilter>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct FallbackFilter {
    pub geoip: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoip_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geosite: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipcidr: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,
}