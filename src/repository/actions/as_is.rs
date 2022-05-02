use std::{error::Error, fs, path::PathBuf};

use crate::{
    model::{DirectoryDescriptor, FileDescriptor},
    repository::Repository,
};

pub fn create_from_target(
    repository: &Repository,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<(), Box<dyn Error>> {
    let home = dirs::home_dir().ok_or_else::<Box<dyn Error>, _>(|| "no home directory".into())?;
    let target = home.join(&file.target);
    let source = repository.directory.join(dir_path).join(&file.source);

    if let Some(parent) = source.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(target, source)?;

    Ok(())
}

pub fn get_content(
    repository: &Repository,
    dir_path: &PathBuf,
    dir: &DirectoryDescriptor,
    file: &FileDescriptor,
) -> Result<String, Box<dyn Error>> {
    let source = repository.directory.join(dir_path).join(&file.source);

    let content = fs::read_to_string(source)?;

    Ok(content)
}
