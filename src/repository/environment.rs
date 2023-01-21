use std::{error::Error, fs, path::PathBuf};

pub trait Environment {
    fn home_dir() -> Result<PathBuf, Box<dyn Error>>;

    fn config_dir() -> Result<PathBuf, Box<dyn Error>>;

    fn permission_to_string(permissions: fs::Permissions) -> String;

    fn permission_from_string(text: &str) -> Option<fs::Permissions>;
}

pub struct DefaultEnvironment {}

impl Environment for DefaultEnvironment {
    fn home_dir() -> Result<PathBuf, Box<dyn Error>> {
        Ok(dirs::home_dir().ok_or("no home directory")?)
    }

    fn config_dir() -> Result<PathBuf, Box<dyn Error>> {
        Ok(dirs::config_dir()
            .map(|dir| dir.join("dotium"))
            .ok_or("Unable to get config dir")?)
    }

    #[cfg(unix)]
    fn permission_to_string(permissions: fs::Permissions) -> String {
        use std::os::unix::fs::PermissionsExt;

        format!("{:04o}", (permissions.mode() & 0o777))
    }

    #[cfg(not(unix))]
    fn permission_to_string(permissions: fs::Permissions) -> String {
        if permissions.readonly() {
            "0444".to_string()
        } else {
            "0644".to_string()
        }
    }

    #[cfg(unix)]
    fn permission_from_string(text: &str) -> Option<fs::Permissions> {
        use std::{fs::Permissions, os::unix::prelude::PermissionsExt};

        u32::from_str_radix(text, 8).map(Permissions::from_mode).ok()
    }

    #[cfg(not(unix))]
    fn permission_from_string(_: &str) -> Option<fs::Permissions> {
        None
    }
}
