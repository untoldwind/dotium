use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub fn relative_target_file<P: AsRef<Path>>(source: P) -> Result<PathBuf, Box<dyn Error>> {
    let home = dirs::home_dir().ok_or_else::<Box<dyn Error>, _>(|| "no home directory".into())?;

    Ok(source.as_ref().strip_prefix(home)?.to_path_buf())
}

pub fn source_file_from_target<P: AsRef<Path>>(target: P) -> PathBuf {
    let mut source = PathBuf::new();

    if target.as_ref().is_absolute() {
        source.push("root");
    }
    for part in target.as_ref() {
        if part.to_string_lossy().starts_with('.') {
            source.push(&part.to_string_lossy()[1..]);
        } else {
            source.push(part)
        }
    }
    source
}
