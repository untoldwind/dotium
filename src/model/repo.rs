use serde::{Deserialize, Serialize};

use super::Recipient;

#[derive(Debug, Serialize, Deserialize)]
pub struct RootDescriptor {
    #[serde(default)]
    pub recipients: Vec<Recipient>,
    #[serde(default)]
    pub recipient_requests: Vec<Recipient>,
    #[serde(default)]
    pub directories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryDescriptor {}

#[derive(Debug, Serialize, Deserialize)]
pub enum Descriptor {
    Root(RootDescriptor),
    Directory(DirectoryDescriptor),
}
