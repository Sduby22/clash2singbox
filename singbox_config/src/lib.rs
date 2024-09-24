pub mod def;
pub mod macros;

pub fn serialize(config: &def::Config, pretty: bool) -> Result<String, serde_json::Error> {
    if pretty {
        serde_json::to_string_pretty(config)
    } else {
        serde_json::to_string(config)
    }
}

pub fn deserialize(config: &str) -> Result<def::Config, serde_json::Error> {
    serde_json::from_str(config)
}
