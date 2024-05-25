use std::{error::Error, fs, path::PathBuf};

use crate::{
    model::FileDescriptor,
    repository::{file_ref::RepositoryInfo, Environment},
};

pub fn create_from_target<E: Environment>(
    info: &RepositoryInfo<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<(), Box<dyn Error>> {
    let home = E::home_dir()?;
    let target = home.join(&file.target);
    let source = info.directory.join(dir_path).join(&file.source);

    if let Some(parent) = source.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(target, source)?;

    Ok(())
}

pub fn get_content<E: Environment>(
    info: &RepositoryInfo<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let source = info.directory.join(dir_path).join(&file.source);

    let content = fs::read(source)?;

    Ok(content)
}

pub fn set_content<E: Environment>(
    info: &RepositoryInfo<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
    content: &[u8],
) -> Result<(), Box<dyn Error>> {
    let source = info.directory.join(dir_path).join(&file.source);

    fs::write(source, content)?;

    Ok(())
}
