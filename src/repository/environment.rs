use std::{error::Error, path::PathBuf};

pub trait Environment {
    fn home_dir() -> Result<PathBuf, Box<dyn Error>>;

    fn config_dir() -> Result<PathBuf, Box<dyn Error>>;
}

pub struct DefaultEnvironment {}

impl Environment for DefaultEnvironment {
    fn home_dir() -> Result<PathBuf, Box<dyn Error>> {
        Ok(dirs::home_dir().ok_or_else::<Box<dyn Error>, _>(|| "no home directory".into())?)
    }

    fn config_dir() -> Result<PathBuf, Box<dyn Error>> {
        dirs::config_dir()
            .map(|dir| dir.join("dotium"))
            .ok_or_else(|| "Unable to get config dir".into())
    }
}
