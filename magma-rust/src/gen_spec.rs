use std::path::PathBuf;

use crate::input;
use eyre::Result;

pub struct GenSpec {
    input: input::InputTemplate,
}

impl GenSpec {
    pub fn try_new(input_file: PathBuf) -> Result<Self> {
        let input_file_reader = std::fs::File::open(&input_file)?;
        let input = serde_yaml::from_reader(input_file_reader)?;

        Ok(Self { input })
    }

    pub fn run(self) -> Result<()> {
        println!("{:#?}", self.input);
        Ok(())
    }
}

// let input = serde_yaml::from_str::<input::InputTemplate>(
//     &std::fs::read_to_string(input_file).unwrap(),
// );
//
// println!("{:#?}", input.unwrap());
