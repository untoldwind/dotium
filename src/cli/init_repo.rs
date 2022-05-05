use std::{error::Error, path::PathBuf};

use clap::Args;
use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{
    cli::common::require_self,
    config::ConfigurationHolder,
    repository::{DefaultEnvironment, Repository},
};

#[derive(Debug, Args)]
pub struct InitRepoCommand {
    #[clap(default_value = ".")]
    directory: PathBuf,
}

impl InitRepoCommand {
    pub fn run(self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        let recipient = require_self(&config)?;

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
