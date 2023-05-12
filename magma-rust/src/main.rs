pub mod constants;
pub mod gen_spec;
pub mod input;
pub mod spec;

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
    GenerateSpec { input_file: PathBuf },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateSpec { input_file } => {
            let gen_spec = gen_spec::GenSpec::try_new(input_file)?;
            gen_spec.run()?;
        }
    };

    Ok(())
}
