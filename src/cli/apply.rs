use std::error::Error;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct ApplyCommand {}

impl ApplyCommand {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
