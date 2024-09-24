pub mod dns;
pub mod experimental;
pub mod inbounds;
pub mod outbounds;
pub mod routes;
pub mod rule_set;
pub mod rules;
pub mod types;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub inbounds: Vec<inbounds::Inbound>,
    pub outbounds: Vec<outbounds::Outbound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<dns::Dns>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<routes::Route>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Log>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp: Option<Ntp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<experimental::Experimental>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Log {
    pub disabled: bool,
    pub level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    pub timestamp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ntp {
    pub enabled: bool,
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<types::Dial>,
}
