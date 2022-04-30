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
    Completions(completions::CompletionCommand),
    #[clap(about = "Generate new age-compatible public/private key pair")]
    GenKey(gen_key::GenKeyCommand),
    Init(init::InitCommand),
    #[clap(about = "Initialize a new repository")]
    InitRepo(init_repo::InitRepoCommand),
}

impl Subcommand {
    pub fn run(&self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        match self {
            Subcommand::GenKey(cmd) => cmd.run(),
            Subcommand::Completions(cmd) => cmd.run(),
            Subcommand::Init(cmd) => cmd.run(config),
            Subcommand::InitRepo(cmd) => cmd.run(config),
        }
    }
}
