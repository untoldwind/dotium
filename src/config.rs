use std::{error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{model::Recipient, secret_key::SecretKey};

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

impl ConfigurationHolder {
    pub fn config_dir() -> Result<PathBuf, Box<dyn Error>> {
        dirs::config_dir()
            .map(|dir| dir.join("dotium"))
            .ok_or_else(|| "Unable to get config dir".into())
    }

    pub fn read_config(
        maybe_config_file: &Option<PathBuf>,
        maybe_keys_file: &Option<PathBuf>,
    ) -> Result<ConfigurationHolder, Box<dyn Error>> {
        let config_file = match maybe_config_file {
            Some(file) => file.clone(),
            None => Self::config_dir()?.join("config.json"),
        };
        let keys_file = match maybe_keys_file {
            Some(file) => file.clone(),
            None => Self::config_dir()?.join("keys.txt"),
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

    pub fn init(&self, hostname: &str) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = self.config_file.parent() {
            fs::create_dir_all(parent)?;
        }
        if let Some(parent) = self.keys_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let sk = SecretKey::generate();
        let default_recipient = sk.as_recipient(hostname);
        let configuration = Configuration { default_recipient };
        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.config_file)?;
        serde_json::to_writer_pretty(&mut config_file, &configuration)?;

        let key_file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.keys_file)?;
        sk.write_to(&key_file)?;

        Ok(())
    }
}
