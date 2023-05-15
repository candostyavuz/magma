package magma

import (
	"encoding/json"
	"os"
)

func CreateSpecWithHeader(data []APIData, importedChains []string, chainNameFlag string, chainIdxFlag string) Proposal {
	headerData := Proposal{
		Title:       "Add Specs: " + chainNameFlag,
		Description: "Adding new specification support for relaying " + chainNameFlag + " data on Lava",
		Specs: []Spec{
			{Index: chainIdxFlag,
				Name:                          chainNameFlag,
				Enabled:                       Enabled,
				Imports:                       handleImportsFormat(importedChains),
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
	return headerData
}

func WriteJSONFile(dataWithHeader Proposal) error {
	fileName := "output.json"
	// Write the JSON data to a file
	file, err := os.Create(fileName)
	if err != nil {
		return err
	}
	defer file.Close()

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
