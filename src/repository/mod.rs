use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use crate::model::{DirectoryDescriptor, FileDescriptor, Recipient, RootDescriptor};

mod actions;
pub mod path_translate;

pub struct Repository {
    pub directory: PathBuf,
    root_file: PathBuf,
    root: RootDescriptor,
    dirs: HashMap<PathBuf, DirectoryDescriptor>,
}

impl Repository {
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
        };
        repo.store()?;

        Ok(repo)
    }

    pub fn add_files<I: IntoIterator<Item = FileDescriptor>>(
        &mut self,
        files: I,
    ) -> Result<(), Box<dyn Error>> {
        for mut file in files {
            let source = if !file.source.is_relative() {
                if !file.source.starts_with(&self.directory) {
                    return Err(format!(
                        "{} is not inside repository",
                        file.source.to_string_lossy()
                    )
                    .into());
                } else {
                    file.source.strip_prefix(&self.directory)?
                }
            } else {
                &file.source
            };

            actions::create_from_target(self, &file)?;

            let parent = source.parent().map(|p| p.to_path_buf()).unwrap_or_default();
            file.source = file.source.strip_prefix(&parent)?.to_path_buf();

            if let Some(dir) = self.dirs.get_mut(&parent) {
                dir.files.push(file);
            } else {
                self.dirs
                    .insert(parent, DirectoryDescriptor { files: vec![file] });
            }
        }
        self.root.directories = self.dirs.keys().cloned().collect();
        self.store()
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
