use serde::{Serialize, Deserialize};

use super::types::TunCommon;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
#[serde(tag="type")]
pub enum Listener {
    #[serde(skip)]
    None,
    Socks(Socks),
    Http(Http),
    Mixed(Mixed),
    Redir(Redir),
    Tproxy(Tproxy),
    Tunnel(Tunnel),
    Tun(Tun),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerBase {
    pub name: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Socks {
    #[serde(flatten)]
    pub base: ListenerBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Http {
    #[serde(flatten)]
    pub base: ListenerBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mixed {
    #[serde(flatten)]
    pub base: ListenerBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redir {
    #[serde(flatten)]
    pub base: ListenerBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tproxy {
    #[serde(flatten)]
    pub base: ListenerBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tunnel {
    #[serde(flatten)]
    pub base: ListenerBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tun {
    #[serde(flatten)]
    pub base: ListenerBase,
    #[serde(flatten)]
    pub tun: TunCommon,
}
