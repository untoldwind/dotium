use std::{
    error::Error,
    fs,
    io::{self, Read, Write},
    path::PathBuf,
};

use age::{
    armor::{ArmoredReader, ArmoredWriter, Format},
    Decryptor, Encryptor, Recipient,
};

use crate::{
    model::{FileDescriptor, SecretKey},
    repository::{file_ref::RepositoryInfo, Environment},
};

pub fn create_from_target<E: Environment>(
    info: &RepositoryInfo<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<(), Box<dyn Error>> {
    let home = E::home_dir()?;
    let target = home.join(&file.target);
    let source = info.directory.join(dir_path).join(&file.source);
    let encryptor = Encryptor::with_recipients(
        info.recipients
            .iter()
            .map(|r| r.to_age())
            .collect::<Result<Vec<Box<dyn Recipient>>, Box<dyn Error>>>()?,
    );

    if let Some(parent) = source.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut intput_file = fs::File::open(target)?;
    let output_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(source)?;
    let mut output =
        encryptor.wrap_output(ArmoredWriter::wrap_output(output_file, Format::AsciiArmor)?)?;

    io::copy(&mut intput_file, &mut output)?;

    output.finish()?.finish()?;

    Ok(())
}

pub fn get_content<E: Environment>(
    info: &RepositoryInfo<E>,
    secret_keys: &[SecretKey],
    dir_path: &PathBuf,
    file: &FileDescriptor,
) -> Result<String, Box<dyn Error>> {
    let source = info.directory.join(dir_path).join(&file.source);

    if let Decryptor::Recipients(decryptor) =
        Decryptor::new(ArmoredReader::new(fs::File::open(source)?))?
    {
        let mut content = String::new();
        decryptor
            .decrypt(secret_keys.iter().map(|s| s.to_age()))?
            .read_to_string(&mut content)?;

        Ok(content)
    } else {
        Err("Invalid encryption format: No recipients".into())
    }
}

pub fn set_content<E: Environment>(
    info: &RepositoryInfo<E>,
    dir_path: &PathBuf,
    file: &FileDescriptor,
    content: &str,
) -> Result<(), Box<dyn Error>> {
    let source = info.directory.join(dir_path).join(&file.source);
    let encryptor = Encryptor::with_recipients(
        info.recipients
            .iter()
            .map(|r| r.to_age())
            .collect::<Result<Vec<Box<dyn Recipient>>, Box<dyn Error>>>()?,
    );
    let output_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(source)?;

    let mut output =
        encryptor.wrap_output(ArmoredWriter::wrap_output(output_file, Format::AsciiArmor)?)?;

    output.write_all(content.as_bytes())?;

    output.finish()?.finish()?;

    Ok(())
}
