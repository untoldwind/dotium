use std::error::Error;

use clap::Parser;

use crate::config::ConfigurationHolder;

mod completions;
mod gen_key;
mod init;
mod init_repo;

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[clap(about = "Generate shell completions")]
    Completions(completions::CompletionOptions),
    #[clap(about = "Generate new age-compatible public/private key pair")]
    GenKey(gen_key::GenKeyOptions),
    Init(init::InitOptions),
    #[clap(about = "Initialize a new repository")]
    InitRepo(init_repo::InitRepoOptions),
}

impl Subcommand {
    pub fn run(&self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        match self {
            Subcommand::GenKey(opts) => gen_key::run(opts),
            Subcommand::Completions(opts) => completions::run(opts),
            Subcommand::Init(opts) => init::run(opts, config),
            Subcommand::InitRepo(opts) => init_repo::run(opts, config),
        }
    }
}