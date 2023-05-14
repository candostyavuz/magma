package magma

import (
	"fmt"

	"github.com/candostyavuz/magma/pkg/magma"

	"github.com/spf13/cobra"
)

var gencosmosCmd = &cobra.Command{
	Use:     "gen-cosmos-spec [cosmos-chain-endpoint] | Flags: [--chain-name] , [--chain-idx], [--imports]",
	Aliases: []string{"gencosmos"},
	Short:   "Generates a valid cosmos spec file from a valid chain endpoint.",
	Long: `Generates a valid cosmos sdk chain spec file from a valid chain endpoint.
	Program uses the endpoint to fetch supported api methods. Then, each fetched method is iterated and subcommands are extracted.
	Cosmos and IBC methods will be added as 'imports' field`,
	Args: cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		fmt.Println("End point is: ", args[0])
		err := magma.GenerateCosmosSpec(args[0])
		return err
	},
}

func init() {
	rootCmd.AddCommand(gencosmosCmd)
}
