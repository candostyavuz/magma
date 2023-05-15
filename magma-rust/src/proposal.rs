use crate::{
    constants,
    input::{ApiMethod, InputItem},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProposalFile {
    pub proposal: Proposal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub specs: Vec<Spec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    pub index: NetworkName,
    pub name: String,
    #[serde(default = "constants::spec_enabled")]
    pub enabled: bool,
    #[serde(default = "constants::reliability_threshold")]
    pub reliability_threshold: u32,
    #[serde(default = "constants::data_reliability_enabled")]
    pub data_reliability_enabled: bool,
    #[serde(default = "constants::block_distance_for_finalized_data")]
    pub block_distance_for_finalized_data: u64,
    #[serde(default = "constants::blocks_in_finalization_proof")]
    pub blocks_in_finalization_proof: u8,
    #[serde(default = "constants::average_block_time")]
    pub average_block_time: String,
    #[serde(default = "constants::allowed_block_lag_for_qos_sync")]
    pub allowed_block_lag_for_qos_sync: String,
    #[serde(default)]
    pub min_stake_provider: MinStake,
    #[serde(default)]
    pub min_stake_client: MinStake,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub imports: Vec<NetworkName>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub apis: Vec<ApiData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinStake {
    denom: String,
    amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiData {
    pub name: String,
    block_parsing: BlockParsingData,
    compute_units: String,
    enabled: bool,
    api_interfaces: Vec<ApiInterfaceData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ParseFunc {
    Empty,
    Default,
    ParseByArg,
    ParseCanonical,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockParsingData {
    parser_arg: Vec<String>,
    parser_func: ParseFunc,
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

impl From<InputItem> for Spec {
    fn from(input: InputItem) -> Self {
        use constants::*;

        let name = input
            .chain_name
            .clone()
            .unwrap_or_else(|| input.chain_index.to_string());

        let apis: Vec<ApiData> = input.api_methods.into_iter().map(Into::into).collect();

        Self {
            name,
            index: input.chain_index,
            imports: input.imports,
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
                parser_arg: vec![api.parse_arg()],
                parser_func: api.parse_func(),
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
