use std::{error::Error, fs, path::PathBuf};

use clap::Parser;
use console::Style;
use dialoguer::Confirm;

use crate::{config::ConfigurationHolder, model::{Recipient, RepoDescriptor}};

#[derive(Debug, Parser)]
pub struct InitRepoOptions {
    directory: Option<PathBuf>,
}

pub fn run(opts: &InitRepoOptions, config: ConfigurationHolder) -> Result<(), Box<dyn Error>> {
    let recipient = match config.configuration {
        Some(config) => config.default_recipient,
        None => return Err("Dotium not initialized. Use 'dotium init' or create configuration manually".into()),
    };
    let repo_file = opts.directory.clone().unwrap_or_else(|| PathBuf::from(".")).join("dotium.toml");

    if repo_file.is_file() {
        return Err("Already initialized".into())
    }
    let name = match repo_file.parent().and_then(|p| p.file_name()) {
        Some(parent) => parent.to_string_lossy().to_string(),
        None => return Err("Unable to access directory".into()),
    };

    let bold = Style::new().bold();
    println!("Initialize repository");
    println!("  Repository file: {}", bold.apply_to(repo_file.to_string_lossy()));
    println!("  Reciepient:      {}", bold.apply_to(&recipient.name));

    println!();

    if Confirm::new().with_prompt("Continue").interact()? {
        do_init_repo(name, recipient, repo_file)?;
    }

    Ok(())
}

fn do_init_repo(name: String, recipient: Recipient, repo_file_name: PathBuf) -> Result<(), Box<dyn Error>> {
    let descriptor = RepoDescriptor {
        name,
        recipients: vec![recipient],
        recipient_requests: vec![],
    };

    let mut repo_file = fs::OpenOptions::new().write(true).create_new(true).open(repo_file_name)?;
    serde_json::to_writer_pretty(&mut repo_file, &descriptor)?;

    Ok(())
}