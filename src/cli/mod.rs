use std::error::Error;

use clap::Subcommand;

use crate::config::ConfigurationHolder;

mod apply;
mod common;
mod completions;
mod gen_key;
mod init;
mod init_repo;
mod recipients;
mod track;

#[derive(Debug, Subcommand)]
pub enum MainCommand {
    #[clap(about = "Apply repository contents to current config")]
    Apply(apply::ApplyCommand),
    #[clap(about = "Generate shell completions")]
    Completions(completions::CompletionCommand),
    #[clap(about = "Generate new age-compatible public/private key pair")]
    GenKey(gen_key::GenKeyCommand),
    #[clap(about = "Initialize dotium configuration on new machine")]
    Init(init::InitCommand),
    #[clap(about = "Initialize a new repository")]
    InitRepo(init_repo::InitRepoCommand),
    #[clap(about = "Manage recipients of a repository")]
    Recipients(recipients::RecipientsCommand),
    #[clap(about = "Track dot-file (i.e. add it to repository")]
    Track(track::TrackCommand),
}

impl MainCommand {
    pub fn run(self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        match self {
            MainCommand::Apply(cmd) => cmd.run(config),
            MainCommand::GenKey(cmd) => cmd.run(),
            MainCommand::Completions(cmd) => cmd.run(),
            MainCommand::Init(cmd) => cmd.run(config),
            MainCommand::InitRepo(cmd) => cmd.run(config),
            MainCommand::Recipients(cmd) => cmd.run(config),
            MainCommand::Track(cmd) => cmd.run(),
        }
    }
}
