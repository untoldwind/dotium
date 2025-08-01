use std::marker::PhantomData;
use std::rc::Rc;
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use crate::model::{DirectoryDescriptor, FileAction, Recipient, RootDescriptor, SecretKey};

pub use self::environment::*;
pub use self::file_ref::FileRef;
use self::file_ref::RepositoryInfo;
pub use self::outcome::{Changes, Outcome};

mod actions;
mod environment;
mod file_ref;
mod outcome;
#[cfg(test)]
mod tests;

pub struct Repository<E> {
    info: Rc<RepositoryInfo<E>>,
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
            info: Rc::new(RepositoryInfo {
                directory,
                recipients: root.recipients.clone(),
                phantom: PhantomData,
            }),
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
            info: Rc::new(RepositoryInfo {
                directory,
                recipients: root.recipients.clone(),
                phantom: PhantomData,
            }),
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
    ) -> Result<Vec<FileRef<E>>, Box<dyn Error>> {
        let mut added = Vec::new();
        for target in targets {
            let file_ref = FileRef::new(self.info.clone(), target, action)?;

            if file_ref.absolute_source().exists() {
                return Err(format!("{file_ref} already in repository").into());
            }

            actions::create_from_target(&self.info, &file_ref.dir_path, &file_ref.file)?;

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

    pub fn directory(&self) -> PathBuf {
        self.info.directory.to_path_buf()
    }

    pub fn recipients(&self) -> impl Iterator<Item = &Recipient> {
        self.root.recipients.iter()
    }

    pub fn recipient_requests(&self) -> impl Iterator<Item = &Recipient> {
        self.root.recipient_requests.iter()
    }

    pub fn files(&self) -> impl Iterator<Item = FileRef<E>> + '_ {
        self.dirs.iter().flat_map(move |(dir_path, dir)| {
            dir.files.iter().map(move |file| FileRef {
                repository: self.info.clone(),
                dir_path: dir_path.clone(),
                file: file.clone(),
            })
        })
    }

    pub fn add_recipient_request(&mut self, recipient: Recipient) {
        self.root.recipient_requests.push(recipient);
    }

    pub fn approve_recipients(
        &mut self,
        approved: &[Recipient],
        secret_keys: &[SecretKey],
    ) -> Result<(), Box<dyn Error>> {
        let mut remaining_requests = vec![];

        for recipient_request in self.root.recipient_requests.drain(..) {
            if approved.iter().any(|r| r.key == recipient_request.key) {
                self.root.recipients.push(recipient_request)
            } else {
                remaining_requests.push(recipient_request)
            }
        }
        self.root.recipient_requests = remaining_requests;
        self.info = Rc::new(RepositoryInfo {
            directory: self.info.directory.clone(),
            recipients: self.root.recipients.clone(),
            phantom: PhantomData,
        });

        for file in self.files() {
            let content = file.get_content(secret_keys)?;

            file.set_content(&content)?;
        }

        self.store()
    }

    pub fn store(&self) -> Result<(), Box<dyn Error>> {
        let root_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.root_file)?;

        serde_json::to_writer_pretty(root_file, &self.root)?;

        for (dir_path, dir) in &self.dirs {
            let dir_path = self.info.directory.join(dir_path);
            fs::create_dir_all(&dir_path)?;

            let dir_file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(dir_path.join("dotium_dir.json"))?;

            serde_json::to_writer_pretty(dir_file, dir)?;
        }

        Ok(())
    }
}
