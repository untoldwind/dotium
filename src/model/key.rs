use std::{error::Error, fmt, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

impl fmt::Display for Recipient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.key)
    }
}
