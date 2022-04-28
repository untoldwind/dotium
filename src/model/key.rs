use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    pub name: String,
    pub key: String,
}
