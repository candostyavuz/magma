use crate::proposal::{ApiData, Spec};
use crate::GenerateSpecArgs;
use crate::{input, proposal::Proposal};
use colored::Colorize;
use eyre::{Result, WrapErr};

pub struct GenerateSpec {
    args: GenerateSpecArgs,
    input: input::InputTemplate,
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
        let chain_name = self
            .args
            .chain_name
            .clone()
            .or_else(|| self.input.chain_name.clone())
            .unwrap_or_else(|| self.input.chain_index.to_string());

        let full_title = if let Some(title) = self.args.title {
            title
        } else {
            format!("Add specs: {chain_name}")
        };

        let description = if let Some(description) = self.args.description {
            description
        } else {
            format!("Adding new specification support for relaying {chain_name} data on Lava")
        };

        let apis: Vec<ApiData> = self.input.api_methods.into_iter().map(Into::into).collect();

        let specs = Spec::new(chain_name, self.input.chain_index, apis);

        Proposal::new(full_title, description, vec![specs])
    }
}
