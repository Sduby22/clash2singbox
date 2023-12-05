use clash_config::def as cdef;
use singbox_config::def as sdef;

use super::conversion::Conversion;


pub fn config_to_inbounds(input: &cdef::Config) -> Vec<sdef::inbounds::Inbound> {
    let mut v: Vec<sdef::inbounds::Inbound> = Vec::new();
    if Option::is_some(&input.mixed_port) {
        v.push(sdef::inbounds::Inbound::Mixed(sdef::inbounds::Mixed::cfrom(&input)));
    }
    if Option::is_some(&input.port) {
        v.push(sdef::inbounds::Inbound::Http(sdef::inbounds::Http::cfrom(&input)));
    }
    if Option::is_some(&input.socks_port) {
        v.push(sdef::inbounds::Inbound::Socks(sdef::inbounds::Socks::cfrom(&input)));
    }
    if Option::is_some(&input.redir_port) {
        v.push(sdef::inbounds::Inbound::Redirect(sdef::inbounds::Redirect::cfrom(&input)));
    }
    if Option::is_some(&input.tproxy_port) {
        v.push(sdef::inbounds::Inbound::Tproxy(sdef::inbounds::Tproxy::cfrom(&input)));
    }
    v
}

pub fn proxies_to_outbounds(input: &cdef::proxies::Proxy) -> sdef::outbounds::Outbound {
    match input {
        cdef::proxies::Proxy::Ss(e) => sdef::outbounds::Outbound::Shadowsocks(sdef::outbounds::Shadowsocks::cfrom(&e)),
        cdef::proxies::Proxy::Trojan(e) => sdef::outbounds::Outbound::Trojan(sdef::outbounds::Trojan::cfrom(&e)),
        cdef::proxies::Proxy::Vless(e) => sdef::outbounds::Outbound::Vless(sdef::outbounds::Vless::cfrom(&e)),
        cdef::proxies::Proxy::Vmess(e) => sdef::outbounds::Outbound::Vmess(sdef::outbounds::Vmess::cfrom(&e)),
        cdef::proxies::Proxy::Hysteria2(e) => sdef::outbounds::Outbound::Hysteria2(sdef::outbounds::Hysteria2::cfrom(&e)),
        _ => sdef::outbounds::Outbound::None,
    }
}

pub fn proxygroups_to_outbounds(input: &cdef::proxy_groups::ProxyGroup) -> sdef::outbounds::Outbound {
    match input {
        cdef::proxy_groups::ProxyGroup::UrlTest(u) => sdef::outbounds::Outbound::UrlTest(sdef::outbounds::UrlTest::cfrom(&u)),
        cdef::proxy_groups::ProxyGroup::Select(s) => sdef::outbounds::Outbound::Selector(sdef::outbounds::Selector::cfrom(&s)),
        _ => sdef::outbounds::Outbound::None,
    }
}