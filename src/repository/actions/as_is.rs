use std::{error::Error, fs, path::PathBuf};

use crate::{
    model::FileDescriptor,
    repository::{Environment, Repository},
};

pub fn create_from_target<E: Environment>(
    repository: &Repository<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<(), Box<dyn Error>> {
    let home = E::home_dir()?;
    let target = home.join(&file.target);
    let source = repository.directory.join(dir_path).join(&file.source);

    if let Some(parent) = source.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(target, source)?;

    Ok(())
}

pub fn get_content<E: Environment>(
    repository: &Repository<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<String, Box<dyn Error>> {
    let source = repository.directory.join(dir_path).join(&file.source);

    let content = fs::read_to_string(source)?;

    Ok(content)
}
