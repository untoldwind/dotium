use std::{error::Error, fs, path::PathBuf};

use tera::{Context, Tera};

use crate::{
    model::{FileContext, FileDescriptor},
    repository::{file_ref::RepositoryInfo, Environment},
};

pub fn create_from_target<E: Environment>(
    info: &RepositoryInfo<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<(), Box<dyn Error>> {
    let home = E::home_dir()?;
    let target = home.join(&file.target);
    let source = info
        .directory
        .join(dir_path)
        .join(format!("{}.j2", &file.source));

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
) -> Result<String, Box<dyn Error>> {
    let source = info.directory.join(dir_path).join(&file.source);

    let content = fs::read_to_string(source)?;

    Ok(content)
}

pub fn get_rendered<E: Environment>(
    info: &RepositoryInfo<E>,
    file_context: &FileContext,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<String, Box<dyn Error>> {
    let source = info.directory.join(dir_path).join(&file.source);
    let mut tera = Tera::default();
    let mut context = Context::new();

    context.insert("recipient", &file_context.machine.recipient.name);
    context.insert("machine", &file_context.machine.variables);

    tera.add_template_file(source, Some(&file.source))?;

    match tera.render(&file.source, &context) {
        Ok(content) => Ok(content),
        Err(err) => match err.source() {
            Some(source) => Err(format!("{}", source).into()),
            _ => Err(err.into()),
        },
    }
}

pub fn set_content<E: Environment>(
    info: &RepositoryInfo<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
    content: &str,
) -> Result<(), Box<dyn Error>> {
    let source = info.directory.join(dir_path).join(&file.source);

    fs::write(source, content)?;

    Ok(())
}
