pub mod constants;
pub mod generate_spec;
pub mod input;
pub mod proposal;
pub mod read_spec;
pub mod validate;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::Colorize;
use eyre::Result;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Magma a CLI tool for creating specs for lava",
    arg_required_else_help(true)
)]
struct Cli {
    #[clap(short, long, global = true, help = "Sets the log level")]
    log_level: Option<String>,

    // #[clap(short, long, global = true, help = "Sets the log level")]
    #[command(subcommand)]
    command: crate::Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        name = "genspec",
        visible_aliases = ["gen", "g", "gen-spec"], 
        about = "Generates a valid proposal file from a list of supported api calls. Currently, the only supported input format for the spec file is yaml file"
    )]
    GenerateSpec(GenerateSpecArgs),

    #[command(
        visible_aliases = ["validate-proposal"], 
        about = "Generates a valid spec file from a list of supported api calls. Currently, the only supported input format for the spec file is yaml file"
    )]
    Validate(ValidateArgs),

    #[command(
        visible_aliases = ["read-proposal"], 
        about = "Reads and returns information about a proposal file"
    )]
    ReadSpec(ReadSpecArgs),
}

#[derive(Parser)]
pub struct GenerateSpecArgs {
    pub input_file: PathBuf,

    #[arg(short, long, help = "The title for the spec", required = false)]
    pub title: Option<String>,

    #[arg(short, long, help = "The description for the spec", required = false)]
    pub description: Option<String>,

    #[arg(long, help = "The chain name", required = false)]
    pub chain_name: Option<String>,

    #[arg(short, long, help = "The output file", required = false)]
    pub output_file: Option<PathBuf>,
}

#[derive(Parser, Debug, Clone)]
pub struct ValidateArgs {
    pub input_file: PathBuf,
}

#[derive(Parser, Debug, Clone)]
pub struct ReadSpecArgs {
    pub input_file: PathBuf,

    #[arg(short, long, help = "Print all implemented APIs", required = false)]
    pub print_all: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let cli = Cli::parse();

    if let Some(log_level) = cli.log_level {
        let log_level = log_level.parse::<log::LevelFilter>()?;
        log::set_max_level(log_level);
    }

    match cli.command {
        Commands::GenerateSpec(gen_spec) => {
            println!("{}", "Generating spec file".green());

            let gen_spec = generate_spec::GenerateSpec::try_new(gen_spec)?;
            gen_spec.run()?;
        }
        Commands::Validate(validate_args) => {
            println!("{}", "Validating spec file");

            let validate_args = validate::Validate::try_new(validate_args)?;
            validate_args.run()?;
        }
        Commands::ReadSpec(read_spec_args) => {
            println!("{}", "Reading spec file");

            let read_spec_args = read_spec::ReadSpec::try_new(read_spec_args)?;
            read_spec_args.run()?;
        }
    };

    Ok(())
}
