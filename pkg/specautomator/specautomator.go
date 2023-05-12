package specautomator

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func GenerateSpec(fileName string) error {

	// Check if fileName has ".txt" extension, and add it if not
	if !strings.HasSuffix(fileName, ".txt") {
		fileName += ".txt"
	}

	// Open the file
	file, err := os.Open(fileName)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return err
	}
	defer file.Close()

	// Read the file contents into memory as a byte slice
	fileBytes, err := ioutil.ReadAll(file)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return err
	}

	// Convert the byte slice to a string and split it into lines
	fileContent := string(fileBytes)
	lines := strings.Split(fileContent, "\n")

	// Iterate through the lines
	for _, line := range lines {
		fmt.Println(line)
	}

	return nil
}
