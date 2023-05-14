package magma

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"

	"gopkg.in/yaml.v3"
)

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
	err = WriteJSONFile("output.json", data, chainNameFlag, chainIdxFlag, imports)
	if err != nil {
		fmt.Println("Error writing JSON file:", err)
		return nil
	}
	fmt.Println("JSON file written successfully.")

	return nil
}

func WriteJSONFile(fileName string, data APIDataList, chainNameFlag string, chainIdxFlag string, imports []string) error {
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
				Imports:                       handleImportsFormat(imports),
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

func handleImportsFormat(imports []string) []string {
	var newImports []string
	for _, imp := range imports {
		imp = strings.TrimSpace(imp)
		if imp == "" {
			continue
		}
		parts := strings.Split(imp, " ")
		newImports = append(newImports, parts...)
	}
	return newImports
}
