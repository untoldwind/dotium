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
pub struct InitRepoCommand {}

impl InitRepoCommand {
    pub fn run(
        self,
        config: ConfigurationHolder,
        repository_path: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let recipient = require_self(&config)?;

        if Repository::<DefaultEnvironment>::open(&repository_path).is_ok() {
            return Err("Already initialized".into());
        }

        let bold = Style::new().bold();
        println!("Initialize repository");
        println!(
            "  Directory: {}",
            bold.apply_to(repository_path.to_string_lossy())
        );
        println!("  Reciepient:      {}", bold.apply_to(&recipient.name));

        println!();

        if let Some(true) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Continue")
            .default(true)
            .interact_opt()?
        {
            Repository::<DefaultEnvironment>::init(repository_path, recipient)?;
        }

        Ok(())
    }
}
