use std::{error::Error, path::PathBuf, str};

use clap::Args;
use dialoguer::{theme::ColorfulTheme, Editor, FuzzySelect};

use crate::{
    config::ConfigurationHolder,
    repository::{DefaultEnvironment, FileRef, Repository},
};

use super::common::require_secret_keys;

#[derive(Debug, Args)]
pub struct EditCommand {
    #[clap(help = "Repository entry to edit")]
    entry: Option<PathBuf>,
}

impl EditCommand {
    pub fn run(
        &self,
        config: ConfigurationHolder,
        repository_path: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let repository = Repository::<DefaultEnvironment>::open(&repository_path)?;
        let secret_keys = require_secret_keys(&config)?;

        let mut files = repository.files().collect::<Vec<FileRef<_>>>();

        if let Some(entry) = &self.entry {
            if let Some(file) = files
                .iter()
                .find(|f| entry == &f.dir_path.join(&f.file.source))
            {
                let content = file.get_content(&secret_keys)?;
                let content = str::from_utf8(&content)?;

                if let Some(new_content) = Editor::new().trim_newlines(false).edit(content)? {
                    file.set_content(new_content.as_bytes())?;
                }

                return Ok(());
            }
        }

        files.sort();

        if let Some(index) = FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&files)
            .with_prompt("Select file to edit")
            .interact_opt()?
        {
            let file = &files[index];
            let content = file.get_content(&secret_keys)?;
            let content = str::from_utf8(&content)?;

            if let Some(new_content) = Editor::new().trim_newlines(false).edit(content)? {
                file.set_content(new_content.as_bytes())?;
            }
        }

        Ok(())
    }
}
