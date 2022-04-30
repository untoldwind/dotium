use std::{
    error::Error,
    path::{Path, PathBuf},
};

use clap::Parser;
use console::Style;
use dialoguer::Confirm;

use crate::{
    model::{FileAction, FileDescriptor},
    repository::{
        path_translate::{relative_target_file, source_file_from_target},
        Repository,
    },
};

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

        let mut files = Vec::new();

        if self.file_or_directory.is_file() {
            files.push(make_file_descriptor(
                &self.file_or_directory,
                self.action,
                &repository.directory,
            )?);
        } else if self.file_or_directory.is_dir() {
            for file in self.file_or_directory.read_dir()? {
                files.push(make_file_descriptor(
                    file?.path(),
                    self.action,
                    &repository.directory,
                )?);
            }
        } else {
            return Err(format!(
                "{} does not exists",
                self.file_or_directory.to_string_lossy()
            )
            .into());
        }

        let bold = Style::new().bold();
        println!(
            "Add files to repository {}",
            bold.apply_to(&repository.directory.to_string_lossy())
        );
        for file in &files {
            println!(
                "  {} -> {} ({:?})",
                bold.apply_to(file.target.to_string_lossy()),
                bold.apply_to(file.source.to_string_lossy()),
                file.action
            );
        }

        println!();

        if Confirm::new().with_prompt("Continue").interact()? {
            repository.add_files(files)?;
        }

        Ok(())
    }
}

fn make_file_descriptor<P: AsRef<Path>>(
    file: P,
    action: FileAction,
    repository_directory: &Path,
) -> Result<FileDescriptor, Box<dyn Error>> {
    let target = relative_target_file(file)?;
    let source = source_file_from_target(&target);

    if repository_directory.join(&source).exists() {
        return Err(format!("{} already exists in repository", target.to_string_lossy()).into());
    }

    Ok(FileDescriptor {
        source,
        target,
        action,
    })
}
