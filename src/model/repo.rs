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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FileDescriptor {
    pub source: String,
    pub target: PathBuf,
    pub action: FileAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

impl ArgEnum for FileAction {
    fn value_variants<'a>() -> &'a [Self] {
        &[FileAction::AsIs, FileAction::Crypted, FileAction::J2]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue<'a>> {
        Some(match self {
            FileAction::AsIs => PossibleValue::new("as-is"),
            FileAction::Crypted => PossibleValue::new("crypted"),
            FileAction::J2 => PossibleValue::new("j2"),
        })
    }
}
