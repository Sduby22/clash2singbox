use serde_yaml::Result;

pub mod def;


pub fn parse(config_content: &str) -> Result<def::Config> {
    serde_yaml::from_str(config_content)
}