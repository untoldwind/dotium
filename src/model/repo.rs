use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Recipient;

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDescriptor {
    #[serde(default)]
    pub recipients: Vec<Recipient>,
    #[serde(default)]
    pub recipient_requests: Vec<Recipient>,
    #[serde(default)]
    pub directories: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryDescriptor {
    #[serde(default)]
    pub files: Vec<FileDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDescriptor {
    pub source: PathBuf,
    pub target: PathBuf,
    pub action: FileAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FileAction {
    AsIs,
}