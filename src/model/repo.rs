use std::path::PathBuf;

use clap::{ArgEnum, PossibleValue};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDescriptor {
    pub source: String,
    pub target: PathBuf,
    pub action: FileAction,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FileAction {
    AsIs,
    Crypted,
}

impl ArgEnum for FileAction {
    fn value_variants<'a>() -> &'a [Self] {
        &[FileAction::AsIs, FileAction::Crypted]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue<'a>> {
        Some(match self {
            FileAction::AsIs => PossibleValue::new("as-is"),
            FileAction::Crypted => PossibleValue::new("crypted"),
        })
    }
}
