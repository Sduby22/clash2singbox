use clash_config::def as cdef;
use singbox_config::def::{self as sdef, types::OutboundTls};


pub trait Conversion<T> {
    fn cfrom(from: &T) -> Self;
}

impl Conversion<cdef::Config> for sdef::inbounds::Mixed {
    fn cfrom(from: &cdef::Config) -> Self {
        let listen = sdef::types::Listen {
            listen: from.bind_address.clone(),
            listen_port: from.mixed_port,
            ..Default::default()
        };
        sdef::inbounds::Mixed {
            tag: "mix-in".to_owned(),
            listen: Some(listen),
            set_system_proxy: Some(false),
        }
    }
}

impl Conversion<cdef::Config> for sdef::inbounds::Http {
    fn cfrom(from: &cdef::Config) -> Self {
        let listen = sdef::types::Listen {
            listen: from.bind_address.clone(),
            listen_port: from.port,
            ..Default::default()
        };
        sdef::inbounds::Http {
            tag: "http-in".to_owned(),
            listen: Some(listen),
            set_system_proxy: Some(false),
            ..Default::default()
        }
    }
}

impl Conversion<cdef::Config> for sdef::inbounds::Socks {
    fn cfrom(from: &cdef::Config) -> Self {
        let listen = sdef::types::Listen {
            listen: from.bind_address.clone(),
            listen_port: from.socks_port,
            ..Default::default()
        };
        sdef::inbounds::Socks {
            tag: "socks-in".to_owned(),
            listen: Some(listen),
            ..Default::default()
        }
    }
}

impl Conversion<cdef::Config> for sdef::inbounds::Redirect {
    fn cfrom(from: &cdef::Config) -> Self {
        let listen = sdef::types::Listen {
            listen: from.bind_address.clone(),
            listen_port: from.redir_port,
            ..Default::default()
        };
        sdef::inbounds::Redirect {
            tag: "redir-in".to_owned(),
            listen: Some(listen),
            ..Default::default()
        }
    }
}

impl Conversion<cdef::Config> for sdef::inbounds::Tproxy {
    fn cfrom(from: &cdef::Config) -> Self {
        let listen = sdef::types::Listen {
            listen: from.bind_address.clone(),
            listen_port: from.tproxy_port,
            ..Default::default()
        };
        sdef::inbounds::Tproxy {
            tag: "tproxy-in".to_owned(),
            listen: Some(listen),
            ..Default::default()
        }
    }
}

impl Conversion<cdef::proxies::Shadowsocks> for sdef::outbounds::Shadowsocks {
    fn cfrom(from: &cdef::proxies::Shadowsocks) -> Self {
        sdef::outbounds::Shadowsocks{
            tag: from.base.name.clone(),
            server: from.base.server.clone(),
            server_port: from.base.port,
            method: from.cipher.clone(),
            password: from.password.clone(),
            network: Some(if from.base.udp == Some(true) {"udp".to_owned()} else {"tcp".to_owned()}),
            plugin: from.plugin.clone(),
            plugin_opts: None,
            udp_over_tcp: Some(false),
            multiplex: None,
            dial: None,
        }
    }
}

impl Conversion<cdef::proxies::Trojan> for sdef::outbounds::Trojan {
    fn cfrom(from: &cdef::proxies::Trojan) -> Self {
        sdef::outbounds::Trojan {
            tag: from.base.name.clone(),
            server: from.base.server.clone(),
            server_port: from.base.port,
            password: from.password.clone(),
            network: Some(if from.base.udp == Some(true) {"udp".to_owned()} else {"tcp".to_owned()}),
            tls: Some(OutboundTls {
                enabled: true,
                server_name: from.sni.clone(),
                insecure: from.skip_cert_verify,
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

impl Conversion<cdef::proxies::Vmess> for sdef::outbounds::Vmess {
    fn cfrom(from: &cdef::proxies::Vmess) -> Self {
        todo!()
    }
}

impl Conversion<cdef::proxies::Vless> for sdef::outbounds::Vless {
    fn cfrom(from: &cdef::proxies::Vless) -> Self {
        todo!()
    }
}

impl Conversion<cdef::proxies::Hysteria2> for sdef::outbounds::Hysteria2 {
    fn cfrom(from: &cdef::proxies::Hysteria2) -> Self {
        todo!()
    }
}

impl Conversion<cdef::proxy_groups::Select> for sdef::outbounds::Selector {
    fn cfrom(from: &cdef::proxy_groups::Select) -> Self {
        sdef::outbounds::Selector{
            tag: from.name.clone(),
            outbounds: from.proxies.clone(),
            default: None,
            interrupt_exist_connections: None,
        }
    }
}

impl Conversion<cdef::proxy_groups::UrlTest> for sdef::outbounds::UrlTest {
    fn cfrom(from: &cdef::proxy_groups::UrlTest) -> Self {
        sdef::outbounds::UrlTest{
            tag: from.name.clone(),
            outbounds: from.proxies.clone(),
            url: Some(from.url.clone()),
            interval: Some(format!("{}s", from.interval)),
            idle_timeout: None,
            tolerance: from.tolerance,
            interrupt_exist_connections: None
        }
    }
}