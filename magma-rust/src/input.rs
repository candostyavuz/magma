use serde::{Deserialize, Serialize};

use crate::proposal::{NetworkName, ParseFunc};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApiMethod {
    JustMethod(String),
    WithArgs(ApiMethodWithArgs),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiMethodWithArgs {
    pub name: String,
    pub args: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InputTemplate(pub Vec<InputItem>);

#[derive(Deserialize, Debug, Clone)]
pub struct InputItem {
    pub chain_name: Option<String>,

    pub chain_index: NetworkName,

    #[serde(default)]
    pub api_methods: Vec<ApiMethod>,

    #[serde(default)]
    pub imports: Vec<NetworkName>,
}

impl ApiMethod {
    pub fn name(&self) -> &str {
        match self {
            ApiMethod::JustMethod(name) => name,
            ApiMethod::WithArgs(ApiMethodWithArgs { name, .. }) => name,
        }
    }

    pub fn parse_arg(&self) -> String {
        match self {
            ApiMethod::JustMethod(_) => "latest".to_string(),
            ApiMethod::WithArgs(ApiMethodWithArgs { args, .. }) => args.to_string(),
        }
    }

    pub fn parse_func(&self) -> ParseFunc {
        match self {
            ApiMethod::JustMethod(_) => ParseFunc::Default,
            ApiMethod::WithArgs(_) => ParseFunc::ParseByArg,
        }
    }
}
