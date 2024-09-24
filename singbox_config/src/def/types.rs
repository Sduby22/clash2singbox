use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Listen {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_fast_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_multi_path: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_fragment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_timeout: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sniff: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sniff_override_destination: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sniff_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_disable_domain_unmapping: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_interface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet4_bind_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inet6_bind_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mark: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse_addr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_fast_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_multi_path: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_fragment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_delay: Option<String>,
}

// TODO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InboundAcme {
    pub enabled: bool,
    pub pq_signature_schemes_enabled: bool,
    pub dynamic_record_sizing_disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_path: Option<String>,
}

// TODO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InboundEch {
    pub enabled: bool,
    pub pq_signature_schemes_enabled: bool,
    pub dynamic_record_sizing_disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutboundEch {
    pub enabled: bool,
    pub pq_signature_schemes_enabled: bool,
    pub dynamic_record_sizing_disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utls {
    pub enabled: bool,
    pub fingerprint: String,
}

// TODO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InboundReality {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutboundReality {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InboundTls {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_suites: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acme: Option<InboundAcme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ech: Option<InboundEch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality: Option<InboundReality>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutboundTls {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_sni: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_suites: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ech: Option<OutboundEch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utls: Option<Utls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality: Option<OutboundReality>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brutal {
    pub enabled: bool,
    pub up_mbps: u32,
    pub down_mbps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutboundMultiplex {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_streams: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_streams: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brutal: Option<Brutal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
#[serde(tag="type")]
pub enum V2rayTransport {
    Http(HttpTransport),
    Ws(WebsocketTransport),
    Quic(QUICTransport),
    Grpc(GrpcTransport),
    HttpUpgrade(HttpUpgradeTransport),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpTransport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_timeout: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebsocketTransport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_early_data: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_data_header_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QUICTransport {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrpcTransport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permit_without_stream: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpUpgradeTransport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<OneOrMany<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpOverTcp {
    pub enabled: bool,
    pub version: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authentication {
    pub username: String,
    pub password: String,
}