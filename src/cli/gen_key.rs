use age::secrecy::ExposeSecret;
use clap::Parser;

use std::{io::Write, error::Error};

#[derive(Debug, Parser)]
pub struct GenKeyOptions {

}

pub fn run(opts: &GenKeyOptions) -> Result<(), Box<dyn Error>> {
    let mut output = std::io::stdout();

    let sk = age::x25519::Identity::generate();
    let pk = sk.to_public();

    writeln!(&mut output, "# created: {}", chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true))?;
    writeln!(&mut output, "# public key: {}", pk)?;
    writeln!(&mut output, "{}", sk.to_string().expose_secret())?;

    Ok(())
}