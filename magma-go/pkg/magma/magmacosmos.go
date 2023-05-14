package magma

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func GenerateCosmosSpec(endpoint string) error {
	fmt.Println("endpoint in func :", endpoint)
	cmd := exec.Command("grpcurl", "-plaintext", endpoint, "list")
	out, err := cmd.Output()
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}
	// fmt.Println(string(out))

	//build structure to contain the Output
	data := APIDataList{
		Apis: make([]APIData, 0),
	}

	subcommands := strings.Split(strings.TrimSpace(string(out)), "\n")

	for _, subcommand := range subcommands {
		if subcommand == "" {
			continue
		}
		if strings.HasPrefix(subcommand, "osmosis") {
			cmd := exec.Command("grpcurl", "-plaintext", endpoint, "describe", subcommand)
			out, err := cmd.Output()
			if err != nil {
				fmt.Println("Error:", err)
				os.Exit(1)
			}

			lines := strings.Split(string(out), "\n")
			for _, line := range lines {

				if strings.Contains(line, "option (.google.api.http) = { get:") {
					line = strings.TrimSpace(line)
					line = strings.TrimPrefix(line, "option (.google.api.http) = { get:")
					line = strings.TrimSuffix(line, " };")
					line = strings.ReplaceAll(line, "\"", "")

					parseFunc := "DEFAULT"
					parseArg := []string{"latest"}

					newData := APIData{
						Name: line,
						BlockParsing: BlockParsingData{
							ParserArg:  parseArg,
							ParserFunc: parseFunc,
						},
						ComputeUnits: "10",
						Enabled:      true,
						ApiInterfaces: []ApiInterfaceData{
							{
								Category: CategoryData{
									Deterministic: false,
									Local:         false,
									Subscription:  false,
									Stateful:      0,
								},
								Interface:         "rest",
								Type:              "GET",
								ExtraComputeUnits: "0",
							},
						},
					}
					data.Apis = append(data.Apis, newData)
					fmt.Printf("Method Implemented: %v \n", line)
				}
			}
		}
	}

	fmt.Printf("TOTAL METHODS IMPLEMENTED: %d  \n", len(subcommands))

	// Write the JSON data to a file
	err = WriteJSONFileCosmos("outputCosmos.json", data)
	if err != nil {
		fmt.Println("Error writing JSON file:", err)
		return nil
	}
	fmt.Println("JSON file written successfully.")

	return nil
}

func WriteJSONFileCosmos(fileName string, data APIDataList) error {
	// Write the JSON data to a file
	file, err := os.Create(fileName)
	if err != nil {
		return err
	}
	defer file.Close()

	dataWithHeader := Proposal{
		Title:       "Add Specs: ",
		Description: "Adding new specification support for relaying ",
		Specs: []Spec{
			{Index: "",
				Name:                          "",
				Enabled:                       Enabled,
				Imports:                       nil,
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
