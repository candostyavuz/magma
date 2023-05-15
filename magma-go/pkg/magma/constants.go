package magma

const (
	Enabled                       = true
	ReliabilityThreshold          = 268435455
	DataReliabilityEnabled        = true
	BlockDistanceForFinalizedData = 64
	BlocksInFinalizationProof     = 3
	AverageBlockTime              = "13000"
	AllowedBlockLagForQosSync     = "2"
	Denom                         = "ulava"
	Amount                        = "50000000000"
)

var importedMethodPrefix = [3]string{"cosmos", "cosmwasm", "ibc"}
var ignoreMethodPrefix = [4]string{"grpc", "icq", "testdata", "gaia"}
var importedChainNames = [4]string{"COSMOSSDK", "COSMOSWASM", "IBC", "COSMOSSDKFULL"}

type Proposal struct {
	Title       string `json:"title"`
	Description string `json:"description"`
	Specs       []Spec `json:"specs"`
}

type Spec struct {
	Index                         string    `json:"index"`
	Name                          string    `json:"name"`
	Enabled                       bool      `json:"enabled"`
	Imports                       []string  `json:"imports"`
	ReliabilityThreshold          uint32    `json:"reliability_threshold"`
	DataReliabilityEnabled        bool      `json:"data_reliability_enabled"`
	BlockDistanceForFinalizedData uint64    `json:"block_distance_for_finalized_data"`
	BlocksInFinalizationProof     uint8     `json:"blocks_in_finalization_proof"`
	AverageBlockTime              string    `json:"average_block_time"`
	AllowedBlockLagForQosSync     string    `json:"allowed_block_lag_for_qos_sync"`
	MinStakeProvider              MinStake  `json:"min_stake_provider"`
	MinStakeClient                MinStake  `json:"min_stake_client"`
	APIs                          []APIData `json:"apis"`
}

type MinStake struct {
	Denom  string `json:"denom"`
	Amount string `json:"amount"`
}

type APIData struct {
	Name          string             `json:"name"`
	BlockParsing  BlockParsingData   `json:"block_parsing"`
	ComputeUnits  string             `json:"compute_units"`
	Enabled       bool               `json:"enabled"`
	ApiInterfaces []ApiInterfaceData `json:"api_interfaces"`
}

type BlockParsingData struct {
	ParserArg  []string `json:"parser_arg"`
	ParserFunc string   `json:"parser_func"`
}

type ApiInterfaceData struct {
	Category          CategoryData `json:"category"`
	Interface         string       `json:"interface"`
	Type              string       `json:"type"`
	ExtraComputeUnits string       `json:"extra_compute_units"`
}

type CategoryData struct {
	Deterministic bool `json:"deterministic"`
	Local         bool `json:"local"`
	Subscription  bool `json:"subscription"`
	Stateful      int  `json:"stateful"`
}

type InputTemplate struct {
	ChainType  string      `yaml:"chain-type"`
	APIMethods []APIMethod `yaml:"api_methods"`
}

type APIMethod struct {
	Name string `yaml:"name"`
	Args int    `yaml:"args"`
}
