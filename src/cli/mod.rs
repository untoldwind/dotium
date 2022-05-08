use std::{error::Error, path::PathBuf};

use clap::Subcommand;

use crate::config::ConfigurationHolder;

mod apply;
mod common;
mod completions;
mod edit;
mod gen_key;
mod init;
mod init_repo;
mod recipients;
mod track;
mod variables;

#[derive(Debug, Subcommand)]
pub enum MainCommand {
    #[clap(about = "Apply repository contents to current config")]
    Apply(apply::ApplyCommand),
    #[clap(about = "Generate shell completions")]
    Completions(completions::CompletionCommand),
    #[clap(about = "Edit a file in the repository")]
    Edit(edit::EditCommand),
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
    #[clap(about = "Manage variables")]
    Variables(variables::VariablesCommand),
}

impl MainCommand {
    pub fn run(
        self,
        config: ConfigurationHolder,
        repository_path: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            MainCommand::Apply(cmd) => cmd.run(config, repository_path),
            MainCommand::GenKey(cmd) => cmd.run(),
            MainCommand::Completions(cmd) => cmd.run(),
            MainCommand::Edit(cmd) => cmd.run(config, repository_path),
            MainCommand::Init(cmd) => cmd.run(config),
            MainCommand::InitRepo(cmd) => cmd.run(config, repository_path),
            MainCommand::Recipients(cmd) => cmd.run(config, repository_path),
            MainCommand::Track(cmd) => cmd.run(repository_path),
            MainCommand::Variables(cmd) => cmd.run(config),
        }
    }
}
