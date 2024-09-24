use serde::{Deserialize, Serialize};

use super::types::OneOrMany;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clash_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_user: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_suffix: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_keyword: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_regex: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geosite: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_geoip: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoip: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_cidr: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_cidr: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<OneOrMany<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_range: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<OneOrMany<u16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<OneOrMany<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_ssid: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_bssid: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set: Option<OneOrMany<String>>,
}
