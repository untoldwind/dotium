use std::{
    error::Error,
    fmt,
    fs::{self, Permissions},
    io::Write,
    path::PathBuf,
};

#[derive(Debug)]
pub enum Changes {
    NewFile,
    Diff(String),
    None,
}
#[derive(Debug)]
pub struct Outcome {
    pub target: PathBuf,
    pub content: String,
    pub permissions: Option<Permissions>,
}

impl Outcome {
    pub fn changes(&self) -> Result<Changes, Box<dyn Error>> {
        if self.target.exists() {
            let current_content = fs::read_to_string(&self.target)?;

            if current_content == self.content {
                Ok(Changes::None)
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
        {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&self.target)?;

            file.write_all(self.content.as_bytes())?;
        }

        if let Some(permissions) = &self.permissions {
            fs::set_permissions(&self.target, permissions.to_owned())?;
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
