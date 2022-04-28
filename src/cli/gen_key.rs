use age::{secrecy::ExposeSecret, x25519::Identity};
use clap::Parser;

use std::{io::Write, error::Error};

#[derive(Debug, Parser)]
pub struct GenKeyOptions {

}

pub fn run(opts: &GenKeyOptions) -> Result<(), Box<dyn Error>> {
    let mut output = std::io::stdout();

    let sk = Identity::generate();

    write_identity(&mut output, sk)?;

    Ok(())
}

pub fn write_identity<W: Write>(mut out: W, sk: Identity) -> Result<(), Box<dyn Error>> {
    let pk = sk.to_public();

    writeln!(out, "# created: {}", chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true))?;
    writeln!(out, "# public key: {}", pk)?;
    writeln!(out, "{}", sk.to_string().expose_secret())?;

    Ok(())
}