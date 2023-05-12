pub mod constants;
pub mod generate_spec;
pub mod input;
pub mod proposal;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
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
        author,
        version,
        name = "gen-spec",
        visible_aliases = ["gen", "g"], 
        about = "Generates a valid spec file from a list of supported api calls. Currently, the only supported input format for the spec file is yaml file"
    )]
    GenerateSpec(GenerateSpecArgs),
}

#[derive(Parser)]
pub struct GenerateSpecArgs {
    pub input_file: PathBuf,

    #[arg(long, help = "The chain name", required = false)]
    pub chain_name: Option<String>,

    #[arg(short, long, help = "Imports", required = false)]
    pub imports: Option<Vec<String>>,

    #[arg(short, long, help = "The output file", required = false)]
    pub output_file: Option<PathBuf>,
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
            let gen_spec = generate_spec::GenerateSpec::try_new(gen_spec)?;
            gen_spec.run()?;
        }
    };

    Ok(())
}
