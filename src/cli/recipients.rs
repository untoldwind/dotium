use std::{error::Error, path::PathBuf};

use clap::{Args, Subcommand};
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{
    config::ConfigurationHolder,
    repository::{DefaultEnvironment, Repository},
};

use super::common::require_self;

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
        match &self.subcommand {
            RecipientsSubCommand::List => self.list(config),
            RecipientsSubCommand::Approve => self.approve(config),
            RecipientsSubCommand::AddSelf => self.add_self(config),
        }
    }

    fn list(&self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn approve(&self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn add_self(&self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        let recipient = require_self(&config)?;
        let mut repository = Repository::<DefaultEnvironment>::open(&self.repository)?;

        if repository.recipients().any(|r| r.key == recipient.key) {
            return Err("Repository already has a recipient with that key".into());
        }
        if repository
            .recipient_requests()
            .any(|r| r.key == recipient.key)
        {
            return Err("Repository already has a recipient request with that key".into());
        }

        if let Some(true) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Add self to repository {}",
                repository.directory.to_string_lossy()
            ))
            .default(true)
            .interact_opt()?
        {
            repository.add_recipient_request(recipient);
            repository.store()?;
        }

        Ok(())
    }
}
