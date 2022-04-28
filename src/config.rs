use std::{error::Error, fs, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::model::Recipient;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub default_recipient: Recipient,
}

#[derive(Debug)]
pub struct ConfigurationHolder {
    pub config_file: PathBuf,
    pub keys_file: PathBuf,
    pub configuration: Option<Configuration>,
}

pub fn config_dir() -> Result<PathBuf, Box<dyn Error>> {
    dirs::config_dir().map(|dir| dir.join("dotium")).ok_or_else(|| "Unable to get config dir".into())
}

pub fn read_config(maybe_config_file: &Option<PathBuf>, maybe_keys_file: &Option<PathBuf>) -> Result<ConfigurationHolder, Box<dyn Error>> {
    let config_file = match maybe_config_file {
        Some(file) => file.clone(),
        None => config_dir()?.join("config.json"),
    };
    let keys_file = match maybe_keys_file {
        Some(file) => file.clone(),
        None => config_dir()?.join("keys.txt"),
    };
    
    let configuration = if config_file.is_file() {
       let mut file = fs::File::open(&config_file)?;

       Some(serde_json::from_reader(&mut file)?)
    } else {
        None
    };

    Ok(ConfigurationHolder {
        config_file,
        keys_file,
        configuration,
    })
}