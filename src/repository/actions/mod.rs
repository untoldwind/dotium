use std::error::Error;

use crate::model::{FileAction, FileDescriptor};

use super::Repository;

mod as_is;

pub fn create_from_target(
    repository: &Repository,
    file: &FileDescriptor,
) -> Result<(), Box<dyn Error>> {
    match file.action {
        FileAction::AsIs => as_is::create_from_target(repository, &file.source, &file.target),
    }
}
