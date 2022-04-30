use std::error::Error;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use std::io;

#[derive(Debug, Parser)]
pub struct CompletionCommand {
    #[clap(arg_enum)]
    shell: Shell,
}

impl CompletionCommand {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut cmd = crate::DotiumOptions::command();
        let name = cmd.get_name().to_string();

        generate(self.shell, &mut cmd, name, &mut io::stdout());

        Ok(())
    }
}
