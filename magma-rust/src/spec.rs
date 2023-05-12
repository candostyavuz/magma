use crate::constants;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    index: String,
    name: String,
    #[serde(default = "constants::enabled")]
    enabled: bool,
    #[serde(default = "constants::reliability_threshold")]
    reliability_threshold: u32,
    #[serde(default = "constants::data_reliability_enabled")]
    data_reliability_enabled: bool,
    #[serde(default = "constants::block_distance_for_finalized_data")]
    block_distance_for_finalized_data: u64,
    #[serde(default = "constants::blocks_in_finalization_proof")]
    blocks_in_finalization_proof: u8,
    #[serde(default = "constants::average_block_time")]
    average_block_time: String,
    #[serde(default = "constants::allowed_block_lag_for_qos_sync")]
    allowed_block_lag_for_qos_sync: String,
    min_stake_provider: MinStake,
    min_stake_client: MinStake,
    apis: ApiDataList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinStake {
    denom: String,
    amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiDataList {
    pub apis: Vec<ApiData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiData {
    name: String,
    block_parsing: BlockParsingData,
    compute_units: String,
    enabled: bool,
    api_interfaces: Vec<ApiInterfaceData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockParsingData {
    parse_arg: Vec<String>,
    parse_func: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiInterfaceData {
    pub category: CategoryData,
    pub interface: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub extra_compute_units: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryData {
    pub deterministic: bool,
    pub local: bool,
    pub subscription: bool,
    pub stateful: i32,
}
