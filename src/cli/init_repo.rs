use std::{error::Error, path::PathBuf};

use clap::Parser;
use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{
    config::ConfigurationHolder,
    repository::{DefaultEnvironment, Repository},
};

#[derive(Debug, Parser)]
pub struct InitRepoCommand {
    #[clap(default_value = ".")]
    directory: PathBuf,
}

impl InitRepoCommand {
    pub fn run(self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        let recipient =
            match config.configuration {
                Some(config) => config.default_recipient,
                None => return Err(
                    "Dotium not initialized. Use 'dotium init' or create configuration manually"
                        .into(),
                ),
            };
        if Repository::<DefaultEnvironment>::open(&self.directory).is_ok() {
            return Err("Already initialized".into());
        }

        let bold = Style::new().bold();
        println!("Initialize repository");
        println!(
            "  Directory: {}",
            bold.apply_to(self.directory.to_string_lossy())
        );
        println!("  Reciepient:      {}", bold.apply_to(&recipient.name));

        println!();

        if let Some(true) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Continue")
            .default(true)
            .interact_opt()?
        {
            Repository::<DefaultEnvironment>::init(self.directory, recipient)?;
        }

        Ok(())
    }
}
