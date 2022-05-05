use std::{error::Error, path::PathBuf};

use clap::{Args, Subcommand};

use crate::config::ConfigurationHolder;

#[derive(Debug, Subcommand)]
pub enum RecipientsSubCommand {
    #[clap(about = "List recipients of repository", alias = "ls")]
    List,
    #[clap(about = "Approve pending recipient requests")]
    Approve,
    #[clap(about = "Add a recipient request for self")]
    AddSelf,
}

#[derive(Debug, Args)]
pub struct RecipientsCommand {
    #[clap(short, long, default_value = ".", help = "Repository to use")]
    repository: PathBuf,

    #[clap(subcommand)]
    subcommand: RecipientsSubCommand,
}

impl RecipientsCommand {
    pub fn run(&self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
