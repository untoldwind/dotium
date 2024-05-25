use std::{error::Error, fs, path::PathBuf};

use clap::Args;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::{
    cli::common::show_color_diff,
    config::ConfigurationHolder,
    model::FileAction,
    repository::{DefaultEnvironment, Environment, FileRef, Repository},
};

use super::common::require_secret_keys;

#[derive(Debug, Args)]
pub struct UpdateCommand {
    #[clap(help = "File or directory to add to repository")]
    file_or_directory: PathBuf,
}

impl UpdateCommand {
    pub fn run(
        &self,
        config: ConfigurationHolder,
        repository_path: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let repository = Repository::<DefaultEnvironment>::open(repository_path)?;
        let secret_keys = require_secret_keys(&config)?;

        for file in repository.files() {
            if self.file_or_directory == file.absolute_target()? {
                if file.file.action == FileAction::J2 {
                    return Err(format!(
                        "Cannot update j2 content; {}",
                        &self.file_or_directory.to_string_lossy()
                    )
                    .into());
                }
                let repository_content = file.get_content(&secret_keys)?;
                let current_content = fs::read(&self.file_or_directory)?;

                if repository_content == current_content {
                    println!("No diff {}", &self.file_or_directory.to_string_lossy());
                    return Ok(());
                }

                return update_diff(&file, &repository_content, &current_content);
            }
        }

        Err(format!(
            "No repository file found for {}",
            &self.file_or_directory.to_string_lossy()
        )
        .into())
    }
}

fn update_diff<E: Environment>(
    file_ref: &FileRef<E>,
    repository_content: &[u8],
    current_content: &[u8],
) -> Result<(), Box<dyn Error>> {
    loop {
        match FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&["Yes", "Show details", "No"])
            .with_prompt(format!(
                "Update file {}",
                file_ref.file.target.to_string_lossy()
            ))
            .default(0)
            .interact_opt()?
        {
            Some(0) => return file_ref.set_content(current_content),
            Some(1) => {
                println!();
                println!("{}", file_ref.file.target.to_string_lossy());
                println!("-------------------------------------------------------------------------------");
                show_color_diff(repository_content, current_content);
                println!("-------------------------------------------------------------------------------");
            }
            Some(2) => return Ok(()),
            None => return Err("Aborted by user".into()),
            _ => (),
        }
    }
}
