use std::path::PathBuf;

use clap::ValueEnum;
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FileDescriptor {
    pub source: String,
    pub target: PathBuf,
    pub action: FileAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
pub enum FileAction {
    AsIs,
    Crypted,
    J2,
}

impl FileAction {
    pub fn default_permission(&self) -> String {
        match self {
            FileAction::AsIs | FileAction::J2 => "0644".to_string(),
            FileAction::Crypted => "0600".to_string(),
        }
    }
}
