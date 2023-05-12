use crate::input;
use crate::GenerateSpecArgs;
use eyre::Result;
use eyre::WrapErr;

pub struct GenerateSpec {
    args: GenerateSpecArgs,
    input: input::InputTemplate,
}

impl GenerateSpec {
    pub fn try_new(args: GenerateSpecArgs) -> Result<Self> {
        let input_file_reader =
            std::fs::File::open(&args.input_file).wrap_err("Unable to open input file")?;

        let input = serde_yaml::from_reader(input_file_reader)?;

        Ok(Self { args, input })
    }

    pub fn run(self) -> Result<()> {
        let title = self
            .args
            .chain_name
            .clone()
            .or_else(|| self.input.chain_name.clone())
            .unwrap_or_else(|| self.input.chain_index.clone());

        let full_title = format!("Add specs: {title}");

        let description =
            format!("Adding new specification support for relaying {title} data on Lava");

        Ok(())
    }
}
