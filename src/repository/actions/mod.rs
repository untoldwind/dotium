use std::{error::Error, path::PathBuf};

use crate::model::{DirectoryDescriptor, FileAction, FileDescriptor};

use super::{Environment, Repository};

mod as_is;

pub fn create_from_target<E: Environment>(
    repository: &Repository<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<(), Box<dyn Error>> {
    match file.action {
        FileAction::AsIs => as_is::create_from_target(repository, dir_path, file),
    }
}

pub fn get_content<E: Environment>(
    repository: &Repository<E>,
    dir_path: &PathBuf,
    dir: &DirectoryDescriptor,
    file: &FileDescriptor,
) -> Result<String, Box<dyn Error>> {
    match file.action {
        FileAction::AsIs => as_is::get_content(repository, dir_path, dir, file),
    }
}
