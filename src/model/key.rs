use std::{error::Error, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    pub name: String,
    pub key: String,
}

impl Recipient {
    pub fn to_age(&self) -> Result<Box<dyn age::Recipient>, Box<dyn Error>> {
        match age::x25519::Recipient::from_str(&self.key) {
            Ok(recipient) => Ok(Box::new(recipient)),
            Err(err) => Err(err.into()),
        }
    }
}
