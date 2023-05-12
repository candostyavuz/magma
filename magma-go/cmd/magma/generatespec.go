package magma

import (
	"fmt"

	"github.com/candostyavuz/magma/pkg/magma"
	"github.com/spf13/cobra"
)

var genspecCmd = &cobra.Command{
	Use:     "genspec [supported-apis-file] | Flags: [--chain-name] , [--chain-idx]",
	Aliases: []string{"gen"},
	Short:   "Generates a valid spec file from a list of supported api calls",
	Long: `Generates a valid spec file from a list of supported api calls.
	Currently, the only supported input format for the spec file is txt file.`,
	Args: cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		fmt.Println("File is: ", args[0])
		chainName, err := cmd.Flags().GetString("chain-name")
		if err != nil {
			return err
		}
		chainIdx, err := cmd.Flags().GetString("chain-idx")
		if err != nil {
			return err
		}
		err = magma.GenerateSpec(args[0], chainName, chainIdx)
		return err
	},
}

func init() {
	genspecCmd.Flags().String("chain-name", "", "Chain Name")
	genspecCmd.Flags().String("chain-idx", "", "Chain Index")
	rootCmd.AddCommand(genspecCmd)
}