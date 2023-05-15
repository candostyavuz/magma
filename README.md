### MAGMA: A CLI Tool For Automating Spec Creation
Introducing Magma - the simple and robust command-line tool for generating and evaluating Lava specifications.
Onboard new chains with ease, for both EVM and Cosmos SDK chains.

## Installation

1. Copy the repo
2. Export GO path into working directory:
```bash
export GOPATH=$(go env GOPATH) 
export GOBIN=$GOPATH/bin 
export PATH="/opt/homebrew/opt/libpq/bin:$GOPATH:$GOBIN:$PATH" 
export DOCKER_DEFAULT_PLATFORM=linux/amd64
```

3. Install dependencies:
```bash
go install
```

## Usage
# For EVM Chains:
Run CLI command with sample `input.yaml` file:
```bash
magma gen-evm-spec input.yaml --chain-name <CHAIN_NAME> --chain-idx <CHAIN_IDX>
```

# For Cosmos Chains:
Run CLI command with a valid Chain node endpoint. The program will automatically:
    - Fetch supported API methods
    - Handle spec inheritence (Base Chain imports)
    - Detect & discard ignored methods

```bash
magma gen-cosmos-spec grpc.osmosis.zone:9090 --chain-name Osmosis --chain-idx COS3
```
-----

Generated spec will be written into `output.json` file (default)



