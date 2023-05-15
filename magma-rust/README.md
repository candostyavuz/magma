# Install

1. Download the Rust compiler: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    - https://www.rust-lang.org/learn/get-started

2. Build with `./build`

3. Run with `./magma gen-spec input.yaml`


# Usage

```bash
❯ ./magma help
Magma a CLI tool for creating specs for lava

Usage: magma [OPTIONS] <COMMAND>

Commands:
  genspec    Generates a valid proposal file from a list of supported api calls. Currently, the only supported input format for the spec file is yaml file [aliases: gen, g, gen-spec]
  validate   Generates a valid spec file from a list of supported api calls. Currently, the only supported input format for the spec file is yaml file [aliases: validate-proposal]
  read-spec  Reads and returns information about a proposal file [aliases: read-proposal]
  help       Print this message or the help of the given subcommand(s)

Options:
  -l, --log-level <LOG_LEVEL>  Sets the log level
  -h, --help                   Print help
  -V, --version                Print version

```
