package magma

import (
	"encoding/json"
	"fmt"
	"gopkg.in/yaml.v3"
	"io"
	"os"
	"strconv"
	"strings"
)

type Proposal struct {
	Title       string `json:"title"`
	Description string `json:"description"`
	Specs       []Spec `json:"specs"`
}

type Spec struct {
	Index                         string      `json:"index"`
	Name                          string      `json:"name"`
	Enabled                       bool        `json:"enabled"`
	ReliabilityThreshold          uint32      `json:"reliability_threshold"`
	DataReliabilityEnabled        bool        `json:"data_reliability_enabled"`
	BlockDistanceForFinalizedData uint64      `json:"block_distance_for_finalized_data"`
	BlocksInFinalizationProof     uint8       `json:"blocks_in_finalization_proof"`
	AverageBlockTime              string      `json:"average_block_time"`
	AllowedBlockLagForQosSync     string      `json:"allowed_block_lag_for_qos_sync"`
	MinStakeProvider              MinStake    `json:"min_stake_provider"`
	MinStakeClient                MinStake    `json:"min_stake_client"`
	APIs                          APIDataList `json:"apis"`
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
	Imports       []string           `json:"imports"`
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

type APIDataList struct {
	Apis []APIData `json:"apis"`
}

type InputTemplate struct {
	ChainType  string      `yaml:"chain-type"`
	APIMethods []APIMethod `yaml:"api_methods"`
}

type APIMethod struct {
	Name string `yaml:"name"`
	Args int    `yaml:"args"`
}

// LOGIC:
func GenerateSpec(fileName string, chainNameFlag string, chainIdxFlag string, imports []string) error {

	// Check if fileName has ".yaml" extension, and add it if not
	if !strings.HasSuffix(fileName, ".yaml") {
		fileName += ".yaml"
	}

	// Open the file
	file, err := os.Open(fileName)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return err
	}
	defer file.Close()

	// Read the file contents into memory as a byte slice
	fileBytes, err := io.ReadAll(file)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return err
	}

	// Load the Input Template and Unmarshal the YAML data into memory
	schema := &InputTemplate{}
	err = yaml.Unmarshal(fileBytes, schema)
	if err != nil {
		fmt.Println("Error unmarshalling YAML:", err)
		return err
	}

	//build structure to contain the Output
	data := APIDataList{
		Apis: make([]APIData, 0),
	}

	//iterate through the API methods and pass them into the Output structure
	for _, method := range schema.APIMethods {
		parseFunc := "DEFAULT"
		parseArg := []string{"latest"}

		if method.Args > 0 {
			parseArg = []string{strconv.Itoa(method.Args)}
			parseFunc = "PARSE_BY_ARG"
		}

		newData := APIData{
			Name: method.Name,
			BlockParsing: BlockParsingData{
				ParserArg:  parseArg,
				ParserFunc: parseFunc,
			},
			ComputeUnits: "10",
			Enabled:      true,
			Imports:      imports,
			ApiInterfaces: []ApiInterfaceData{
				{
					Category: CategoryData{
						Deterministic: false,
						Local:         false,
						Subscription:  false,
						Stateful:      0,
					},
					Interface:         "jsonrpc",
					Type:              "POST",
					ExtraComputeUnits: "0",
				},
			},
		}
		data.Apis = append(data.Apis, newData)
		fmt.Printf("Method Implemented: %v \n", method.Name)
	}

	fmt.Printf("TOTAL METHODS IMPLEMENTED: %d  \n", len(schema.APIMethods))

	// Write the JSON data to a file
	err = WriteJSONFile("output.json", data, chainNameFlag, chainIdxFlag)
	if err != nil {
		fmt.Println("Error writing JSON file:", err)
		return nil
	}
	fmt.Println("JSON file written successfully.")

	return nil
}

func WriteJSONFile(fileName string, data APIDataList, chainNameFlag string, chainIdxFlag string) error {
	// Write the JSON data to a file
	file, err := os.Create(fileName)
	if err != nil {
		return err
	}
	defer file.Close()

	dataWithHeader := Proposal{
		Title:       "Add Specs: " + chainNameFlag,
		Description: "Adding new specification support for relaying " + chainNameFlag + " data on Lava",
		Specs: []Spec{
			{Index: chainIdxFlag,
				Name:                          chainNameFlag,
				Enabled:                       Enabled,
				ReliabilityThreshold:          ReliabilityThreshold,
				DataReliabilityEnabled:        DataReliabilityEnabled,
				BlockDistanceForFinalizedData: BlockDistanceForFinalizedData,
				BlocksInFinalizationProof:     BlocksInFinalizationProof,
				AverageBlockTime:              AverageBlockTime,
				AllowedBlockLagForQosSync:     AllowedBlockLagForQosSync,
				MinStakeProvider: MinStake{
					Denom:  Denom,
					Amount: Amount,
				},
				MinStakeClient: MinStake{
					Denom:  Denom,
					Amount: Amount,
				},
				APIs: data,
			},
		},
	}
	// Marshal the header into JSON format
	jsonData, err := json.MarshalIndent(dataWithHeader, "", "    ")
	if err != nil {
		return err
	}

	_, err = file.Write(jsonData)
	if err != nil {
		return err
	}

	return nil
}
