use serde::{Deserialize, Serialize};

use crate::gen_enum;

use super::types::{Authentication, InboundTls, Listen, OneOrMany};

gen_enum!(Inbound, Direct, Socks, Http, Mixed, Redirect, Tproxy, Tun);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Direct {
    pub tag: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<Listen>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Socks {
    pub tag: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<Listen>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<Authentication>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Http {
    pub tag: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<Listen>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<Authentication>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<InboundTls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_system_proxy: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Mixed {
    pub tag: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<Listen>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_system_proxy: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Redirect {
    pub tag: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<Listen>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tproxy {
    pub tag: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<Listen>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tun {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_address: Option<OneOrMany<String>>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_address: Option<OneOrMany<String>>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gso: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_route: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iproute2_table_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iproute2_rule_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_redirect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_redirect_input_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_redirect_output_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_route: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_address: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_route_address: Option<OneOrMany<String>>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_route_address: Option<OneOrMany<String>>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_exclude_address: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_route_exclude_address: Option<OneOrMany<String>>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_route_exclude_address: Option<OneOrMany<String>>, // deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_address_set: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_exclude_address_set: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_independent_nat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_interface: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_interface: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_uid: Option<OneOrMany<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_uid_range: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uid: Option<OneOrMany<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uid_range: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_android_user: Option<OneOrMany<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_package: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_package: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<Listen>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Platform {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<HttpProxy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpProxy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_domain: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_domain: Option<OneOrMany<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inbounds_ser() {
        let str = r#"
        [
            {
            "type": "mixed",
            "tag": "mixed-in",
            "listen": "::",
            "listen_port": 7890,
            "sniff": true,
            "set_system_proxy": false
            },
            {
            "auto_route": true,
            "inet4_address": "172.19.0.1/30",
            "sniff": true,
            "strict_route": false,
            "tag": "tun-in",
            "type": "tun"
            }
        ]
        "#;
        let inbounds: Vec<Inbound> = serde_json::from_str(&str).unwrap();
        //println!("derialized inbounds = {:?}", inbounds);
        let reser = serde_json::to_string(&inbounds).unwrap();
        println!("reserialized inbounds = {reser}");
    }

    #[test]
    fn inbounds_impl() {
        let n = Inbound::None;
        let str = n.get_tag();
        println!("{:#?}", str);
    }
}
