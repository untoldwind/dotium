use std::marker::PhantomData;
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use crate::model::{DirectoryDescriptor, FileAction, Recipient, RootDescriptor};
use crate::secret_key::SecretKey;

pub use self::environment::*;
pub use self::outcome::{Changes, Outcome};
pub use self::path_translate::FileRef;

mod actions;
mod environment;
mod outcome;
mod path_translate;
#[cfg(test)]
mod tests;

pub struct Repository<E> {
    pub directory: PathBuf,
    root_file: PathBuf,
    root: RootDescriptor,
    dirs: HashMap<PathBuf, DirectoryDescriptor>,
    phantom: PhantomData<E>,
}

impl<E> Repository<E>
where
    E: Environment,
{
    pub fn open<P: Into<PathBuf>>(directory: P) -> Result<Self, Box<dyn Error>> {
        let directory = directory.into();
        let root_file = directory.join("dotium.json");

        if !root_file.is_file() {
            return Err(format!(
                "Repository in directory {} not initialized",
                directory.to_string_lossy()
            )
            .into());
        }
        let root: RootDescriptor = serde_json::from_reader(fs::File::open(&root_file)?)?;

        let mut dirs = HashMap::with_capacity(root.directories.len());

        for sub_directory in &root.directories {
            let dir_file = directory.join(sub_directory).join("dotium_dir.json");

            if dir_file.is_file() {
                let dir = serde_json::from_reader(fs::File::open(&dir_file)?)?;
                dirs.insert(sub_directory.clone(), dir);
            }
        }

        Ok(Repository {
            directory,
            root_file,
            root,
            dirs,
            phantom: PhantomData,
        })
    }

    pub fn init(directory: PathBuf, recipient: Recipient) -> Result<Self, Box<dyn Error>> {
        let root_file = directory.join("dotium.json");

        let root = RootDescriptor {
            recipients: vec![recipient],
            recipient_requests: vec![],
            directories: vec![],
        };

        let repo = Repository {
            directory,
            root_file,
            root,
            dirs: HashMap::new(),
            phantom: PhantomData,
        };
        repo.store()?;

        Ok(repo)
    }

    pub fn add_files<I: IntoIterator<Item = PathBuf>>(
        &mut self,
        action: FileAction,
        targets: I,
    ) -> Result<Vec<FileRef>, Box<dyn Error>> {
        let mut added = Vec::new();
        for target in targets {
            let file_ref = FileRef::new(self, target, action)?;

            if file_ref.absolute_source().exists() {
                return Err(format!("{} already in repository", file_ref).into());
            }

            actions::create_from_target(self, &file_ref.dir_path, &file_ref.file)?;

            if let Some(dir) = self.dirs.get_mut(&file_ref.dir_path) {
                dir.files.push(file_ref.file.clone());
            } else {
                self.dirs.insert(
                    file_ref.dir_path.clone(),
                    DirectoryDescriptor {
                        files: vec![file_ref.file.clone()],
                    },
                );
            }

            added.push(file_ref)
        }
        self.root.directories = self.dirs.keys().cloned().collect();
        self.root.directories.sort();

        Ok(added)
    }

    pub fn outcomes<'a>(
        &'a self,
        secret_keys: &'a [SecretKey],
    ) -> Result<
        impl Iterator<Item = Result<Outcome, Box<dyn Error + 'static>>> + 'a,
        Box<dyn Error + 'static>,
    > {
        let home = E::home_dir()?;

        Ok(self.dirs.iter().flat_map(move |(dir_path, dir)| {
            let home_cloned = home.clone();
            dir.files.iter().map(move |file| {
                let content = actions::get_content(self, secret_keys, dir_path, dir, file)?;

                Ok(Outcome {
                    target: home_cloned.join(&file.target),
                    content,
                })
            })
        }))
    }

    pub fn recipients(&self) -> impl Iterator<Item = &Recipient> {
        self.root.recipients.iter()
    }

    pub fn store(&self) -> Result<(), Box<dyn Error>> {
        let root_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.root_file)?;

        serde_json::to_writer_pretty(root_file, &self.root)?;

        for (dir_path, dir) in &self.dirs {
            let dir_path = self.directory.join(dir_path);
            fs::create_dir_all(&dir_path)?;

            let dir_file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(dir_path.join("dotium_dir.json"))?;

            serde_json::to_writer_pretty(dir_file, dir)?;
        }

        Ok(())
    }
}
