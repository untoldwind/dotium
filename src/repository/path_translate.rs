use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
};

use crate::model::{FileAction, FileDescriptor};

use super::{Environment, Repository};

pub fn relative_target_file<P: AsRef<Path>, E: Environment>(
    source: P,
) -> Result<PathBuf, Box<dyn Error>> {
    if source.as_ref().is_relative() {
        Ok(source.as_ref().to_path_buf())
    } else {
        let home = E::home_dir()?;

        Ok(source.as_ref().strip_prefix(home)?.to_path_buf())
    }
}

pub fn source_file_from_target<P: AsRef<Path>>(target: P) -> (PathBuf, String) {
    let mut dir_path = PathBuf::new();

    if target.as_ref().is_absolute() {
        dir_path.push("root");
    }
    if let Some(parent) = target.as_ref().parent() {
        for part in parent {
            if part.to_string_lossy().starts_with('.') {
                dir_path.push(&part.to_string_lossy()[1..]);
            } else {
                dir_path.push(part)
            }
        }
    }

    (
        dir_path,
        target
            .as_ref()
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
    )
}

pub struct FileRef {
    pub repository_directory: PathBuf,
    pub dir_path: PathBuf,
    pub file: FileDescriptor,
}

impl FileRef {
    pub fn new<P: AsRef<Path>, E: Environment>(
        repository: &Repository<E>,
        target_file: P,
        action: FileAction,
    ) -> Result<Self, Box<dyn Error>> {
        let target = relative_target_file::<_, E>(target_file)?;
        let (dir_path, source) = source_file_from_target(&target);

        Ok(FileRef {
            repository_directory: repository.directory.clone(),
            dir_path,
            file: FileDescriptor {
                source,
                target,
                action,
            },
        })
    }

    pub fn absolute_source(&self) -> PathBuf {
        self.repository_directory
            .join(&self.dir_path)
            .join(&self.file.source)
    }
}

impl fmt::Display for FileRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.dir_path.join(&self.file.source).to_string_lossy()
        )
    }
}
