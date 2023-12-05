use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct TunCommon {
    pub stack: String,
    pub dns_hijack: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_detect_interface: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_route: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<u32>,
    pub inet4_address: Vec<String>,
    pub inet6_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_route: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_route_address: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_route_address: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_independent_nat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_uid: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_uid_range: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uid: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uid_range: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_android_user: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_package: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_package: Option<Vec<String>>,
}