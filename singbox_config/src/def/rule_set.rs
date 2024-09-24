use serde::{Deserialize, Serialize};

use crate::gen_enum;

use super::{rules::Rule, types::OneOrMany};

gen_enum!(RuleSet, Local, Remote, Inline);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inline {
    tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<OneOrMany<Rule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Local {
    tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>, // "source" or "binary"
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RemoteFormat {
    Source,
    Binary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remote {
    tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<RemoteFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    download_detour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_interval: Option<String>,
}
