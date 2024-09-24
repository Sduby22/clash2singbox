use serde::{Deserialize, Serialize};

use super::{rule_set::RuleSet, rules, types::OneOrMany};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Route {
    pub rules: Vec<Rule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set: Option<OneOrMany<RuleSet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoip: Option<GeoIP>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geosite: Option<Geosite>,
    #[serde(rename = "final")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_detect_interface: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_android_vpn: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_interface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mark: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeoIP {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_detour: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Geosite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_detour: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rule {
    pub outbound: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<rules::Rule>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_ser() {
        let str = r#"
        {
          "auto_detect_interface": true,
          "rules": [
            {
              "outbound": "dns-out",
              "protocol": "dns"
            },
            {
              "geoip": [
                "private",
                "cn"
              ],
              "geosite": "cn",
              "domain_suffix": ["home.arpa", "local"],
              "outbound": "direct"
            },
            {
              "geosite": "category-ads-all",
              "outbound": "block"
            }
          ]
        }
        "#;
        let route: Route = serde_json::from_str(&str).unwrap();
        let reser = serde_json::to_string(&route).unwrap();
        println!("reserialized route = {reser}");
    }
}
