use std::error::Error;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct InitRepoOptions {
    directory: String
}

pub fn run(opts: &InitRepoOptions) -> Result<(), Box<dyn Error>>{
    Ok(())
}