use std::{error::Error, fs, path::PathBuf};

use age::x25519::Identity;
use clap::Parser;
use console::Style;
use dialoguer::Confirm;
use gethostname::gethostname;

use crate::{
    config::{Configuration, ConfigurationHolder},
    model::Recipient,
};

use super::gen_key::write_identity;

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

        if Confirm::new().with_prompt("Continue").interact()? {
            do_init(name, config.config_file, config.keys_file)?;
        }

        Ok(())
    }
}

fn do_init(
    hostname: String,
    config_file_name: PathBuf,
    keys_file_name: PathBuf,
) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = config_file_name.parent() {
        fs::create_dir_all(parent)?;
    }
    if let Some(parent) = keys_file_name.parent() {
        fs::create_dir_all(parent)?;
    }

    let sk = Identity::generate();
    let default_recipient = Recipient {
        name: hostname,
        key: sk.to_public().to_string(),
    };
    let configuration = Configuration { default_recipient };
    let mut config_file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config_file_name)?;
    serde_json::to_writer_pretty(&mut config_file, &configuration)?;

    let key_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(keys_file_name)?;
    write_identity(&key_file, sk)?;

    Ok(())
}
