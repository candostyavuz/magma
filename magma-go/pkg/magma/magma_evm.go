package magma

import (
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
	data := make([]APIData, 0)

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
		data = append(data, newData)
		fmt.Printf("Method Implemented: %v \n", method.Name)
	}

	fmt.Printf("TOTAL METHODS IMPLEMENTED: %d  \n", len(schema.APIMethods))

	dataWithHeader := CreateSpecWithHeader(data, imports, chainNameFlag, chainIdxFlag)
	// Write the JSON data to a file
	err = WriteJSONFile(dataWithHeader)
	if err != nil {
		fmt.Println("Error writing JSON file:", err)
		return err
	}
	fmt.Println("JSON file written successfully.")

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
