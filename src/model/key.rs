use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    pub name: String,
    pub key: String,
}
