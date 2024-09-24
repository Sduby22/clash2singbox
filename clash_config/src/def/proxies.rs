use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
#[serde(tag="type")]
pub enum Proxy {
    #[serde(skip)]
    None,
    Direct(Direct),
    Http(Http),
    Socks(Socks),
    Ss(Shadowsocks),
    Vmess(Vmess),
    Vless(Vless),
    Trojan(Trojan),
    Hysteria2(Hysteria2),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct WsOpts {
    pub path: String,
    pub headers: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_early_data: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_data_header_name: Option<String>,
    pub v2ray_http_upgrade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct H2Opts {
    pub path: String,
    pub host: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct HttpOpts {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct GrpcOpts {
    pub grpc_service_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct RealityOpts {
    pub public_key: String,
    pub short_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct TlsOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_cert_verify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality_opts: Option<RealityOpts>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct ProxyBase {
    pub name: String,
    pub server: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mark: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialer_proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mptcp: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Direct {
    #[serde(flatten)]
    pub base: ProxyBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Http {
    #[serde(flatten)]
    pub base: ProxyBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_opts: Option<TlsOpts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Socks {
    #[serde(flatten)]
    pub base: ProxyBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_opts: Option<TlsOpts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub enum ShadowsocksPlugin {
    Obfs,
    V2rayPlugin
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Shadowsocks {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub cipher: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<ShadowsocksPlugin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_opts: Option<HashMap<String,String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_over_tcp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_over_tcp_version: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Vmess {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub cipher: String,
    pub uuid: String,
    #[serde(rename="alterId")]
    pub alter_id: u32,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_opts: Option<TlsOpts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws_opts: Option<WsOpts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2_opts: Option<H2Opts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_opts: Option<HttpOpts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_opts: Option<GrpcOpts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Vless {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub uuid: String,
    pub network: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_opts: Option<TlsOpts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xudp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_cert_verify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality_opts: Option<RealityOpts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_opts: Option<GrpcOpts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws_opts: Option<WsOpts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Trojan {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_opts: Option<TlsOpts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_opts: Option<GrpcOpts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws_opts: Option<WsOpts>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct Hysteria2 {
    #[serde(flatten)]
    pub base: ProxyBase,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfs_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub tls_opts: Option<TlsOpts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_str: Option<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestProxy {
        proxies: Vec<Proxy>,
    }

    #[test]
    fn proxies_ser() {
        let str = r#"
        proxies:
            - {"name":"Test-01","type":"ss","server":"test1.example.com","port":1234,"cipher":"chacha20-ietf-poly1305","password":"1234","udp":true}
            - name: "hysteria2"
              type: hysteria2
              server: server.com
              port: 443
              password: yourpassword
            - name: trojan-grpc
              server: server
              port: 443
              type: trojan
              password: "example"
              network: grpc
              sni: example.com
              udp: true
              grpc-opts:
                grpc-service-name: "example"
            - name: "vless-reality-grpc"
              type: vless
              server: server
              port: 443
              uuid: uuid
              network: grpc
              tls: true
              udp: true
              flow:
              client-fingerprint: chrome
              servername: testingcf.jsdelivr.net # REALITY servername
              grpc-opts:
                grpc-service-name: "grpc"
              reality-opts:
                public-key: CrrQSjAG_YkHLwvM2M-7XkKJilgL5upBKCp0od0tLhE
                short-id: 10f897e26c4b9478
        "#;
        let proxy: TestProxy = serde_yaml::from_str(&str).unwrap();
        let proxy0 = &proxy.proxies[0];
        if let Proxy::Ss(ss) = proxy0 {
            assert_eq!(ss.base.name, "Test-01");
            assert_eq!(ss.base.server, "test1.example.com");
            assert_eq!(ss.base.port, 1234);
            assert_eq!(ss.cipher, "chacha20-ietf-poly1305");
            assert_eq!(ss.password, "1234");
            assert_eq!(ss.base.udp, Some(true));
        } else {
            panic!("Expected Proxy::Ss, got {:?}", proxy0);
        }
        println!("{:#?}", proxy);
    }
}