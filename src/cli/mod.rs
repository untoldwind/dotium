use std::error::Error;

use clap::Parser;

mod gen_key;

#[derive(Debug, Parser)]
pub enum Subcommand {
    GenKey(gen_key::GenKeyOptions),
}

impl Subcommand {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Subcommand::GenKey(opts) => gen_key::run(opts)
        }
    }
}