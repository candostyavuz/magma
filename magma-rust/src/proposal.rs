use crate::{constants, input::ApiMethod};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Proposal {
    title: String,
    description: String,
    specs: Vec<Spec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    index: NetworkName,
    name: String,
    #[serde(default = "constants::spec_enabled")]
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
    #[serde(default)]
    min_stake_provider: MinStake,
    #[serde(default)]
    min_stake_client: MinStake,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub apis: Vec<ApiData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinStake {
    denom: String,
    amount: String,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, derive_more::Display)]
pub enum NetworkName {
    ALFAJORES,
    APT1,
    ARB1,
    ARBN,
    AXELAR,
    AXELART,
    BASET,
    CANTO,
    CELO,
    COS3,
    COS4,
    COS5,
    COS5T,
    ETH1,
    EVMOS,
    EVMOST,
    FTM250,
    GTH1,
    JUN1,
    JUN1T,
    JUNO,
    JUNOT,
    LAV1,
    OPTM,
    OPTMT,
    POLYGON1,
    POLYGON1T,
    STRK,
}

impl Default for MinStake {
    fn default() -> Self {
        Self {
            denom: constants::denom(),
            amount: constants::amount(),
        }
    }
}

impl Proposal {
    pub fn new(title: String, description: String, specs: Vec<Spec>) -> Self {
        Self {
            title,
            description,
            specs,
        }
    }
}

impl Spec {
    pub fn new(name: String, index: NetworkName, apis: Vec<ApiData>) -> Self {
        use constants::*;

        Self {
            name,
            index,
            apis,
            enabled: spec_enabled(),
            reliability_threshold: reliability_threshold(),
            data_reliability_enabled: data_reliability_enabled(),
            block_distance_for_finalized_data: block_distance_for_finalized_data(),
            blocks_in_finalization_proof: blocks_in_finalization_proof(),
            average_block_time: average_block_time(),
            allowed_block_lag_for_qos_sync: allowed_block_lag_for_qos_sync(),
            min_stake_provider: Default::default(),
            min_stake_client: Default::default(),
        }
    }
}

impl From<ApiMethod> for ApiData {
    fn from(api: ApiMethod) -> Self {
        Self {
            name: api.name().to_string(),
            block_parsing: BlockParsingData {
                parse_arg: vec!["latest".to_string()],
                parse_func: "DEFAULT".to_string(),
            },
            compute_units: constants::COMPUTE_UNITS.to_string(),
            enabled: constants::API_ENABLED,
            api_interfaces: vec![ApiInterfaceData::new()],
        }
    }
}

impl ApiInterfaceData {
    pub fn new() -> Self {
        use crate::constants::api_interfaces::*;

        Self {
            category: CategoryData::new(),
            interface: INTERFACE.to_string(),
            _type: TYPE.to_string(),
            extra_compute_units: EXTRA_COMPUTE_UNITS.to_string(),
        }
    }
}

impl CategoryData {
    pub fn new() -> Self {
        use crate::constants::api_interfaces::category_data::*;

        Self {
            deterministic: DETERMINISTIC,
            local: LOCAL,
            subscription: SUBSCRIPTION,
            stateful: STATEFUL,
        }
    }
}

impl Default for CategoryData {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ApiInterfaceData {
    fn default() -> Self {
        Self::new()
    }
}
