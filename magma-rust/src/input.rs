use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ApiMethod {
    JustMethod(String),
    WithArgs(ApiMethodWithArgs),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiMethodWithArgs {
    pub name: String,
    pub args: i32,
}

#[derive(Deserialize, Debug)]
pub struct InputTemplate {
    pub chain_type: String,
    pub api_methods: Vec<ApiMethod>,
}
