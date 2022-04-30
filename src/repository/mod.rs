use std::{error::Error, fs, path::PathBuf};

use crate::model::{DirectoryDescriptor, Recipient, RootDescriptor};

struct RepositoryDir {
    dir_file: PathBuf,
    dir: DirectoryDescriptor,
}

pub struct Repository {
    directory: PathBuf,
    root_file: PathBuf,
    root: RootDescriptor,
    dirs: Vec<RepositoryDir>,
}

impl Repository {
    pub fn open<P: Into<PathBuf>>(directory: P) -> Result<Self, Box<dyn Error>> {
        let directory = directory.into();
        let root_file = directory.join("dotium.json");

        let root: RootDescriptor = serde_json::from_reader(fs::File::open(&root_file)?)?;

        let mut dirs = Vec::with_capacity(root.directories.len());

        for sub_directory in &root.directories {
            let dir_file = directory.join(sub_directory).join("dotium_dir.json");

            if dir_file.is_file() {
                let dir = serde_json::from_reader(fs::File::open(&dir_file)?)?;
                dirs.push(RepositoryDir { dir_file, dir });
            }
        }

        Ok(Repository {
            directory,
            root_file,
            root,
            dirs,
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
            dirs: vec![],
        };
        repo.store()?;

        Ok(repo)
    }

    pub fn store(&self) -> Result<(), Box<dyn Error>> {
        let root_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.root_file)?;

        serde_json::to_writer_pretty(root_file, &self.root)?;

        Ok(())
    }
}
