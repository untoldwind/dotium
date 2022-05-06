use std::{error::Error, path::PathBuf};

use clap::{Args, Subcommand};
use dialoguer::{theme::ColorfulTheme, Confirm};
use prettytable::{cell, row, Table};

use crate::{
    config::ConfigurationHolder,
    repository::{DefaultEnvironment, Repository},
};

use super::common::{require_secret_keys, require_self};

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
    #[clap(subcommand)]
    subcommand: RecipientsSubCommand,
}

impl RecipientsCommand {
    pub fn run(
        &self,
        config: ConfigurationHolder,
        repository_path: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        match &self.subcommand {
            RecipientsSubCommand::List => self.list(repository_path),
            RecipientsSubCommand::Approve => self.approve(config, repository_path),
            RecipientsSubCommand::AddSelf => self.add_self(config, repository_path),
        }
    }

    fn list(&self, repository_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let repository = Repository::<DefaultEnvironment>::open(&repository_path)?;

        let mut table = Table::new();

        table.add_row(row![H2 => "Recipients"]);
        for recipient in repository.recipients() {
            table.add_row(row![recipient.name, recipient.key]);
        }

        table.printstd();

        let mut requests_table = Table::new();
        let mut has_requests = false;
        requests_table.add_row(row![H2 => "Recipients requests"]);

        for recipient in repository.recipient_requests() {
            has_requests = true;
            requests_table.add_row(row![recipient.name, recipient.key]);
        }

        if has_requests {
            requests_table.printstd();
        }

        Ok(())
    }

    fn approve(
        &self,
        config: ConfigurationHolder,
        repository_path: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let mut repository = Repository::<DefaultEnvironment>::open(&repository_path)?;
        let secret_keys = require_secret_keys(&config)?;

        let mut approved = vec![];

        for recipient in repository.recipient_requests() {
            match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Approve {}", recipient))
                .default(false)
                .interact_opt()?
            {
                Some(true) => approved.push(recipient.clone()),
                Some(false) => {}
                None => return Err("Aborted by user".into()),
            }
        }

        repository.approve_recipients(&approved, &secret_keys)
    }

    fn add_self(
        &self,
        config: ConfigurationHolder,
        repository_path: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let recipient = require_self(&config)?;
        let mut repository = Repository::<DefaultEnvironment>::open(&repository_path)?;

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
                repository.directory().to_string_lossy()
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
