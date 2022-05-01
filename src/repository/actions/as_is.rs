use std::{error::Error, fs, path::PathBuf};

use crate::{model::DirectoryDescriptor, repository::Repository};

pub fn create_from_target(
    repository: &Repository,
    source: &PathBuf,
    target: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let home = dirs::home_dir().ok_or_else::<Box<dyn Error>, _>(|| "no home directory".into())?;
    let target = home.join(target);
    let source = repository.directory.join(source);

    if let Some(parent) = source.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(target, source)?;

    Ok(())
}

pub fn get_content(
    repository: &Repository,
    dir: &DirectoryDescriptor,
    source: &PathBuf,
) -> Result<String, Box<dyn Error>> {
    todo!()
}
