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

impl Conversion<cdef::proxies::ProxyBase> for sdef::types::Dial {
    fn cfrom(from: &cdef::proxies::ProxyBase) -> Self {
        sdef::types::Dial {
            detour: from.dialer_proxy.clone(),
            bind_interface: from.interface_name.clone(),
            routing_mark: from.routing_mark,
            tcp_fast_open: from.tfo,
            tcp_multi_path: from.mptcp,
            ..Default::default()
        }
    }
}

impl Conversion<cdef::proxies::TlsOpts> for sdef::types::OutboundTls {
    fn cfrom(from: &cdef::proxies::TlsOpts) -> Self {
        sdef::types::OutboundTls {
            enabled: from.tls.unwrap_or(false),
            server_name: from.sni.as_ref().or(from.servername.as_ref()).cloned(),
            insecure: from.skip_cert_verify,
            ..Default::default()
        }
    }
}

impl Conversion<cdef::proxies::ShadowsocksPlugin> for sdef::outbounds::ShadowsocksPlugin {
    fn cfrom(from: &cdef::proxies::ShadowsocksPlugin) -> Self {
        match from {
            cdef::proxies::ShadowsocksPlugin::Obfs => sdef::outbounds::ShadowsocksPlugin::ObfsLocal,
            cdef::proxies::ShadowsocksPlugin::V2rayPlugin => sdef::outbounds::ShadowsocksPlugin::V2rayPlugin,
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
            network: if from.base.udp == Some(true) {None} else {Some("tcp".to_owned())},
            plugin: from.plugin.as_ref().map(sdef::outbounds::ShadowsocksPlugin::cfrom),
            plugin_opts: None,
            udp_over_tcp: Some(false),
            multiplex: None,
            dial: Some(sdef::types::Dial::cfrom(&from.base)),
        }
    }
}

impl Conversion<cdef::proxies::Trojan> for sdef::outbounds::Trojan {
    fn cfrom(from: &cdef::proxies::Trojan) -> Self {
        let tls =from.tls_opts.as_ref().map(|e| {
           let mut tls = sdef::types::OutboundTls::cfrom(e);
           tls.enabled = true;
           tls
        });
        sdef::outbounds::Trojan {
            tag: from.base.name.clone(),
            server: from.base.server.clone(),
            server_port: from.base.port,
            password: from.password.clone(),
            network: if from.base.udp == Some(true) {None} else {Some("tcp".to_owned())},
            dial: Some(sdef::types::Dial::cfrom(&from.base)),
            tls,
            ..Default::default()
        }
    }
}

impl Conversion<cdef::proxies::Vmess> for sdef::outbounds::Vmess {
    fn cfrom(_from: &cdef::proxies::Vmess) -> Self {
        todo!()
    }
}

impl Conversion<cdef::proxies::Vless> for sdef::outbounds::Vless {
    fn cfrom(_from: &cdef::proxies::Vless) -> Self {
        todo!()
    }
}

impl Conversion<cdef::proxies::Hysteria2> for sdef::outbounds::Hysteria2 {
    fn cfrom(_from: &cdef::proxies::Hysteria2) -> Self {
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