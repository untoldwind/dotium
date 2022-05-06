use std::{error::Error, path::PathBuf};

use clap::Args;
use dialoguer::{theme::ColorfulTheme, Editor, FuzzySelect};

use crate::{
    config::ConfigurationHolder,
    repository::{DefaultEnvironment, FileRef, Repository},
};

use super::common::require_secret_keys;

#[derive(Debug, Args)]
pub struct EditCommand {}

impl EditCommand {
    pub fn run(
        &self,
        config: ConfigurationHolder,
        repository_path: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let repository = Repository::<DefaultEnvironment>::open(&repository_path)?;
        let secret_keys = require_secret_keys(&config)?;

        let mut files = repository.files().collect::<Vec<FileRef>>();

        files.sort();

        if let Some(index) = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&files)
            .with_prompt("Select file to edit")
            .interact_opt()?
        {
            let file = &files[index];
            let content = repository.get_content(&secret_keys, file)?;

            if let Some(new_content) = Editor::new().trim_newlines(false).edit(&content)? {
                repository.set_content(file, &new_content)?;
            }
        }

        Ok(())
    }
}
