package magma

import (
	"fmt"
	"os/exec"
	"strings"
)

func GenerateCosmosSpec(endpoint string, chainNameFlag string, chainIdxFlag string) error {
	cmd := exec.Command("grpcurl", "-plaintext", endpoint, "list")
	out, err := cmd.Output()
	if err != nil {
		return err
	}

	var importableMethods []string

	//build structure to contain the Output
	data := make([]APIData, 0)

	subcommands := strings.Split(strings.TrimSpace(string(out)), "\n")

	for _, subcommand := range subcommands {
		if subcommand == "" {
			continue
		}

		if isIgnoredMethod(subcommand) {
			continue
		}

		if isBaseChain(subcommand) {
			importableMethods = append(importableMethods, subcommand)
			continue
		}

		cmd := exec.Command("grpcurl", "-plaintext", endpoint, "describe", subcommand)
		out, err := cmd.Output()
		if err != nil {
			return err
		}

		lines := strings.Split(string(out), "\n")
		for _, line := range lines {

			if strings.Contains(line, "option (.google.api.http) = { get:") {
				line = strings.TrimSpace(line)
				line = strings.TrimPrefix(line, "option (.google.api.http) = { get:")
				line = strings.TrimSuffix(line, " };")
				line = strings.ReplaceAll(line, "\"", "")

				// check if description includes base method
				if isBaseChain(line) {
					continue
				}
				// check if description includes ignored method
				if isIgnoredMethod(line) {
					continue
				}

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
				data = append(data, newData)
				fmt.Printf("Method Implemented: %v \n", line)
			}
		}

	}

	fmt.Printf("TOTAL METHODS IMPLEMENTED: %d  \n", len(subcommands))

	importedSpecs := handleCosmosImports(importableMethods)

	// Generate header of the spec
	dataWithHeader := CreateSpecWithHeader(data, importedSpecs, chainNameFlag, chainIdxFlag)
	// Write the JSON data to a file
	err = WriteJSONFile(dataWithHeader)
	if err != nil {
		fmt.Println("Error writing JSON file:", err)
		return err
	}
	fmt.Println("JSON file written successfully.")

	return nil
}

func isIgnoredMethod(subcommand string) bool {
	if strings.HasPrefix(subcommand, "/") {
		subcommand = strings.TrimPrefix(subcommand, "/")
	}

	for _, str := range ignoreMethodPrefix {
		if strings.HasPrefix(subcommand, str) {
			return true
		}
	}
	return false
}

func isBaseChain(subcommand string) bool {
	if strings.HasPrefix(subcommand, "/") {
		subcommand = strings.TrimPrefix(subcommand, "/")
	}

	for _, str := range importedMethodPrefix {
		if strings.HasPrefix(subcommand, str) {
			return true
		}
	}
	return false
}

func contains(array []string, item string) bool {
	for _, element := range array {
		if element == item {
			return true
		}
	}
	return false
}

func handleCosmosImports(importableMethods []string) []string {
	importsOut := []string{}
	var isCosmossdk bool = false
	var isCosmwasm bool = false
	var isIbc bool = false

	for _, method := range importableMethods {
		if strings.HasPrefix(method, "cosmos") && !isCosmossdk {
			isCosmossdk = true
		} else if strings.HasPrefix(method, "cosmwasm") && !isCosmwasm {
			isCosmwasm = true
		} else if strings.HasPrefix(method, "ibc") && !isIbc {
			isIbc = true
		} else {
			continue
		}
	}

	if isCosmossdk && isCosmwasm {
		importsOut = append(importsOut, importedChainNames[3])
	} else if isCosmossdk {
		importsOut = append(importsOut, importedChainNames[0])
	} else if isCosmwasm {
		importsOut = append(importsOut, importedChainNames[1])
	}

	if !isCosmossdk && isIbc {
		importsOut = append(importsOut, importedChainNames[2])
	}
	return importsOut
}
