use std::{error::Error, path::PathBuf, fs};

use age::x25519::Identity;
use clap::Parser;
use console::Style;
use dialoguer::Confirm;
use gethostname::gethostname;

use crate::{config::{ConfigurationHolder, Configuration}, model::Recipient};

use super::gen_key::write_identity;


#[derive(Debug, Parser)]
pub struct InitOptions {
    #[clap(short, long, help="Name of the host")]
    pub name: Option<String>,
}

pub fn run(opts: &InitOptions, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
    if config.configuration.is_some() {
        return Err("Already initialized".into())
    }

    let name = match &opts.name {
        Some(name) => name.to_string(),
        None => gethostname().to_string_lossy().to_string(),
    };

    let bold = Style::new().bold();

    println!("Will create new configuration");
    println!("  Hostname:     {}", bold.apply_to(&name));
    println!("  Config file : {}", bold.apply_to(config.config_file.to_string_lossy()));
    println!("  Keys file   : {}", bold.apply_to(config.config_file.to_string_lossy()));
    println!();

    if (Confirm::new().with_prompt("Continue").interact()?) {
        do_init(name, config.config_file, config.keys_file)?;
    }

    Ok(())
}

fn do_init(hostname: String, config_file: PathBuf, keys_file: PathBuf) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = config_file.parent() {
        fs::create_dir_all(parent)?;
    }
    if let Some(parent) = keys_file.parent() {
        fs::create_dir_all(parent)?;
    }

    let sk = Identity::generate();
    let default_recipient = Recipient {
        name: hostname,
        key: sk.to_public().to_string(),
    };
    let configuration = Configuration {
        default_recipient,
    };
    let config_file_content = toml::to_string_pretty(&configuration)?;
    fs::write(config_file, config_file_content)?;
    
    let file = fs::OpenOptions::new().write(true).append(true).create(true).open(keys_file)?;
    write_identity(&file, sk)?;

    Ok(())
}