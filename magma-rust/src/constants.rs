pub const SPEC_ENABLED: bool = true;
pub const RELIABILITY_THRESHOLD: u32 = 268435455;
pub const DATA_RELIABILITY_ENABLED: bool = true;
pub const BLOCK_DISTANCE_FOR_FINALIZED_DATA: u64 = 64;
pub const BLOCKS_IN_FINALIZATION_PROOF: u8 = 3;
pub const AVERAGE_BLOCK_TIME: &str = "13000";
pub const ALLOWED_BLOCK_LAG_FOR_QOS_SYNC: &str = "2";
pub const DENOM: &str = "ulava";
pub const AMOUNT: &str = "50000000000";

pub const COMPUTE_UNITS: &str = "10";
pub const API_ENABLED: bool = true;

pub mod api_interfaces {
    pub const INTERFACE: &str = "jsonrpc";
    pub const TYPE: &str = "POST";
    pub const EXTRA_COMPUTE_UNITS: &str = "0";

    pub mod category_data {
        pub const DETERMINISTIC: bool = false;
        pub const LOCAL: bool = false;
        pub const SUBSCRIPTION: bool = false;
        pub const STATEFUL: i32 = 0;
    }
}

pub fn spec_enabled() -> bool {
    SPEC_ENABLED
}

pub fn reliability_threshold() -> u32 {
    RELIABILITY_THRESHOLD
}

pub fn data_reliability_enabled() -> bool {
    DATA_RELIABILITY_ENABLED
}

pub fn block_distance_for_finalized_data() -> u64 {
    BLOCK_DISTANCE_FOR_FINALIZED_DATA
}

pub fn blocks_in_finalization_proof() -> u8 {
    BLOCKS_IN_FINALIZATION_PROOF
}

pub fn average_block_time() -> String {
    AVERAGE_BLOCK_TIME.to_string()
}

pub fn allowed_block_lag_for_qos_sync() -> String {
    ALLOWED_BLOCK_LAG_FOR_QOS_SYNC.to_string()
}

pub fn denom() -> String {
    DENOM.to_string()
}

pub fn amount() -> String {
    AMOUNT.to_string()
}
