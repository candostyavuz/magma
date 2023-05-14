package magma

import (
	"fmt"

	"github.com/candostyavuz/magma/pkg/magma"

	"github.com/spf13/cobra"
)

var gencosmosCmd = &cobra.Command{
	Use:     "gen-cosmos-spec [cosmos-chain-endpoint] | Required Flags: [--chain-name] , [--chain-idx] ",
	Aliases: []string{"gencosmos"},
	Short:   "Generates a valid cosmos spec file from a valid chain endpoint.",
	Long: `Generates a valid cosmos sdk chain spec file from a valid chain endpoint.
	Program uses the endpoint to fetch supported api methods. Then, each fetched method is iterated and subcommands are extracted.
	Cosmos and IBC methods will be added as 'imports' field`,
	Args: cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		fmt.Println("End point is: ", args[0])
		chainName, err := cmd.Flags().GetString("chain-name")
		if err != nil {
			return err
		}
		chainIdx, err := cmd.Flags().GetString("chain-idx")
		if err != nil {
			return err
		}

		err = magma.GenerateCosmosSpec(args[0], chainName, chainIdx)
		return err
	},
}

func init() {
	gencosmosCmd.Flags().String("chain-name", "", "Chain Name")
	gencosmosCmd.Flags().String("chain-idx", "", "Chain Index")
	gencosmosCmd.MarkFlagRequired("chain-name")
	gencosmosCmd.MarkFlagRequired("chain-idx")
	rootCmd.AddCommand(gencosmosCmd)
}
