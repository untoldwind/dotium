use std::collections::HashMap;

use super::Recipient;

#[derive(Debug, Clone)]
pub struct MachineContext {
    pub recipient: Recipient,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct FileContext<'a> {
    pub machine: &'a MachineContext,
}
