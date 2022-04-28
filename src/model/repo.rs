use serde::{Serialize, Deserialize};

use super::Recipient;


#[derive(Debug, Serialize, Deserialize)]
pub struct RepoDescriptor {
    pub name: String,
    #[serde(default)]
    pub recipients: Vec<Recipient>,
    #[serde(default)]
    pub recipient_requests: Vec<Recipient>,
}