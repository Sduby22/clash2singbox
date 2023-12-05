pub mod def;


pub fn parse(config_content: &str) -> def::Config {
    serde_yaml::from_str(config_content).unwrap()
}