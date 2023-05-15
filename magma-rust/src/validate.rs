use crate::{proposal::ProposalFile, ValidateArgs};
use colored::Colorize;
use eyre::{Context, Result};

#[derive(Debug, Clone)]
pub struct Validate {
    pub args: ValidateArgs,
}

impl Validate {
    pub fn try_new(args: ValidateArgs) -> Result<Self> {
        let input_file_reader =
            std::fs::File::open(&args.input_file).wrap_err("Unable to open input file")?;

        let _input: ProposalFile = serde_json::from_reader(input_file_reader)?;

        Ok(Self { args })
    }

    pub fn run(self) -> Result<()> {
        println!("✅ {}", "Proposal is valid".green());

        Ok(())
    }
}
