use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use clap::{Parser, Subcommand};

use singbox_config::serialize as singbox_serialize;
use singbox_config::def::Config as SingboxConfig;
use singbox_config::def::outbounds::{Outbound, UrlTest, Direct, Block};
use singbox_config::def::routes::{Route, Rule as RouteRule};
use singbox_config::def::rules::Rule;
use singbox_config::def::types::OneOrMany;
use clash_config::parse as clash_parse;
use clash_config::def::Config as ClashConfig;
use clash_config::def::Rule as ClashRule;

use clash2singbox::converters;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Output to the specified file instead of stdout
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Pretty print the output json
    #[arg(short, long)]
    pretty: bool,
}

impl Args {
    pub fn run(&self) {
        let output: SingboxConfig = self.command.run();
        match &self.file {
            Some(f) => fs::write(f, singbox_serialize(&output, self.pretty).unwrap()).unwrap(),
            None => println!("{}", singbox_serialize(&output, self.pretty).unwrap()),
        };
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new empty singbox config
    New,

    /// Generate a singbox config from a clash config file
    #[command(arg_required_else_help = true)]
    Gen {
        /// The clash config file to input
        #[arg(short, long)]
        input: PathBuf,
        /// keep the original proxy groups and rules when generating config. If not set, program will generate a urltest outbound and a bare minimum route set instead
        #[arg(short, long)]
        keep_rules: bool,
    },

    /// update the specified singbox config file with specified elements
    #[command(arg_required_else_help = true)]
    Update {
        /// The clash config file to input
        #[arg(short, long)]
        input: Option<PathBuf>,
        /// The singbox config file to update
        #[arg(short, long)]
        update: PathBuf,
        /// The sections you want to update, space separated. valid values are: inbound, outbound, dns
        #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
        section: Vec<String>,
    },
}

impl Commands {
    pub fn run(&self) -> SingboxConfig {
        match self {
            Commands::New => Commands::new_sconfig(),
            Commands::Gen { input, keep_rules } => Commands::gen(input, keep_rules),
            Commands::Update { input, update, section } => Commands::update(input, update, section),
            
        }
    }

    fn new_sconfig() -> SingboxConfig {
        SingboxConfig{
            inbounds: vec![],
            outbounds: vec![],
            ..Default::default()
        }
    }

    fn gen(input: &PathBuf, keep_rules: &bool) -> SingboxConfig {
        let clash = clash_parse(&fs::read_to_string(input).unwrap()).unwrap();
        SingboxConfig{
            inbounds: converters::config_to_inbounds(&clash),
            outbounds: Commands::gen_outbound(&clash, *keep_rules),
            route: Some(Commands::gen_route(&clash, *keep_rules)),
            ..Default::default()
        }
    }

    fn update(_input: &Option<PathBuf>, _update: &PathBuf, section: &Vec<String>) -> SingboxConfig {
        let _ = section;
        let _ = _update;
        SingboxConfig{
            inbounds: vec![],
            outbounds: vec![],
            ..Default::default()
        }
    }

    fn gen_outbound(clash: &ClashConfig, keep_rules: bool) -> Vec<Outbound> {
        let direct = Direct {
            tag: "DIRECT".to_owned(),
            ..Default::default()
        };
        let block = Block {
            tag: "BLOCK".to_owned(),
        };
        let proxies: Vec<Outbound> = clash.proxies.as_ref().map_or(Vec::new(), |p| p.iter().map(converters::proxies_to_outbounds).filter(|o| !matches!(o, Outbound::None)).collect());
        if keep_rules {
            let proxy_groups: Vec<Outbound> = clash.proxy_groups.as_ref().map_or(Vec::new(), |p| p.iter().map(converters::proxygroups_to_outbounds).filter(|o| !matches!(o, Outbound::None)).collect());
            std::iter::once(Outbound::Direct(direct)).chain(std::iter::once(Outbound::Block(block))).chain(proxy_groups).chain(proxies).collect()
        }
        else {
            let urltest = UrlTest {
                tag: "auto".to_owned(),
                outbounds: proxies.iter().flat_map(|o| o.get_tag()).cloned().collect(),
                ..Default::default()
            };
            std::iter::once(Outbound::Direct(direct)).chain(std::iter::once(Outbound::Block(block))).chain(std::iter::once(Outbound::UrlTest(urltest))).chain(proxies).collect()
        }
    }

    fn gen_route(clash: &ClashConfig, keep_rules: bool) -> Route {
        if keep_rules {
            let rule_map: Option<HashMap<&String, Vec<&ClashRule>>> = clash.rules.as_ref().map(|v| v.iter().fold(HashMap::new(), Commands::accumulate_rule));
            let rules = match rule_map {
                Some(map) => {
                    let mut ret: Vec<RouteRule> = Vec::new();
                    for (k,v) in map {
                        ret.push(RouteRule {
                            outbound: k.clone(),
                            rule: Some(Rule {
                                domain: Commands::construct_rule_vec("DOMAIN", &v),
                                domain_suffix: Commands::construct_rule_vec("DOMAIN-SUFFIX", &v),
                                domain_keyword: Commands::construct_rule_vec("DOMAIN-KEYWORD", &v),
                                ip_cidr: Commands::construct_rule_vec("IP-CIDR", &v),
                                source_ip_cidr: Commands::construct_rule_vec("SRC-IP-CIDR", &v),
                                port: Commands::construct_port_vec("DST-PORT", &v), // TODO: port has special parsing rules, not implemented yet.
                                source_port: Commands::construct_port_vec("SRC-PORT", &v),
                                process_name: Commands::construct_rule_vec("PROCESS-NAME", &v),
                                process_path: Commands::construct_rule_vec("PROCESS-PATH", &v),
                                geosite: Commands::construct_rule_vec("GEOSITE", &v),
                                geoip: Commands::construct_rule_vec("GEOIP", &v),
                                inbound: Commands::construct_rule_vec("IN-NAME", &v),
                                network: Commands::construct_rule_vec("NETWORK", &v),
                                ..Default::default()
                            }),
                        });
                    }
                    ret
                },
                _ => Vec::new(),
            };
            Route {
                rules,
                ..Default::default()
            }
        }
        else {
            let block_ad_rule = RouteRule {
                outbound: "BLOCK".to_owned(),
                rule: Some(Rule {
                    geosite: Some(OneOrMany::One("category-ads-all".to_owned())),
                    ..Default::default()
                }),
            };
            let direct_rule = RouteRule {
                outbound: "DIRECT".to_owned(),
                rule: Some(Rule {
                    geosite: Some(OneOrMany::One("cn".to_owned())),
                    geoip: Some(OneOrMany::Many(vec!["cn".to_owned(), "private".to_owned()])),
                    ..Default::default()
                }),
            };
            Route {
                rules: vec![block_ad_rule, direct_rule],
                default: Some("auto".to_owned()),
                ..Default::default()
            }
        }
    }

    fn accumulate_rule<'a>(mut acc: HashMap<&'a String, Vec<&'a ClashRule>>, r: &'a ClashRule) -> HashMap<&'a String, Vec<&'a ClashRule>> {
        let v = acc.entry(&r.target).or_default();
        v.push(r);
        acc
    }

    fn construct_rule_vec(typ: &str, v: &[&ClashRule]) -> Option<OneOrMany<String>> {
        let k: Vec<&String> = v.iter().filter(|r| r.category == typ).map(|r| &r.content).collect();
        if k.is_empty() {
            None
        }
        else if k.len() == 1 {
            Some(OneOrMany::One(k[0].clone()))
        }
        else {
            Some(OneOrMany::Many(k.into_iter().cloned().collect()))
        }
    }

    fn construct_port_vec(typ: &str, v: &[&ClashRule]) -> Option<OneOrMany<u16>> {
        let k: Vec<u16> = v.iter().filter(|r| r.category == typ).flat_map(|r| r.content.parse()).collect();
        if k.is_empty() {
            None
        }
        else if k.len() == 1 {
            Some(OneOrMany::One(k[0]))
        }
        else {
            Some(OneOrMany::Many(k))
        }
    }
}