use std::{error::Error, fs};

use crate::{config::ConfigurationHolder, model::Recipient, secret_key::SecretKey};

pub fn require_self(config: &ConfigurationHolder) -> Result<Recipient, Box<dyn Error>> {
    match &config.configuration {
        Some(config) => Ok(config.default_recipient.clone()),
        None => {
            return Err(
                "Dotium not initialized. Use 'dotium init' or create configuration manually".into(),
            )
        }
    }
}

pub fn require_secret_keys(config: &ConfigurationHolder) -> Result<Vec<SecretKey>, Box<dyn Error>> {
    if !config.keys_file.is_file() {
        return Err(
            "Keys file not initialized. Use 'dotium init' or create configuration manually".into(),
        );
    }
    let secret_keys = SecretKey::read_from(fs::File::open(&config.keys_file)?)?;

    if secret_keys.is_empty() {
        return Err(
            "Keys file is empty. Use 'dotium init' or create configuration manually".into(),
        );
    }

    Ok(secret_keys)
}
