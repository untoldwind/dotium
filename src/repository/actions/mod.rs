use std::{error::Error, path::PathBuf};

use crate::{
    model::{FileAction, FileDescriptor},
    secret_key::SecretKey,
};

use super::{file_ref::RepositoryInfo, Environment};

mod as_is;
mod crypted;

pub fn create_from_target<E: Environment>(
    info: &RepositoryInfo<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<(), Box<dyn Error>> {
    match file.action {
        FileAction::AsIs => as_is::create_from_target(info, dir_path, file),
        FileAction::Crypted => crypted::create_from_target(info, dir_path, file),
    }
}

pub fn get_content<E: Environment>(
    info: &RepositoryInfo<E>,
    secret_keys: &[SecretKey],
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<String, Box<dyn Error>> {
    match file.action {
        FileAction::AsIs => as_is::get_content(info, dir_path, file),
        FileAction::Crypted => crypted::get_content(info, secret_keys, dir_path, file),
    }
}

pub fn set_content<E: Environment>(
    info: &RepositoryInfo<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
    content: &str,
) -> Result<(), Box<dyn Error>> {
    match file.action {
        FileAction::AsIs => as_is::set_content(info, dir_path, file, content),
        FileAction::Crypted => crypted::set_content(info, dir_path, file, content),
    }
}
