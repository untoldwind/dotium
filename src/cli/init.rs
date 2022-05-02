use std::error::Error;

use clap::Parser;
use console::Style;
use dialoguer::{Confirm, theme::ColorfulTheme};
use gethostname::gethostname;

use crate::config::ConfigurationHolder;

#[derive(Debug, Parser)]
pub struct InitCommand {
    #[clap(short, long, help = "Name of the host")]
    pub name: Option<String>,
}

impl InitCommand {
    pub fn run(&self, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
        if config.configuration.is_some() {
            return Err("Already initialized".into());
        }

        let name = match &self.name {
            Some(name) => name.to_string(),
            None => gethostname().to_string_lossy().to_string(),
        };

        let bold = Style::new().bold();

        println!("Will create new configuration");
        println!("  Hostname:     {}", bold.apply_to(&name));
        println!(
            "  Config file : {}",
            bold.apply_to(config.config_file.to_string_lossy())
        );
        println!(
            "  Keys file   : {}",
            bold.apply_to(config.keys_file.to_string_lossy())
        );
        println!();

        if let Some(true) = Confirm::with_theme(&ColorfulTheme::default()).with_prompt("Continue").default(true).interact_opt()? {
            config.init(&name)?;
        }

        Ok(())
    }
}
