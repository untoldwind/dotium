use std::{error::Error, fmt, fs, marker::PhantomData, path::PathBuf};

use super::Environment;

#[derive(Debug)]
pub enum Changes {
    NewFile,
    Diff(Vec<u8>),
    ChangePermission(String),
    None,
}
#[derive(Debug)]
pub struct Outcome<E> {
    pub target: PathBuf,
    pub content: Vec<u8>,
    pub permission: String,
    pub phantom: PhantomData<E>,
}

impl<E> Outcome<E>
where
    E: Environment,
{
    pub fn changes(&self) -> Result<Changes, Box<dyn Error>> {
        if self.target.exists() {
            let current_content = fs::read(&self.target)?;

            if current_content == self.content {
                let current_permission =
                    E::permission_to_string(fs::metadata(&self.target)?.permissions());

                if self.permission != current_permission {
                    Ok(Changes::ChangePermission(current_permission))
                } else {
                    Ok(Changes::None)
                }
            } else {
                Ok(Changes::Diff(current_content))
            }
        } else {
            Ok(Changes::NewFile)
        }
    }

    pub fn apply(&self) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = self.target.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.target, &self.content)?;

        if let Some(permissions) = E::permission_from_string(&self.permission) {
            fs::set_permissions(&self.target, permissions)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OutcomeError {
    pub target: PathBuf,
    pub error: Box<dyn Error + 'static>,
}

impl Error for OutcomeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.error.as_ref())
    }
}

impl fmt::Display for OutcomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
