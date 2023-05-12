package specautomator

import (
	"fmt"

	"github.com/candostyavuz/specautomator/pkg/specautomator"
	"github.com/spf13/cobra"
)

var genspecCmd = &cobra.Command{
	Use:     "genspec [supported-apis-file]",
	Aliases: []string{"gen"},
	Short:   "Generates a valid spec file from a list of supported api calls",
	Long: `Generates a valid spec file from a list of supported api calls.
	Currently, the only supported input format for the spec file is txt file.`,
	Args: cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		fmt.Println("File is: ", args[0])
		err := specautomator.GenerateSpec(args[0])
		return err
	},
}

func init() {
	rootCmd.AddCommand(genspecCmd)
}
