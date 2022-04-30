use clap::Parser;

use std::error::Error;

use crate::secret_key::SecretKey;

#[derive(Debug, Parser)]
pub struct GenKeyCommand {}

impl GenKeyCommand {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut output = std::io::stdout();

        let sk = SecretKey::generate();

        sk.write_to(&mut output)?;

        Ok(())
    }
}
