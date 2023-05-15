use crate::{
    proposal::{Proposal, ProposalFile},
    ReadSpecArgs,
};
use colored::Colorize;
use eyre::{Context, Result};

#[derive(Debug)]
pub struct ReadSpec {
    pub args: ReadSpecArgs,
    pub proposal: Proposal,
}

impl ReadSpec {
    pub fn try_new(args: ReadSpecArgs) -> Result<Self> {
        let input_file_reader =
            std::fs::File::open(&args.input_file).wrap_err("Unable to open input file")?;

        let input: ProposalFile = serde_json::from_reader(input_file_reader)?;

        Ok(Self {
            args,
            proposal: input.proposal,
        })
    }

    pub fn run(self) -> Result<()> {
        println!("✅ {}\n", "Proposal is valid".green());

        println!("Title: {}", self.proposal.title);
        println!("Description: {}", self.proposal.description);
        println!("Number of Specs: {}", self.proposal.specs.len());

        for spec in self.proposal.specs {
            println!("\nSpec: {}", spec.name);
            println!(" Index: {}", spec.index);

            if !spec.imports.is_empty() {
                println!(" Imports: {:?}", spec.imports);
            }

            if !spec.apis.is_empty() {
                println!(" Number of APIs: {}", spec.apis.len());
            }

            if self.args.print_all {
                println!(" APIs: ");
                for api in spec.apis {
                    println!(" - {}", api.name);
                }
            }
        }

        Ok(())
    }
}
