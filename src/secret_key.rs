use std::{error::Error, io::Write};

use age::{secrecy::ExposeSecret, x25519::Identity};

use crate::model::Recipient;

pub struct SecretKey(Identity);

impl SecretKey {
    pub fn generate() -> Self {
        SecretKey(Identity::generate())
    }

    pub fn as_recipient(&self, name: &str) -> Recipient {
        Recipient {
            name: name.to_string(),
            key: self.0.to_public().to_string(),
        }
    }

    pub fn write_to<W: Write>(&self, mut out: W) -> Result<(), Box<dyn Error>> {
        let pk = self.0.to_public();

        writeln!(
            out,
            "# created: {}",
            chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        )?;
        writeln!(out, "# public key: {}", pk)?;
        writeln!(out, "{}", self.0.to_string().expose_secret())?;

        Ok(())
    }
}
