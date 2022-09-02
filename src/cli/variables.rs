use std::error::Error;

use clap::{Args, Subcommand};
use prettytable::{row, Table};

use crate::config::ConfigurationHolder;

use super::common::require_self;

#[derive(Debug, Args)]
pub struct SetMachineArgs {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Subcommand)]
pub enum VariablesSubCommand {
    #[clap(about = "Set machine variable")]
    SetMachine(SetMachineArgs),
    #[clap(about = "Show all configured variables")]
    Show,
}

#[derive(Debug, Args)]
pub struct VariablesCommand {
    #[clap(subcommand)]
    subcommand: VariablesSubCommand,
}

impl VariablesCommand {
    pub fn run(&self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        match self.subcommand {
            VariablesSubCommand::SetMachine(ref args) => self.set_machine(config, args),
            VariablesSubCommand::Show => self.show(config),
        }
    }

    fn set_machine(
        &self,
        mut config: ConfigurationHolder,
        args: &SetMachineArgs,
    ) -> Result<(), Box<dyn Error>> {
        let configuration =
            match &mut config.configuration {
                Some(configuration) => configuration,
                None => return Err(
                    "Dotium not initialized. Use 'dotium init' or create configuration manually"
                        .into(),
                ),
            };

        configuration
            .variables
            .insert(args.key.clone(), args.value.clone());

        config.store()
    }

    fn show(&self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        let recipient = require_self(&config)?;
        let mut table = Table::new();

        table.add_row(row!["recipient", recipient.name]);

        for (key, value) in config
            .configuration
            .map(|c| c.variables)
            .unwrap_or_default()
        {
            table.add_row(row![format!("machine.{}", key), value]);
        }

        table.printstd();

        Ok(())
    }
}
