use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
#[serde(tag="type")]
pub enum ProxyGroup {
    #[serde(skip)]
    None,
    Relay(Relay),
    UrlTest(UrlTest),
    Fallback(Fallback),
    LoadBalance(LoadBalance),
    Select(Select),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relay {
    pub name: String,
    pub proxies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlTest {
    pub name: String,
    pub proxies: Vec<String>,
    pub url: String,
    pub interval: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fallback {
    pub name: String,
    pub proxies: Vec<String>,
    pub url: String,
    pub interval: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_udp: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadBalance {
    pub name: String,
    pub proxies: Vec<String>,
    pub url: String,
    pub interval: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_udp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Select {
    pub name: String,
    pub proxies: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_udp: Option<bool>,
}