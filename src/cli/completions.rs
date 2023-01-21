use std::error::Error;

use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};
use std::io;

#[derive(Debug, Args)]
pub struct CompletionCommand {
    #[clap(value_enum)]
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
