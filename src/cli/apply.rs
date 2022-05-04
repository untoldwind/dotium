use std::{error::Error, path::PathBuf};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::{
    repository::{Changes, DefaultEnvironment, Outcome, Repository},
    utils::color_diff::ColorDiff,
};

#[derive(Debug, Parser)]
pub struct ApplyCommand {
    #[clap(short, long, default_value = ".", help = "Repository to use")]
    repository: PathBuf,
    #[clap(
        short,
        long,
        help = "Only apply changes to specific config file/directory"
    )]
    only: Option<PathBuf>,
}

impl ApplyCommand {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let repository = Repository::<DefaultEnvironment>::open(&self.repository)?;

        for try_outcome in repository.outcomes()? {
            let outcome = try_outcome?;

            if !self
                .only
                .iter()
                .all(|filter| outcome.target.starts_with(filter))
            {
                continue;
            }

            match outcome.changes()? {
                Changes::NewFile => confirm_new_file(&outcome)?,
                Changes::Diff(current) => config_diff(&outcome, &current)?,
                Changes::None => continue,
            };
        }

        Ok(())
    }
}

fn confirm_new_file(outcome: &Outcome) -> Result<(), Box<dyn Error>> {
    loop {
        match FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&["Yes", "Skip", "Show details", "Abort"])
            .with_prompt(format!(
                "Create new file {}",
                outcome.target.to_string_lossy()
            ))
            .default(0)
            .interact_opt()?
        {
            Some(0) => return outcome.apply(),
            Some(1) => return Ok(()),
            Some(2) => {
                println!();
                println!("{}", outcome.target.to_string_lossy());
                println!("-------------------------------------------------------------------------------");
                println!("{}", outcome.content);
                println!();
                println!("-------------------------------------------------------------------------------");
            }
            Some(3) => return Err("Aborted by user".into()),
            None => return Err("Aborted by user".into()),
            _ => (),
        }
    }
}

fn config_diff(outcome: &Outcome, current_content: &str) -> Result<(), Box<dyn Error>> {
    loop {
        match FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&["Yes", "Skip", "Show details", "Abort"])
            .with_prompt(format!("Change file {}", outcome.target.to_string_lossy()))
            .default(0)
            .interact_opt()?
        {
            Some(0) => return outcome.apply(),
            Some(1) => return Ok(()),
            Some(2) => {
                println!();
                println!("{}", outcome.target.to_string_lossy());
                println!("-------------------------------------------------------------------------------");
                println!("{}", ColorDiff::new(current_content, &outcome.content));
                println!("-------------------------------------------------------------------------------");
            }
            Some(3) => return Err("Aborted by user".into()),
            None => return Err("Aborted by user".into()),
            _ => (),
        }
    }
}
