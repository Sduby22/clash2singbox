use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::gen_enum;

use super::types::{Dial, OutboundTls, OutboundMultiplex, V2rayTransport, OneOrMany};


gen_enum!(Outbound, Direct, Block, Dns, Selector, UrlTest, Socks, Http, Shadowsocks, Vmess, Trojan, Hysteria, Vless, Hysteria2, Ssh);


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Direct {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol: Option<u8>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<Dial>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dns {
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Selector {
    pub tag: String,
    pub outbounds: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupt_exist_connections: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UrlTest {
    pub tag: String,
    pub outbounds: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupt_exist_connections: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Socks {
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_over_tcp: Option<bool>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<Dial>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Http {
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<OutboundTls>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<Dial>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShadowsocksPlugin {
    #[serde(rename = "obfs-local")]
    ObfsLocal,
    #[serde(rename = "v2ray-plugin")]
    V2rayPlugin
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Shadowsocks {
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub method: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<ShadowsocksPlugin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_opts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_over_tcp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex: Option<OutboundMultiplex>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<Dial>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vmess {
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alter_id: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_padding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticated_length: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<OutboundTls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<V2rayTransport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex: Option<OutboundMultiplex>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<Dial>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Trojan {
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<OutboundTls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex: Option<OutboundMultiplex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<V2rayTransport>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<Dial>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Hysteria {
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub up: String,
    pub up_mbps: u32,
    pub down: String,
    pub down_mbps: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_str: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window_conn: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_mtu_discovery: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<OutboundTls>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<Dial>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vless {
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<OutboundTls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex: Option<OutboundMultiplex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<V2rayTransport>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<Dial>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Hysteria2 {
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_mbps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_mbps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfs: Option<HashMap<String,String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub tls: OutboundTls,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brutal_debug: Option<bool>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<Dial>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ssh {
    pub tag: String,
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_passphrase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_key_algorithms: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_version: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial: Option<Dial>,
}
