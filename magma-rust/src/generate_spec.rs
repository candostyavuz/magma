use crate::proposal::{ApiData, Spec};
use crate::GenerateSpecArgs;
use crate::{input, proposal::Proposal};
use colored::Colorize;
use eyre::{Result, WrapErr};

pub struct GenerateSpec {
    args: GenerateSpecArgs,
    input: input::Input,
}

impl GenerateSpec {
    pub fn try_new(args: GenerateSpecArgs) -> Result<Self> {
        let input_file_reader =
            std::fs::File::open(&args.input_file).wrap_err("Unable to open input file")?;

        let input = serde_yaml::from_reader(input_file_reader)?;
        println!("Input file parsed successfully");

        Ok(Self { args, input })
    }

    pub fn run(self) -> Result<()> {
        let output_file_path = if let Some(ref output) = self.args.output_file {
            output.clone()
        } else {
            let mut output = std::env::current_dir()?;
            output.push("output.json");

            output
        };

        let proposal = self.create_proposal_struct();
        let proposal_json = serde_json::to_vec_pretty(&proposal)?;

        // write proposal to file
        println!(
            "Writing proposal to file: {}",
            output_file_path.to_string_lossy().green()
        );

        std::fs::write(output_file_path, proposal_json)?;

        Ok(())
    }

    fn create_proposal_struct(self) -> Proposal {
        let full_title = if let Some(title) = self.args.title {
            title
        } else {
            format!("Adding specs")
        };

        let description = if let Some(description) = self.args.description {
            description
        } else {
            format!("Adding new specification support for relaying data on Lava")
        };

        let mut specs: Vec<Spec> = Vec::with_capacity(self.input.0.len());

        for input_item in self.input.0 {
            let chain_name = input_item
                .chain_name
                .clone()
                .unwrap_or_else(|| input_item.chain_index.to_string());

            let apis: Vec<ApiData> = input_item.api_methods.into_iter().map(Into::into).collect();
            let spec = Spec::new(chain_name, input_item.chain_index, apis);

            specs.push(spec)
        }

        Proposal::new(full_title, description, specs)
    }
}
