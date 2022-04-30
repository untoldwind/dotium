use std::{error::Error, path::PathBuf};

use clap::Parser;
use console::Style;
use dialoguer::Confirm;

use crate::{config::ConfigurationHolder, repository::Repository};

#[derive(Debug, Parser)]
pub struct InitRepoCommand {
    directory: Option<PathBuf>,
}

impl InitRepoCommand {
    pub fn run(&self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        let recipient =
            match config.configuration {
                Some(config) => config.default_recipient,
                None => return Err(
                    "Dotium not initialized. Use 'dotium init' or create configuration manually"
                        .into(),
                ),
            };
        let directory = self.directory.clone().unwrap_or_else(|| PathBuf::from("."));

        if Repository::open(&directory).is_ok() {
            return Err("Already initialized".into());
        }

        let bold = Style::new().bold();
        println!("Initialize repository");
        println!(
            "  Directory: {}",
            bold.apply_to(directory.to_string_lossy())
        );
        println!("  Reciepient:      {}", bold.apply_to(&recipient.name));

        println!();

        if Confirm::new().with_prompt("Continue").interact()? {
            Repository::init(directory, recipient)?;
        }

        Ok(())
    }
}
