use std::error::Error;

use clap::{Parser, CommandFactory};
use clap_complete::{Shell, generate};
use std::io;

#[derive(Debug, Parser)]
pub struct CompletionOptions {
    #[clap(arg_enum)]
    shell: Shell,
}

pub fn run(opts: &CompletionOptions) -> Result<(), Box<dyn Error>> {
    let mut cmd = crate::DotiumOptions::command();
    let name = cmd.get_name().to_string();

    generate(opts.shell.clone(), &mut cmd, name, &mut io::stdout());

    Ok(())
}