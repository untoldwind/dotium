use std::{error::Error, path::PathBuf};

use clap::Parser;
use console::Style;
use dialoguer::Confirm;
use std::fs;

use crate::{model::FileAction, repository::Repository};

#[derive(Debug, Parser)]
pub struct TrackCommand {
    file_or_directory: PathBuf,
    #[clap(short, long, default_value = ".")]
    repository: PathBuf,
    #[clap(short, long, arg_enum, default_value = "as-is")]
    action: FileAction,
}

impl TrackCommand {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut repository = Repository::open(&self.repository)?;

        let mut targets = Vec::new();

        if self.file_or_directory.is_file() {
            targets.push(self.file_or_directory.clone());
        } else if self.file_or_directory.is_dir() {
            for file in self.file_or_directory.read_dir()? {
                targets.push(file?.path().to_path_buf());
            }
        } else {
            return Err(format!(
                "{} does not exists",
                self.file_or_directory.to_string_lossy()
            )
            .into());
        }

        let added = repository.add_files(self.action, targets)?;

        let bold = Style::new().bold();
        println!(
            "Add files to repository {}",
            bold.apply_to(&repository.directory.to_string_lossy())
        );
        for file_ref in &added {
            println!(
                "  {} -> {} ({:?})",
                bold.apply_to(file_ref.file.target.to_string_lossy()),
                bold.apply_to(
                    file_ref
                        .dir_path
                        .join(&file_ref.file.source)
                        .to_string_lossy()
                ),
                file_ref.file.action
            );
        }

        println!();

        if Confirm::new().with_prompt("Continue").interact()? {
            repository.store()?;
        } else {
            for file_ref in added {
                fs::remove_file(file_ref.absolute_source()).ok();
            }
        }

        Ok(())
    }
}
