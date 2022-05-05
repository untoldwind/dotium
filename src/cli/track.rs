use std::{error::Error, path::{PathBuf, Path}};

use clap::Parser;
use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::fs;

use crate::{
    model::FileAction,
    repository::{DefaultEnvironment, Repository},
};

#[derive(Debug, Parser)]
pub struct TrackCommand {
    #[clap(help = "File or directory to add to repository")]
    file_or_directory: PathBuf,
    #[clap(short, long, default_value = ".", help = "Repository to use")]
    repository: PathBuf,
    #[clap(short, long, arg_enum, default_value = "as-is")]
    action: FileAction,
}

impl TrackCommand {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut repository = Repository::<DefaultEnvironment>::open(&self.repository)?;

        let mut targets = Vec::new();

        if self.file_or_directory.is_file() {
            targets.push(self.file_or_directory.clone());
        } else if self.file_or_directory.is_dir() {
            collect_targets(&mut targets, &self.file_or_directory)?;
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

        if let Some(true) = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Continue")
            .default(true)
            .interact_opt()?
        {
            repository.store()?;
        } else {
            for file_ref in added {
                fs::remove_file(file_ref.absolute_source()).ok();
            }
        }

        Ok(())
    }
}

fn collect_targets(targets: &mut Vec<PathBuf>, directory: &Path) -> Result<(), Box<dyn Error>> {
    for entry in directory.read_dir()? {
        let file = entry?.path();

        if file.is_file() {
            targets.push(file);
        } else if file.is_dir() {
            collect_targets(targets, &file)?;
        }
    }
    Ok(())
}
