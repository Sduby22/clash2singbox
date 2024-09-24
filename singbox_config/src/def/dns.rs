use serde::{Deserialize, Serialize};

use super::{rules, types::OneOrMany};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dns {
    pub rules: Vec<Rule>,
    pub servers: Vec<Server>,
    #[serde(rename = "final")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_expire: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub independent_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_mapping: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rule {
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrite_ttl: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<OneOrMany<String>>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<rules::Rule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Server {
    pub tag: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_resolver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_subnet: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dns_ser() {
        let str = r#"
        {
          "rules": [
            {
              "disable_cache": true,
              "geosite": "category-ads-all",
              "server": "block"
            },
            {
              "geosite": ["cn", "private"],
              "inbound": "tun-in",
              "server": "local"
            }
          ],
          "servers": [
            {
              "address": "192.168.1.1",
              "detour": "direct",
              "tag": "local"
            },
            {
              "address": "rcode://success",
              "tag": "block"
            }
          ],
          "strategy": "prefer_ipv4"
        }
        "#;
        let dns: Dns = serde_json::from_str(&str).unwrap();
        let reser = serde_json::to_string(&dns).unwrap();
        println!("reserialized dns = {reser}");
    }
}
