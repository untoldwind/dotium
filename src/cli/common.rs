use std::{error::Error, fs, str};

use crate::{
    config::ConfigurationHolder,
    model::{Recipient, SecretKey},
    utils::color_diff::ColorDiff,
};

pub fn require_self(config: &ConfigurationHolder) -> Result<Recipient, Box<dyn Error>> {
    match &config.configuration {
        Some(config) => Ok(config.default_recipient.clone()),
        None => {
            Err("Dotium not initialized. Use 'dotium init' or create configuration manually".into())
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
        Err("Keys file is empty. Use 'dotium init' or create configuration manually".into())
    } else {
        Ok(secret_keys)
    }
}

pub fn show_color_diff(left: &[u8], right: &[u8]) {
    match (str::from_utf8(left), str::from_utf8(right)) {
        (Ok(left), Ok(right)) => {
            println!("{}", ColorDiff::new(left, right))
        }

        _ => println!("Binary content"),
    };
}
