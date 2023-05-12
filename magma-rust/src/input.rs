use serde::{Deserialize, Serialize};

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
pub struct InputTemplate {
    pub chain_name: Option<String>,

    pub chain_index: String,
    pub api_methods: Vec<ApiMethod>,
}

impl ApiMethod {
    pub fn name(&self) -> &str {
        match self {
            ApiMethod::JustMethod(name) => name,
            ApiMethod::WithArgs(ApiMethodWithArgs { name, .. }) => name,
        }
    }
}
