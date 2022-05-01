use std::{error::Error, path::PathBuf};

use crate::model::{DirectoryDescriptor, FileAction, FileDescriptor};

use super::Repository;

mod as_is;

pub fn create_from_target(
    repository: &Repository,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<(), Box<dyn Error>> {
    match file.action {
        FileAction::AsIs => as_is::create_from_target(repository, dir_path, file),
    }
}

pub fn get_content(
    repository: &Repository,
    dir_path: &PathBuf,
    dir: &DirectoryDescriptor,
    file: &FileDescriptor,
) -> Result<String, Box<dyn Error>> {
    match file.action {
        FileAction::AsIs => as_is::get_content(repository, dir_path, dir, file),
    }
}
