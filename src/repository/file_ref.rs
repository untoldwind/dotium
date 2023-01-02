use std::{
    error::Error,
    fmt, fs,
    marker::PhantomData,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::model::{FileAction, FileContext, FileDescriptor, MachineContext, Recipient, SecretKey};

use super::{actions, outcome::OutcomeError, Environment, Outcome};

#[derive(Debug)]
pub struct RepositoryInfo<E> {
    pub directory: PathBuf,
    pub recipients: Vec<Recipient>,
    pub phantom: PhantomData<E>,
}

#[derive(Debug)]
pub struct FileRef<E> {
    pub repository: Rc<RepositoryInfo<E>>,
    pub dir_path: PathBuf,
    pub file: FileDescriptor,
}

impl<E> FileRef<E>
where
    E: Environment,
{
    pub fn new<P: AsRef<Path>>(
        repository: Rc<RepositoryInfo<E>>,
        target_file: P,
        action: FileAction,
    ) -> Result<Self, Box<dyn Error>> {
        let target = relative_target_file::<_, E>(target_file)?;
        let (dir_path, source) = source_file_from_target(&target);

        let permissions = fs::metadata(E::home_dir()?.join(&target))?.permissions();

        Ok(FileRef {
            repository,
            dir_path,
            file: FileDescriptor {
                source,
                target,
                action,
                permission: Some(E::permission_to_string(permissions)),
            },
        })
    }

    pub fn absolute_source(&self) -> PathBuf {
        self.repository
            .directory
            .join(&self.dir_path)
            .join(&self.file.source)
    }

    pub fn get_content(&self, secret_keys: &[SecretKey]) -> Result<String, Box<dyn Error>> {
        actions::get_content(&self.repository, secret_keys, &self.dir_path, &self.file)
    }

    pub fn get_rendered(
        &self,
        machine: &MachineContext,
        secret_keys: &[SecretKey],
    ) -> Result<String, Box<dyn Error>> {
        let file_context = FileContext { machine };
        actions::get_rendered(
            &self.repository,
            &file_context,
            secret_keys,
            &self.dir_path,
            &self.file,
        )
    }

    pub fn set_content(&self, content: &str) -> Result<(), Box<dyn Error>> {
        actions::set_content(&self.repository, &self.dir_path, &self.file, content)
    }

    pub fn outcome(
        &self,
        context: &MachineContext,
        secret_keys: &[SecretKey],
    ) -> Result<Outcome<E>, OutcomeError> {
        let home = E::home_dir().map_err(|error| OutcomeError {
            target: self.file.target.clone(),
            error,
        })?;

        let content = self
            .get_rendered(context, secret_keys)
            .map_err(|error| OutcomeError {
                target: home.join(&self.file.target),
                error,
            })?;

        Ok(Outcome {
            target: home.join(&self.file.target),
            content,
            permission: self
                .file
                .permission
                .to_owned()
                .unwrap_or_else(|| self.file.action.default_permission()),
            phantom: PhantomData,
        })
    }
}

impl<E> fmt::Display for FileRef<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.dir_path.join(&self.file.source).to_string_lossy()
        )
    }
}

impl<E> PartialEq for FileRef<E> {
    fn eq(&self, other: &Self) -> bool {
        self.repository.directory == other.repository.directory
            && self.dir_path == other.dir_path
            && self.file == other.file
    }
}

impl<E> Eq for FileRef<E> {}

impl<E> PartialOrd for FileRef<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self
            .repository
            .directory
            .partial_cmp(&other.repository.directory)
        {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.dir_path.partial_cmp(&other.dir_path) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.file.partial_cmp(&other.file)
    }
}

impl<E> Ord for FileRef<E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.repository.directory.cmp(&other.repository.directory) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.dir_path.cmp(&other.dir_path) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.file.cmp(&other.file)
    }
}

fn relative_target_file<P: AsRef<Path>, E: Environment>(
    source: P,
) -> Result<PathBuf, Box<dyn Error>> {
    if source.as_ref().is_relative() {
        Ok(source.as_ref().to_path_buf())
    } else {
        let home = E::home_dir()?;

        Ok(source.as_ref().strip_prefix(home)?.to_path_buf())
    }
}

fn source_file_from_target<P: AsRef<Path>>(target: P) -> (PathBuf, String) {
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

    if dir_path.as_os_str().is_empty() {
        dir_path.push("home")
    }

    (
        dir_path,
        target
            .as_ref()
            .file_name()
            .map(|name| {
                let name = name.to_string_lossy();

                if let Some(stripped) = name.strip_prefix('.') {
                    stripped.to_string()
                } else {
                    name.to_string()
                }
            })
            .unwrap_or_default(),
    )
}
