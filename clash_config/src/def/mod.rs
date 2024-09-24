pub mod dns;
pub mod listeners;
pub mod proxies;
pub mod proxy_groups;
pub mod tun;
pub mod types;
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};

use self::listeners::Listener;
use self::{proxies::Proxy, proxy_groups::ProxyGroup};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_lan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_auth_prefixes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive_interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_process_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_delay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_concurrent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mark: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socks_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redir_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tproxy_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<dns::Dns>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<Listener>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxies: Option<Vec<Proxy>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_groups: Option<Vec<ProxyGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub category: String,
    pub content: String,
    pub target: String,
    pub no_resolve: bool,
}

impl Serialize for Rule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut str = format!("{},{},{}", self.category, self.content, self.target);
        if self.category == "MATCH" {
            str = format!("{},{}", self.category, self.target);
        } else if self.no_resolve {
            str.push_str(",no-resolve");
        }
        serializer.serialize_str(&str)
    }
}

impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(RuleVisitor)
    }
}

struct RuleVisitor;

impl<'de> Visitor<'de> for RuleVisitor {
    type Value = Rule;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("the struct Rule")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let strs: Vec<&str> = value.split(",").collect();
        if strs.len() < 3 {
            Ok(Rule {
                category: strs[0].to_owned(),
                content: "".to_owned(),
                target: strs[1].to_owned(),
                no_resolve: false,
            })
        } else {
            Ok(Rule {
                category: strs[0].to_owned(),
                content: strs[1].to_owned(),
                target: strs[2].to_owned(),
                no_resolve: if strs.len() > 3 { true } else { false },
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestRules {
        rules: Vec<Rule>,
    }

    #[test]
    fn proxies_ser() {
        let str = r#"
        rules:
            - DOMAIN-SUFFIX,dmm.com,DIRECT
            - DOMAIN-SUFFIX,dns-dns.com,abc
            - DOMAIN-SUFFIX,dns-stuff.com,add
            - DOMAIN,dns.google,add
            - IP-CIDR,192.168.1.0/24,DIRECT,no-resolve
        "#;
        let rules: TestRules = serde_yaml::from_str(&str).unwrap();
        println!("{:?}", rules);
        let reser = serde_yaml::to_string(&rules);
        println!("{:?}", reser);
    }
}
